// Component pipeline tracking (Phase 0 baseline):
// - preserve intrinsic html! behavior via regression snapshots
// - keep compile-fail harness in place for future diagnostic tests
// - route generated symbol naming through the __fr_* convention
mod to_html;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Fields, FnArg, ItemFn, ItemStruct, Pat, ReturnType, Type, TypePath};
use to_html::MacroMode;

fn to_pascal_case(source: &str) -> String {
    let mut out = String::new();
    for part in source.split('_').filter(|part| !part.is_empty()) {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.push(first.to_ascii_uppercase());
            out.extend(chars);
        }
    }
    out
}

fn unwrap_pat_ident(arg: &FnArg, arg_label: &str) -> Result<syn::Ident, TokenStream> {
    let FnArg::Typed(pat_type) = arg else {
        return Err(syn::Error::new_spanned(
            arg,
            format!("{arg_label} argument must be a named binding"),
        )
        .to_compile_error()
        .into());
    };

    match &*pat_type.pat {
        Pat::Ident(pat_ident) => Ok(pat_ident.ident.clone()),
        _ => Err(syn::Error::new_spanned(
            &pat_type.pat,
            format!("{arg_label} argument pattern must be an identifier"),
        )
        .to_compile_error()
        .into()),
    }
}

fn extract_type_path(arg: &FnArg, arg_label: &str) -> Result<TypePath, TokenStream> {
    let FnArg::Typed(pat_type) = arg else {
        return Err(
            syn::Error::new_spanned(arg, format!("{arg_label} argument must be typed"))
                .to_compile_error()
                .into(),
        );
    };

    match &*pat_type.ty {
        Type::Path(type_path) => Ok(type_path.clone()),
        _ => Err(syn::Error::new_spanned(
            &pat_type.ty,
            format!("{arg_label} argument type must be a path type"),
        )
        .to_compile_error()
        .into()),
    }
}

fn extract_type(arg: &FnArg, arg_label: &str) -> Result<Type, TokenStream> {
    let FnArg::Typed(pat_type) = arg else {
        return Err(
            syn::Error::new_spanned(arg, format!("{arg_label} argument must be typed"))
                .to_compile_error()
                .into(),
        );
    };

    Ok((*pat_type.ty).clone())
}

#[proc_macro_attribute]
pub fn with_children(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_tokens = proc_macro2::TokenStream::from(attr);
    if !attr_tokens.is_empty() {
        return syn::Error::new_spanned(attr_tokens, "#[with_children] does not accept arguments")
            .to_compile_error()
            .into();
    }

    let mut item_struct = match syn::parse::<ItemStruct>(item) {
        Ok(item_struct) => item_struct,
        Err(error) => return error.to_compile_error().into(),
    };

    let Fields::Named(fields_named) = &mut item_struct.fields else {
        return syn::Error::new_spanned(
            &item_struct,
            "#[with_children] requires a struct with named fields",
        )
        .to_compile_error()
        .into();
    };

    let already_has_children = fields_named.named.iter().any(|field| {
        field
            .ident
            .as_ref()
            .map(|ident| ident == "children")
            .unwrap_or(false)
    });

    if !already_has_children {
        fields_named.named.push(Field {
            attrs: Vec::new(),
            vis: syn::parse_quote!(pub),
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new("children", proc_macro2::Span::call_site())),
            colon_token: Some(Default::default()),
            ty: syn::parse_quote!(::std::string::String),
        });
    }

    quote!(#item_struct).into()
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_tokens = proc_macro2::TokenStream::from(attr);
    if !attr_tokens.is_empty() {
        return syn::Error::new_spanned(attr_tokens, "#[component] does not accept arguments")
            .to_compile_error()
            .into();
    }

    let function = match syn::parse::<ItemFn>(item.clone()) {
        Ok(function) => function,
        Err(error) => return error.to_compile_error().into(),
    };

    if !function.sig.generics.params.is_empty() {
        return syn::Error::new_spanned(
            &function.sig.generics,
            "#[component] does not currently support generic component functions",
        )
        .to_compile_error()
        .into();
    }

    let original_ident = function.sig.ident.clone();
    let pascal_name = to_pascal_case(&original_ident.to_string());
    if pascal_name.is_empty() {
        return syn::Error::new_spanned(
            &original_ident,
            "#[component] requires a non-empty function name",
        )
        .to_compile_error()
        .into();
    }
    let wrapper_ident = format_ident!("{}", pascal_name, span = original_ident.span());
    let wrapper_props_ident = format_ident!("{}Props", pascal_name, span = original_ident.span());

    let inputs: Vec<FnArg> = function.sig.inputs.iter().cloned().collect();
    let (wrapper_params, call_args, props_ty_path) = match inputs.len() {
        1 => {
            let props_ident = match unwrap_pat_ident(&inputs[0], "props") {
                Ok(ident) => ident,
                Err(err) => return err,
            };
            let props_ty = match extract_type_path(&inputs[0], "props") {
                Ok(ty) => ty,
                Err(err) => return err,
            };

            (
                quote!(#props_ident: #wrapper_props_ident),
                quote!(#props_ident),
                props_ty,
            )
        }
        2 => {
            let ctx_ident = match unwrap_pat_ident(&inputs[0], "context") {
                Ok(ident) => ident,
                Err(err) => return err,
            };
            let ctx_ty = match extract_type(&inputs[0], "context") {
                Ok(ty) => ty,
                Err(err) => return err,
            };

            let props_ident = match unwrap_pat_ident(&inputs[1], "props") {
                Ok(ident) => ident,
                Err(err) => return err,
            };
            let props_ty = match extract_type_path(&inputs[1], "props") {
                Ok(ty) => ty,
                Err(err) => return err,
            };

            (
                quote!(#ctx_ident: #ctx_ty, #props_ident: #wrapper_props_ident),
                quote!(#ctx_ident, #props_ident),
                props_ty,
            )
        }
        _ => {
            return syn::Error::new_spanned(
                &function.sig.inputs,
                "#[component] expects function signatures of fn(props) or fn(ctx, props)",
            )
            .to_compile_error()
            .into();
        }
    };

    let wrapper_return_ty = match &function.sig.output {
        ReturnType::Default => quote!(),
        ReturnType::Type(_, ty) => quote!(-> #ty),
    };

    let call_expr = if function.sig.asyncness.is_some() {
        quote!(#original_ident(#call_args).await)
    } else {
        quote!(#original_ident(#call_args))
    };

    let wrapper_fn = if function.sig.asyncness.is_some() {
        quote! {
            #[allow(non_snake_case)]
            pub async fn #wrapper_ident(#wrapper_params) #wrapper_return_ty {
                #call_expr
            }
        }
    } else {
        quote! {
            #[allow(non_snake_case)]
            pub fn #wrapper_ident(#wrapper_params) #wrapper_return_ty {
                #call_expr
            }
        }
    };

    let should_emit_props_alias = props_ty_path.qself.is_none()
        && props_ty_path.path.leading_colon.is_none()
        && props_ty_path.path.segments.len() == 1
        && props_ty_path
            .path
            .segments
            .first()
            .map(|segment| segment.ident != wrapper_props_ident)
            .unwrap_or(true);

    let props_alias = if should_emit_props_alias {
        quote!(pub type #wrapper_props_ident = #props_ty_path;)
    } else {
        quote!()
    };

    quote! {
        #function
        #props_alias
        #wrapper_fn
    }
    .into()
}

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::Html)
}

#[proc_macro]
pub fn html_async(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::HtmlAsync)
}

#[proc_macro]
pub fn html_in(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::HtmlIn)
}

#[proc_macro]
pub fn html_async_in(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::HtmlAsyncIn)
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
