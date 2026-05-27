// Component pipeline tracking (Phase 0 baseline):
// - preserve intrinsic html! behavior via regression snapshots
// - keep compile-fail harness in place for future diagnostic tests
// - route generated symbol naming through the __fr_* convention
mod to_html;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Field, Fields, FnArg, ItemFn, ItemStruct, Pat, ReturnType, Type, TypePath};
use to_html::MacroMode;

struct HtmlInvocationShape {
    has_writer_prefix: bool,
}

impl Parse for HtmlInvocationShape {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _: syn::Expr = input.parse()?;
        if input.is_empty() {
            return Ok(Self {
                has_writer_prefix: false,
            });
        }

        let _: syn::Token![,] = input.parse()?;
        let _: proc_macro2::TokenStream = input.parse()?;
        Ok(Self {
            has_writer_prefix: true,
        })
    }
}

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
    let original_item_tokens = proc_macro2::TokenStream::from(item.clone());
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
        2 => {
            let out_ident = match unwrap_pat_ident(&inputs[0], "writer") {
                Ok(ident) => ident,
                Err(err) => return err,
            };
            let out_ty = match extract_type(&inputs[0], "writer") {
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
                quote!(#out_ident: #out_ty, #props_ident: #wrapper_props_ident),
                quote!(#out_ident, #props_ident),
                props_ty,
            )
        }
        3 => {
            let out_ident = match unwrap_pat_ident(&inputs[0], "writer") {
                Ok(ident) => ident,
                Err(err) => return err,
            };
            let out_ty = match extract_type(&inputs[0], "writer") {
                Ok(ty) => ty,
                Err(err) => return err,
            };

            let ctx_ident = match unwrap_pat_ident(&inputs[1], "context") {
                Ok(ident) => ident,
                Err(err) => return err,
            };
            let ctx_ty = match extract_type(&inputs[1], "context") {
                Ok(ty) => ty,
                Err(err) => return err,
            };

            let props_ident = match unwrap_pat_ident(&inputs[2], "props") {
                Ok(ident) => ident,
                Err(err) => return err,
            };
            let props_ty = match extract_type_path(&inputs[2], "props") {
                Ok(ty) => ty,
                Err(err) => return err,
            };

            (
                quote!(#out_ident: #out_ty, #ctx_ident: #ctx_ty, #props_ident: #wrapper_props_ident),
                quote!(#out_ident, #ctx_ident, #props_ident),
                props_ty,
            )
        }
        _ => {
            return syn::Error::new_spanned(
                &function.sig.inputs,
                "#[component] expects function signatures of fn(writer, props) or fn(writer, ctx, props)",
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
        //#function
        #original_item_tokens
        #props_alias
        #wrapper_fn
    }
    .into()
}

#[proc_macro_attribute]
pub fn rsx_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_tokens = proc_macro2::TokenStream::from(attr);
    if !attr_tokens.is_empty() {
        return syn::Error::new_spanned(attr_tokens, "#[rsx_component] does not accept arguments")
            .to_compile_error()
            .into();
    }

    let item_tokens = proc_macro2::TokenStream::from(item);
    quote! {
        #[allow(non_snake_case)]
        #item_tokens
    }
    .into()
}

#[proc_macro]
/// Renders RSX-style markup into HTML.
///
/// This macro supports two invocation shapes:
///
/// - Fragment mode: returns `::freshed_rs_runtime::HtmlFragment`
/// - Writer mode: writes directly into a writer and returns `RenderResult`
///
/// Dynamic values are HTML-escaped by default.
///
/// # Fragment Mode
///
/// ```ignore
/// use freshed_rs_macros::html;
///
/// let fragment = html!(<div class="card">{"hello"}</div>);
/// ```
///
/// # Writer Mode
///
/// ```ignore
/// use freshed_rs_macros::html;
///
/// let mut out = String::new();
/// html!(&mut out, <p id={format!("p-{}", 1)}>"ok"</p>)?;
/// ```
///
/// # Notes
///
/// - Intrinsic tags are lower-case (for example `div`, `span`, `my-widget`).
/// - Uppercase tags are treated as component calls.
/// - Self-closing void elements (for example `input`, `br`) are emitted without children.
pub fn html(tokens: TokenStream) -> TokenStream {
    let input_tokens: proc_macro2::TokenStream = tokens.into();
    let parse_result = syn::parse2::<HtmlInvocationShape>(input_tokens.clone());

    if let Ok(shape) = parse_result {
        if shape.has_writer_prefix {
            return to_html::compile(input_tokens.into(), MacroMode::Html);
        }
    }

    let builder_ident = format_ident!("__fr_fragment_builder");
    let html_tokens: proc_macro2::TokenStream = to_html::compile(
        quote!(&mut #builder_ident, #input_tokens).into(),
        MacroMode::Html,
    )
    .into();

    quote! {
        {
            let mut #builder_ident = ::freshed_rs_runtime::FragmentBuilder::new();
            let __fr_render_result = #html_tokens;
            match __fr_render_result {
                Ok(()) => #builder_ident.finish(),
                Err(__fr_err) => {
                    panic!("html! fragment rendering failed: {:?}", __fr_err);
                }
            }
        }
    }
    .into()
}

#[proc_macro]
/// Renders markup into a `Result<HtmlFragment, RenderError>`.
///
/// This is the fallible fragment variant of `html!`.
///
/// # Example
///
/// ```ignore
/// use freshed_rs_macros::html_try;
///
/// let fragment = html_try!(<li>{"item"}</li>)?;
/// ```
///
/// # Input Shape
///
/// Accepts markup only. Writer-prefixed invocation is rejected.
pub fn html_try(tokens: TokenStream) -> TokenStream {
    let input_tokens: proc_macro2::TokenStream = tokens.into();
    let parse_result = syn::parse2::<HtmlInvocationShape>(input_tokens.clone());

    if let Ok(shape) = parse_result {
        if shape.has_writer_prefix {
            return syn::Error::new_spanned(
                input_tokens,
                "html_try! expects markup only, e.g. html_try!(<li>{value}</li>)",
            )
            .to_compile_error()
            .into();
        }
    }

    let builder_ident = format_ident!("__fr_fragment_builder");
    let html_tokens: proc_macro2::TokenStream = to_html::compile(
        quote!(&mut #builder_ident, #input_tokens).into(),
        MacroMode::Html,
    )
    .into();

    quote! {
        {
            let mut #builder_ident = ::freshed_rs_runtime::FragmentBuilder::new();
            let __fr_render_result = #html_tokens;
            match __fr_render_result {
                Ok(()) => ::core::result::Result::<
                    ::freshed_rs_runtime::HtmlFragment,
                    ::freshed_rs_runtime::RenderError,
                >::Ok(#builder_ident.finish()),
                Err(__fr_err) => ::core::result::Result::Err(__fr_err),
            }
        }
    }
    .into()
}

#[proc_macro]
/// Renders markup and returns a `Result<String, RenderError>`.
///
/// This is convenient for direct string rendering in tests or examples.
///
/// # Example
///
/// ```ignore
/// use freshed_rs_macros::html_to_string;
///
/// let html = html_to_string!(<div id="root">{"ok"}</div>)?;
/// ```
pub fn html_to_string(tokens: TokenStream) -> TokenStream {
    let markup_tokens: proc_macro2::TokenStream = tokens.into();
    let buffer_ident = format_ident!("__fr_rendered_html");

    let html_tokens: proc_macro2::TokenStream = to_html::compile(
        quote!( &mut #buffer_ident, #markup_tokens ).into(),
        MacroMode::Html,
    )
    .into();

    quote! {
        {
            let mut #buffer_ident = ::std::string::String::new();
            match #html_tokens {
                Ok(()) => ::core::result::Result::<::std::string::String, ::freshed_rs_runtime::RenderError>::Ok(#buffer_ident),
                Err(err) => ::core::result::Result::Err(err)
            }
        }
    }
    .into()
}

#[proc_macro]
/// Async variant of `html!`.
///
/// Expands to an async rendering expression that supports intrinsic markup and
/// async component calls.
///
/// # Example
///
/// ```ignore
/// use freshed_rs_macros::html_async;
///
/// # async fn run() -> Result<(), freshed_rs_runtime::RenderError> {
/// let mut out = String::new();
/// html_async!(&mut out, <div>{"hello"}</div>).await?;
/// # Ok(()) }
/// ```
pub fn html_async(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::HtmlAsync)
}

#[proc_macro]
/// Async-like convenience variant that returns a `Result<String, RenderError>`.
///
/// This macro renders markup into an owned `String`.
///
/// # Example
///
/// ```ignore
/// use freshed_rs_macros::html_async_to_string;
///
/// let html = html_async_to_string!(<section>{"ok"}</section>)?;
/// ```
pub fn html_async_to_string(tokens: TokenStream) -> TokenStream {
    let markup_tokens: proc_macro2::TokenStream = tokens.into();
    let buffer_ident = format_ident!("__fr_rendered_html");

    let html_tokens: proc_macro2::TokenStream = to_html::compile(
        quote!( &mut #buffer_ident, #markup_tokens ).into(),
        MacroMode::Html,
    )
    .into();

    quote! {
        {
            let mut #buffer_ident = ::std::string::String::new();
            match #html_tokens {
                Ok(()) => ::core::result::Result::<::std::string::String, ::freshed_rs_runtime::RenderError>::Ok(#buffer_ident),
                Err(err) => ::core::result::Result::Err(err)
            }
        }
    }
    .into()
}

#[proc_macro]
/// Context-aware variant of `html!`.
///
/// Expected input shape:
///
/// - `writer_expr, context_expr, <markup...>`
///
/// The context expression is evaluated once and threaded into component calls
/// that accept `(writer, ctx, props)` signatures.
///
/// # Example
///
/// ```ignore
/// use freshed_rs_macros::html_ctx;
///
/// let mut out = String::new();
/// html_ctx!(&mut out, app_ctx, <AppShell title={"home"} />)?;
/// ```
pub fn html_ctx(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::HtmlContext)
}

#[proc_macro]
/// Async context-aware variant of `html!`.
///
/// Expected input shape:
///
/// - `writer_expr, context_expr, <markup...>`
///
/// Supports async component invocation in context-aware rendering.
///
/// # Example
///
/// ```ignore
/// use freshed_rs_macros::html_async_ctx;
///
/// # async fn run() -> Result<(), freshed_rs_runtime::RenderError> {
/// let mut out = String::new();
/// html_async_ctx!(&mut out, app_ctx, <Dashboard async />).await?;
/// # Ok(()) }
/// ```
pub fn html_async_ctx(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::HtmlAsyncContext)
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
