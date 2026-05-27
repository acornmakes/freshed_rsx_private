use std::{cell::Cell, collections::HashSet, rc::Rc};

use quote::{ToTokens, quote, quote_spanned};
use rstml::{
    Parser, ParserConfig,
    node::{Node, NodeAttribute, NodeName},
    visitor::{Visitor, visit_attributes, visit_nodes},
};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum MacroMode {
    Html,
    HtmlAsync,
    HtmlContext,
    HtmlAsyncContext,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CompileMode {
    SyncNoCtx,
    AsyncNoCtx,
    SyncWithCtx,
    AsyncWithCtx,
}

impl MacroMode {
    fn requires_context_arg(self) -> bool {
        matches!(self, Self::HtmlContext | Self::HtmlAsyncContext)
    }

    fn macro_name(self) -> &'static str {
        match self {
            Self::Html => "html!",
            Self::HtmlAsync => "html_async!",
            Self::HtmlContext => "html_ctx!",
            Self::HtmlAsyncContext => "html_async_ctx!",
        }
    }

    fn compile_mode(self) -> CompileMode {
        match self {
            Self::Html => CompileMode::SyncNoCtx,
            Self::HtmlAsync => CompileMode::AsyncNoCtx,
            Self::HtmlContext => CompileMode::SyncWithCtx,
            Self::HtmlAsyncContext => CompileMode::AsyncWithCtx,
        }
    }
}

impl CompileMode {
    fn uses_context(self) -> bool {
        matches!(self, Self::SyncWithCtx | Self::AsyncWithCtx)
    }

    fn is_async(self) -> bool {
        matches!(self, Self::AsyncNoCtx | Self::AsyncWithCtx)
    }
}

struct WriterMarkupInput {
    writer_expr: syn::Expr,
    _comma: syn::Token![,],
    markup_tokens: proc_macro2::TokenStream,
}

impl Parse for WriterMarkupInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            writer_expr: input.parse()?,
            _comma: input.parse()?,
            markup_tokens: input.parse()?,
        })
    }
}

struct WriterCtxMarkupInput {
    writer_expr: syn::Expr,
    _comma_a: syn::Token![,],
    ctx_expr: syn::Expr,
    _comma_b: syn::Token![,],
    markup_tokens: proc_macro2::TokenStream,
}

impl Parse for WriterCtxMarkupInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            writer_expr: input.parse()?,
            _comma_a: input.parse()?,
            ctx_expr: input.parse()?,
            _comma_b: input.parse()?,
            markup_tokens: input.parse()?,
        })
    }
}

struct ParsedMacroInput {
    writer_expr: syn::Expr,
    context_expr: Option<syn::Expr>,
    markup_tokens: proc_macro2::TokenStream,
}

#[derive(Clone)]
struct ComponentTagPaths {
    component_fn_path: syn::Path,
    props_type_path: syn::Path,
}

#[derive(Clone)]
struct ComponentSymbolHint {
    component_fn_path: syn::Path,
    props_type_path: syn::Path,
}

#[derive(Clone)]
struct ParsedComponentProp {
    key: String,
    key_span: proc_macro2::Span,
    value_tokens: proc_macro2::TokenStream,
}

#[derive(Default)]
struct ParsedComponentProps {
    props: Vec<ParsedComponentProp>,
    has_children_prop: bool,
    children_key_span: Option<proc_macro2::Span>,
    async_marker_span: Option<proc_macro2::Span>,
    diagnostics: Vec<proc_macro2::TokenStream>,
}

#[derive(Default)]
struct WalkNodesOutput {
    statements: Vec<proc_macro2::TokenStream>,
    diagnostics: Vec<proc_macro2::TokenStream>,
    component_symbol_hints: Vec<ComponentSymbolHint>,
}

struct WalkNodes<'a> {
    compile_mode: CompileMode,
    writer_binding: syn::Ident,
    context_binding: Option<syn::Ident>,
    empty_elements: &'a HashSet<&'a str>,
    name_counter: Rc<Cell<usize>>,
    literal_buffer: String,
    output: WalkNodesOutput,
}

impl<'a> WalkNodes<'a> {
    fn child_output(&self, writer_binding: syn::Ident) -> Self {
        Self {
            compile_mode: self.compile_mode,
            writer_binding,
            context_binding: self.context_binding.clone(),
            empty_elements: self.empty_elements,
            name_counter: Rc::clone(&self.name_counter),
            literal_buffer: String::new(),
            output: WalkNodesOutput::default(),
        }
    }

    fn next_ident(&self, prefix: &str) -> syn::Ident {
        let idx = self.name_counter.get();
        self.name_counter.set(idx + 1);
        syn::Ident::new(
            &format!("__fr_{prefix}_{idx}"),
            proc_macro2::Span::call_site(),
        )
    }

    fn push_write_literal<S: AsRef<str>>(&mut self, literal: S) {
        let literal = literal.as_ref();
        if literal.is_empty() {
            return;
        }

        self.literal_buffer.push_str(literal);
    }

    fn flush_literal_buffer(&mut self) {
        if self.literal_buffer.is_empty() {
            return;
        }

        let writer_binding = &self.writer_binding;
        let literal = std::mem::take(&mut self.literal_buffer);
        self.output.statements.push(quote! {
            ::core::fmt::Write::write_str(#writer_binding, #literal)
                .map_err(::freshed_rs_runtime::RenderError::from)?;
        });
    }

    fn push_statement(&mut self, statement: proc_macro2::TokenStream) {
        self.flush_literal_buffer();
        self.output.statements.push(statement);
    }

    fn extend_output(&mut self, other: WalkNodesOutput) {
        self.flush_literal_buffer();
        self.output.extend(other);
    }

    fn into_output(mut self) -> WalkNodesOutput {
        self.flush_literal_buffer();
        self.output.coalesce_literal_write_statements();
        self.output
    }
}

impl<'a> syn::visit_mut::VisitMut for WalkNodes<'a> {}

impl<'a, C> Visitor<C> for WalkNodes<'a>
where
    C: rstml::node::CustomNode + 'static,
{
    fn visit_doctype(&mut self, doctype: &mut rstml::node::NodeDoctype) -> bool {
        let value = doctype.value.to_token_stream_string();
        self.push_write_literal(format!("<!DOCTYPE {}>", value));
        false
    }

    fn visit_text_node(&mut self, node: &mut rstml::node::NodeText) -> bool {
        self.push_write_literal(node.value_string());
        false
    }

    fn visit_raw_node<OtherC: rstml::node::CustomNode>(
        &mut self,
        node: &mut rstml::node::RawText<OtherC>,
    ) -> bool {
        self.push_write_literal(node.to_string_best());
        false
    }

    fn visit_fragment(&mut self, fragment: &mut rstml::node::NodeFragment<C>) -> bool {
        let visitor = self.child_output(self.writer_binding.clone());
        let child_output = visit_nodes(&mut fragment.children, visitor);
        self.extend_output(child_output.into_output());
        false
    }

    fn visit_comment(&mut self, comment: &mut rstml::node::NodeComment) -> bool {
        self.push_write_literal(format!(
            "<!-- {} -->",
            comment.value.to_token_stream().to_string()
        ));
        false
    }

    fn visit_block(&mut self, block: &mut rstml::node::NodeBlock) -> bool {
        let writer_binding = &self.writer_binding;
        self.push_statement(quote! {
            ::freshed_rs_runtime::write_text(#writer_binding, (#block))?;
        });
        false
    }

    fn visit_element(&mut self, element: &mut rstml::node::NodeElement<C>) -> bool {
        let name = element.name().to_string();

        if let Some(paths) = component_paths(&element.open_tag.name) {
            self.output
                .component_symbol_hints
                .push(ComponentSymbolHint {
                    component_fn_path: paths.component_fn_path.clone(),
                    props_type_path: paths.props_type_path.clone(),
                });

            let component_label = element.open_tag.name.to_string();
            let mut parsed_props =
                parse_component_props(&component_label, element.attributes_mut());

            if parsed_props.has_children_prop && !element.children.is_empty() {
                let span = parsed_props
                    .children_key_span
                    .unwrap_or_else(|| element.open_tag.name.span());
                let conflict = proc_macro2_diagnostics::Diagnostic::spanned(
                    span,
                    proc_macro2_diagnostics::Level::Error,
                    "children provided both as prop and as child nodes",
                );
                parsed_props
                    .diagnostics
                    .push(conflict.emit_as_expr_tokens());
            }

            let mut props_fields = Vec::new();
            for prop in &parsed_props.props {
                match ident_from_prop_key(&prop.key, prop.key_span) {
                    Ok(ident) => {
                        let value_tokens = prop.value_tokens.clone();
                        props_fields.push(quote_spanned!(prop.key_span => #ident: #value_tokens));
                    }
                    Err(diagnostic) => parsed_props.diagnostics.push(diagnostic),
                }
            }

            if !parsed_props.has_children_prop && !element.children.is_empty() {
                let children_ident = self.next_ident("children");
                let children_writer_ident = self.next_ident("children_out");

                let child_visitor = self.child_output(children_writer_ident.clone());
                let child_output = visit_nodes(&mut element.children, child_visitor);
                let WalkNodesOutput {
                    statements: child_statements,
                    diagnostics,
                    component_symbol_hints,
                } = child_output.into_output();
                self.output.diagnostics.extend(diagnostics);
                self.output
                    .component_symbol_hints
                    .extend(component_symbol_hints);
                self.push_statement(quote! {
                    let mut #children_ident = ::std::string::String::new();
                    {
                        let #children_writer_ident = &mut #children_ident;
                        #(#child_statements)*
                    }
                });
                props_fields.push(
                    quote_spanned!(element.open_tag.name.span() => children: #children_ident),
                );
            }

            self.output.diagnostics.extend(parsed_props.diagnostics);

            if parsed_props.async_marker_span.is_some() && !self.compile_mode.is_async() {
                let sync_marker_error = proc_macro2_diagnostics::Diagnostic::spanned(
                    parsed_props
                        .async_marker_span
                        .unwrap_or_else(|| element.open_tag.name.span()),
                    proc_macro2_diagnostics::Level::Error,
                    "async component marker is only supported in html_async! and html_async_ctx!",
                );
                self.output
                    .diagnostics
                    .push(sync_marker_error.emit_as_expr_tokens());
            }

            let await_suffix =
                if parsed_props.async_marker_span.is_some() && self.compile_mode.is_async() {
                    quote!(.await)
                } else {
                    quote!()
                };

            let writer_binding = &self.writer_binding;
            let component_fn_path = paths.component_fn_path;
            let props_type_path = paths.props_type_path;

            let call_stmt = if self.compile_mode.uses_context() {
                if let Some(ctx_ident) = &self.context_binding {
                    quote_spanned! { element.open_tag.name.span() =>
                        #component_fn_path(
                            #writer_binding,
                            #ctx_ident,
                            #props_type_path {
                                #(#props_fields,)*
                                ..::core::default::Default::default()
                            }
                        )#await_suffix?;
                    }
                } else {
                    let missing_ctx = proc_macro2_diagnostics::Diagnostic::spanned(
                        element.open_tag.name.span(),
                        proc_macro2_diagnostics::Level::Error,
                        "internal error: missing context binding for html_ctx/html_async_ctx",
                    );
                    self.output
                        .diagnostics
                        .push(missing_ctx.emit_as_expr_tokens());
                    quote! {}
                }
            } else {
                quote_spanned! { element.open_tag.name.span() =>
                    #component_fn_path(
                        #writer_binding,
                        #props_type_path {
                            #(#props_fields,)*
                            ..::core::default::Default::default()
                        }
                    )#await_suffix?;
                }
            };

            self.push_statement(call_stmt);
            return false;
        }

        self.push_write_literal(format!("<{}", name));

        let visitor = self.child_output(self.writer_binding.clone());
        let attribute_visitor = visit_attributes(element.attributes_mut(), visitor);
        self.extend_output(attribute_visitor.into_output());

        if self
            .empty_elements
            .contains(element.open_tag.name.to_string().as_str())
        {
            if !element.children.is_empty() {
                let warning = proc_macro2_diagnostics::Diagnostic::spanned(
                    element.open_tag.name.span(),
                    proc_macro2_diagnostics::Level::Warning,
                    "Element is processed as empty, and cannot have any child",
                );
                self.output.diagnostics.push(warning.emit_as_expr_tokens());
            }

            self.push_write_literal("/>");
            return false;
        }

        self.push_write_literal(">");

        let visitor = self.child_output(self.writer_binding.clone());
        let child_output = visit_nodes(&mut element.children, visitor);
        self.extend_output(child_output.into_output());
        self.push_write_literal(format!("</{}>", name));

        false
    }

    fn visit_attribute(&mut self, attribute: &mut NodeAttribute) -> bool {
        match attribute {
            NodeAttribute::Block(block) => {
                self.push_write_literal(" ");
                let writer_binding = &self.writer_binding;
                self.push_statement(quote! {
                    ::freshed_rs_runtime::write_attr(#writer_binding, (#block))?;
                });
            }
            NodeAttribute::Attribute(attribute) => {
                self.push_write_literal(format!(" {}", attribute.key));
                if let Some(value) = attribute.value() {
                    self.push_write_literal("=\"");
                    if let Some(static_attr_value) = compile_time_attr_string_literal(value) {
                        self.push_write_literal(static_attr_value);
                    } else {
                        let writer_binding = &self.writer_binding;
                        self.push_statement(quote! {
                            ::freshed_rs_runtime::write_attr(#writer_binding, (#value))?;
                        });
                    }
                    self.push_write_literal("\"");
                }
            }
        }

        false
    }
}

impl WalkNodesOutput {
    fn extend(&mut self, other: WalkNodesOutput) {
        self.statements.extend(other.statements);
        self.diagnostics.extend(other.diagnostics);
        self.component_symbol_hints
            .extend(other.component_symbol_hints);
    }

    fn coalesce_literal_write_statements(&mut self) {
        let mut merged = Vec::with_capacity(self.statements.len());
        let mut pending: Option<(proc_macro2::TokenStream, String)> = None;

        for statement in self.statements.drain(..) {
            if let Some((writer, literal)) = parse_literal_write_statement(&statement) {
                match &mut pending {
                    Some((pending_writer, pending_literal))
                        if pending_writer.to_string() == writer.to_string() =>
                    {
                        pending_literal.push_str(&literal);
                    }
                    Some((pending_writer, pending_literal)) => {
                        merged.push(build_literal_write_statement(
                            pending_writer.clone(),
                            pending_literal,
                        ));
                        pending = Some((writer, literal));
                    }
                    None => pending = Some((writer, literal)),
                }
            } else {
                if let Some((pending_writer, pending_literal)) = pending.take() {
                    merged.push(build_literal_write_statement(
                        pending_writer,
                        &pending_literal,
                    ));
                }
                merged.push(statement);
            }
        }

        if let Some((pending_writer, pending_literal)) = pending.take() {
            merged.push(build_literal_write_statement(
                pending_writer,
                &pending_literal,
            ));
        }

        self.statements = merged;
    }
}

fn build_literal_write_statement(
    writer: proc_macro2::TokenStream,
    literal: &str,
) -> proc_macro2::TokenStream {
    quote! {
        ::core::fmt::Write::write_str(#writer, #literal)
            .map_err(::freshed_rs_runtime::RenderError::from)?;
    }
}

fn parse_literal_write_statement(
    statement: &proc_macro2::TokenStream,
) -> Option<(proc_macro2::TokenStream, String)> {
    let parsed = syn::parse2::<syn::Stmt>(statement.clone()).ok()?;
    let expr = match parsed {
        syn::Stmt::Expr(expr, _) => expr,
        _ => return None,
    };

    let try_expr = match expr {
        syn::Expr::Try(try_expr) => try_expr,
        _ => return None,
    };

    let map_err_call = match *try_expr.expr {
        syn::Expr::MethodCall(call) if call.method == "map_err" => call,
        _ => return None,
    };

    let write_call = match *map_err_call.receiver {
        syn::Expr::Call(call) => call,
        _ => return None,
    };

    let write_fn = match *write_call.func {
        syn::Expr::Path(path) => path,
        _ => return None,
    };

    if write_fn.path.segments.last()?.ident != "write_str" {
        return None;
    }

    if write_call.args.len() != 2 {
        return None;
    }

    let writer_expr = write_call.args.first()?.to_token_stream();
    let literal_expr = write_call.args.iter().nth(1)?;
    let literal = match literal_expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(lit),
            ..
        }) => lit.value(),
        _ => return None,
    };

    Some((writer_expr, literal))
}

fn compile_time_attr_string_literal(value: &impl ToTokens) -> Option<String> {
    let expression = syn::parse2::<syn::Expr>(value.to_token_stream()).ok()?;
    let literal = match strip_grouping_expr(&expression) {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(lit),
            ..
        }) => lit.value(),
        _ => return None,
    };

    Some(escape_html_literal(&literal))
}

fn strip_grouping_expr(mut expression: &syn::Expr) -> &syn::Expr {
    loop {
        expression = match expression {
            syn::Expr::Group(group) => &group.expr,
            syn::Expr::Paren(paren) => &paren.expr,
            _ => return expression,
        };
    }
}

fn escape_html_literal(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn is_component_tag(name: &NodeName) -> bool {
    let raw = name.to_string();

    if raw.contains("::") {
        return true;
    }

    raw.chars()
        .next()
        .map(|ch| ch.is_ascii_uppercase())
        .unwrap_or(false)
}

fn component_paths(name: &NodeName) -> Option<ComponentTagPaths> {
    if !is_component_tag(name) {
        return None;
    }

    let component_fn_path: syn::Path = syn::parse2(name.to_token_stream()).ok()?;

    let mut props_type_path = component_fn_path.clone();
    let last_segment = props_type_path.segments.last_mut()?;
    let props_ident = syn::Ident::new(
        &format!("{}Props", last_segment.ident),
        last_segment.ident.span(),
    );
    last_segment.ident = props_ident;

    Some(ComponentTagPaths {
        component_fn_path,
        props_type_path,
    })
}

fn parse_component_props(
    component_label: &str,
    attributes: &mut [NodeAttribute],
) -> ParsedComponentProps {
    let mut parsed = ParsedComponentProps::default();
    let mut seen_keys: HashSet<String> = HashSet::new();

    for attribute in attributes {
        match attribute {
            NodeAttribute::Attribute(attribute) => {
                let key = attribute.key.to_string();

                if key == "async" {
                    if let Some(value) = attribute.value() {
                        let invalid = proc_macro2_diagnostics::Diagnostic::spanned(
                            value.span(),
                            proc_macro2_diagnostics::Level::Error,
                            "async component marker must be a bare attribute, e.g. <Card async />",
                        );
                        parsed.diagnostics.push(invalid.emit_as_expr_tokens());
                    }

                    if parsed.async_marker_span.is_some() {
                        let duplicate = proc_macro2_diagnostics::Diagnostic::spanned(
                            attribute.key.span(),
                            proc_macro2_diagnostics::Level::Error,
                            format!("duplicate async marker on component '{}'", component_label),
                        );
                        parsed.diagnostics.push(duplicate.emit_as_expr_tokens());
                    } else {
                        parsed.async_marker_span = Some(attribute.key.span());
                    }

                    continue;
                }

                if !seen_keys.insert(key.clone()) {
                    let duplicate = proc_macro2_diagnostics::Diagnostic::spanned(
                        attribute.key.span(),
                        proc_macro2_diagnostics::Level::Error,
                        format!(
                            "duplicate property '{}' on component '{}'",
                            key, component_label
                        ),
                    );
                    parsed.diagnostics.push(duplicate.emit_as_expr_tokens());
                    continue;
                }

                if key == "children" {
                    parsed.has_children_prop = true;
                    parsed.children_key_span = Some(attribute.key.span());
                }

                let value_tokens = attribute
                    .value()
                    .map(ToTokens::to_token_stream)
                    .unwrap_or_else(|| quote!(true));

                parsed.props.push(ParsedComponentProp {
                    key,
                    key_span: attribute.key.span(),
                    value_tokens,
                });
            }
            NodeAttribute::Block(block) => {
                let block_expr_tokens = block.to_token_stream();
                let shorthand_ident = shorthand_ident_from_block_expr(&block_expr_tokens);

                let ident = match shorthand_ident {
                    Some(ident) => ident,
                    None => {
                        let invalid = proc_macro2_diagnostics::Diagnostic::spanned(
                            block.span(),
                            proc_macro2_diagnostics::Level::Error,
                            "component shorthand prop must be an identifier, e.g. {value}",
                        );
                        parsed.diagnostics.push(invalid.emit_as_expr_tokens());
                        continue;
                    }
                };

                let key = ident.to_string();
                if !seen_keys.insert(key.clone()) {
                    let duplicate = proc_macro2_diagnostics::Diagnostic::spanned(
                        ident.span(),
                        proc_macro2_diagnostics::Level::Error,
                        format!(
                            "duplicate property '{}' on component '{}'",
                            key, component_label
                        ),
                    );
                    parsed.diagnostics.push(duplicate.emit_as_expr_tokens());
                    continue;
                }

                if key == "children" {
                    parsed.has_children_prop = true;
                    parsed.children_key_span = Some(ident.span());
                }

                parsed.props.push(ParsedComponentProp {
                    key,
                    key_span: ident.span(),
                    value_tokens: quote_spanned!(ident.span()=> #ident),
                });
            }
        }
    }

    parsed
}

fn shorthand_ident_from_block_expr(tokens: &proc_macro2::TokenStream) -> Option<syn::Ident> {
    if let Some(ident) = shorthand_ident_from_raw_block_tokens(tokens) {
        return Some(ident);
    }

    let expression = syn::parse2::<syn::Expr>(tokens.clone()).ok()?;
    shorthand_ident_from_expr(&expression)
}

fn shorthand_ident_from_raw_block_tokens(tokens: &proc_macro2::TokenStream) -> Option<syn::Ident> {
    let mut outer = tokens.clone().into_iter();
    let group = match (outer.next(), outer.next()) {
        (Some(proc_macro2::TokenTree::Group(group)), None)
            if group.delimiter() == proc_macro2::Delimiter::Brace =>
        {
            group
        }
        _ => return None,
    };

    let mut inner = group.stream().into_iter();
    match (inner.next(), inner.next()) {
        (Some(proc_macro2::TokenTree::Ident(ident)), None) => Some(ident),
        _ => None,
    }
}

fn shorthand_ident_from_expr(expression: &syn::Expr) -> Option<syn::Ident> {
    match expression {
        syn::Expr::Path(path)
            if path.qself.is_none()
                && path.path.leading_colon.is_none()
                && path.path.segments.len() == 1 =>
        {
            path.path.segments.first().map(|seg| seg.ident.clone())
        }
        syn::Expr::Block(block) => {
            if block.block.stmts.len() != 1 {
                return None;
            }

            match &block.block.stmts[0] {
                syn::Stmt::Expr(inner, None) => shorthand_ident_from_expr(inner),
                _ => None,
            }
        }
        _ => None,
    }
}

fn ident_from_prop_key(
    key: &str,
    key_span: proc_macro2::Span,
) -> Result<syn::Ident, proc_macro2::TokenStream> {
    match syn::parse_str::<syn::Ident>(key) {
        Ok(mut ident) => {
            ident.set_span(key_span);
            Ok(ident)
        }
        Err(_) => {
            let diagnostic = proc_macro2_diagnostics::Diagnostic::spanned(
                key_span,
                proc_macro2_diagnostics::Level::Error,
                format!(
                    "component property '{}' must be a valid Rust identifier",
                    key
                ),
            );
            Err(diagnostic.emit_as_expr_tokens())
        }
    }
}

pub(crate) fn compile(tokens: proc_macro::TokenStream, mode: MacroMode) -> proc_macro::TokenStream {
    let parsed = match parse_macro_input(tokens, mode) {
        Ok(parsed) => parsed,
        Err(error) => return error.to_compile_error().into(),
    };

    html_ctxner(
        parsed.writer_expr,
        parsed.context_expr,
        parsed.markup_tokens.into(),
        mode.compile_mode(),
    )
}

fn parse_macro_input(
    tokens: proc_macro::TokenStream,
    mode: MacroMode,
) -> syn::Result<ParsedMacroInput> {
    let tokens: proc_macro2::TokenStream = tokens.into();

    if mode.requires_context_arg() {
        let parsed = syn::parse2::<WriterCtxMarkupInput>(tokens).map_err(|_| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "{} expects input in the form: writer_expr, context_expr, <markup...>",
                    mode.macro_name()
                ),
            )
        })?;

        if parsed.markup_tokens.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "{} is missing markup after writer and context arguments",
                    mode.macro_name()
                ),
            ));
        }
        reject_trailing_markup_garbage(&parsed.markup_tokens)?;

        return Ok(ParsedMacroInput {
            writer_expr: parsed.writer_expr,
            context_expr: Some(parsed.ctx_expr),
            markup_tokens: parsed.markup_tokens,
        });
    }

    let parsed = syn::parse2::<WriterMarkupInput>(tokens).map_err(|_| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "{} expects input in the form: writer_expr, <markup...>",
                mode.macro_name()
            ),
        )
    })?;
    if parsed.markup_tokens.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "{} is missing markup after writer argument",
                mode.macro_name()
            ),
        ));
    }
    reject_trailing_markup_garbage(&parsed.markup_tokens)?;

    Ok(ParsedMacroInput {
        writer_expr: parsed.writer_expr,
        context_expr: None,
        markup_tokens: parsed.markup_tokens,
    })
}

fn reject_trailing_markup_garbage(markup_tokens: &proc_macro2::TokenStream) -> syn::Result<()> {
    for token_tree in markup_tokens.clone() {
        if let proc_macro2::TokenTree::Punct(punct) = token_tree {
            if punct.as_char() == ',' {
                return Err(syn::Error::new(
                    punct.span(),
                    "trailing non-markup tokens after parsed markup are not allowed",
                ));
            }
        }
    }

    Ok(())
}

fn walk_nodes<'a>(
    compile_mode: CompileMode,
    writer_binding: syn::Ident,
    context_binding: Option<syn::Ident>,
    empty_elements: &'a HashSet<&'a str>,
    nodes: &'a mut [Node],
) -> WalkNodesOutput {
    let visitor = WalkNodes {
        compile_mode,
        writer_binding,
        context_binding,
        empty_elements,
        name_counter: Rc::new(Cell::new(0)),
        literal_buffer: String::new(),
        output: WalkNodesOutput::default(),
    };
    let mut nodes = nodes.to_vec();
    let output = visit_nodes(&mut nodes, visitor);
    output.into_output()
}

fn trailing_garbage_diagnostics(nodes: &[Node]) -> Vec<proc_macro2::TokenStream> {
    let mut seen_non_text = false;
    let mut diagnostics = Vec::new();

    for node in nodes {
        match node {
            Node::Text(text) => {
                if seen_non_text && !text.value_string().trim().is_empty() {
                    let diagnostic = proc_macro2_diagnostics::Diagnostic::spanned(
                        text.span(),
                        proc_macro2_diagnostics::Level::Error,
                        "trailing non-markup tokens after top-level markup are not allowed",
                    );
                    diagnostics.push(diagnostic.emit_as_expr_tokens());
                }
            }
            _ => seen_non_text = true,
        }
    }

    diagnostics
}

pub(crate) fn html_ctxner(
    writer_expr: syn::Expr,
    context_expr: Option<syn::Expr>,
    tokens: proc_macro::TokenStream,
    compile_mode: CompileMode,
) -> proc_macro::TokenStream {
    let empty_elements: HashSet<_> = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]
    .into_iter()
    .collect();

    let config = ParserConfig::new()
        .recover_block(true)
        .always_self_closed_elements(empty_elements.clone())
        .raw_text_elements(["script", "style"].into_iter().collect());

    let parser = Parser::new(config);
    let (mut nodes, errors) = parser.parse_recoverable(tokens).split_vec();
    let trailing_diagnostics = trailing_garbage_diagnostics(&nodes);

    let writer_binding = syn::Ident::new("__fr_out", proc_macro2::Span::call_site());
    let context_binding = if compile_mode.uses_context() {
        Some(syn::Ident::new("__fr_ctx", proc_macro2::Span::call_site()))
    } else {
        None
    };

    let WalkNodesOutput {
        statements,
        component_symbol_hints,
        diagnostics,
    } = walk_nodes(
        compile_mode,
        writer_binding.clone(),
        context_binding.clone(),
        &empty_elements,
        &mut nodes,
    );

    let component_hint_statements = component_symbol_hints.into_iter().map(|hint| {
        let component_fn_path = hint.component_fn_path;
        let props_type_path = hint.props_type_path;
        quote_spanned! { component_fn_path.span() =>
            #[allow(unused)]
            use #component_fn_path as _;
            #[allow(unused)]
            let _: ::core::option::Option<#props_type_path> = ::core::option::Option::None;
        }
    });

    let errors = errors
        .into_iter()
        .map(|e| e.emit_as_expr_tokens())
        .chain(diagnostics)
        .chain(trailing_diagnostics);

    let writer_binding_stmt = quote!(let #writer_binding = #writer_expr;);
    let context_binding_stmt = match (compile_mode.uses_context(), context_binding, context_expr) {
        (true, Some(binding), Some(expr)) => quote!(let #binding = (#expr);),
        (true, Some(binding), None) => {
            let diagnostic = proc_macro2_diagnostics::Diagnostic::spanned(
                proc_macro2::Span::call_site(),
                proc_macro2_diagnostics::Level::Error,
                "internal error: missing context expression for html_ctx/html_async_ctx",
            );
            let emitted = diagnostic.emit_as_expr_tokens();
            quote!(#emitted; let #binding = ();)
        }
        _ => quote!(),
    };

    let render_expr = if compile_mode.is_async() {
        quote! {
            async {
                #writer_binding_stmt
                #context_binding_stmt
                #(#statements)*
                ::core::result::Result::<(), ::freshed_rs_runtime::RenderError>::Ok(())
            }
        }
    } else {
        quote! {
            {
                #writer_binding_stmt
                #context_binding_stmt
                (|| -> ::freshed_rs_runtime::RenderResult {
                    #(#statements)*
                    Ok(())
                })()
            }
        }
    };

    quote! {
        {
            #(#errors;)*
            #(#component_hint_statements)*
            #render_expr
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::{
        compile_time_attr_string_literal, component_paths, escape_html_literal, is_component_tag,
        parse_component_props,
    };
    use quote::ToTokens;
    use rstml::{Parser, ParserConfig, node::Node};

    fn parse_first_element_name(markup: &str) -> rstml::node::NodeName {
        let tokens: proc_macro2::TokenStream = markup.parse().expect("valid markup tokens");
        let parser = Parser::new(ParserConfig::new().recover_block(true));
        let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
        assert!(errors.is_empty(), "unexpected parse errors: {errors:?}");

        nodes
            .into_iter()
            .find_map(|node| match node {
                Node::Element(element) => Some(element.open_tag.name),
                _ => None,
            })
            .expect("expected first element")
    }

    fn parse_first_element(markup: &str) -> rstml::node::NodeElement<rstml::node::Infallible> {
        let tokens: proc_macro2::TokenStream = markup.parse().expect("valid markup tokens");
        let parser = Parser::new(ParserConfig::new().recover_block(true));
        let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
        assert!(errors.is_empty(), "unexpected parse errors: {errors:?}");

        nodes
            .into_iter()
            .find_map(|node| match node {
                Node::Element(element) => Some(element),
                _ => None,
            })
            .expect("expected first element")
    }

    #[test]
    fn classifies_lowercase_intrinsic_tags_as_non_component() {
        let name = parse_first_element_name("<div>ok</div>");
        assert!(!is_component_tag(&name));
        assert!(component_paths(&name).is_none());
    }

    #[test]
    fn classifies_custom_element_tags_as_non_component() {
        let name = parse_first_element_name("<my-widget>ok</my-widget>");
        assert!(!is_component_tag(&name));
        assert!(component_paths(&name).is_none());
    }

    #[test]
    fn classifies_uppercase_tags_as_components() {
        let name = parse_first_element_name("<Button>ok</Button>");
        let paths = component_paths(&name).expect("component paths");

        assert!(is_component_tag(&name));
        assert_eq!(
            paths.component_fn_path.to_token_stream().to_string(),
            "Button"
        );
        assert_eq!(
            paths.props_type_path.to_token_stream().to_string(),
            "ButtonProps"
        );
    }

    #[test]
    fn supports_component_tag_paths() {
        let name = parse_first_element_name("<crate::ui::Button />");
        let paths = component_paths(&name).expect("component paths");

        assert!(is_component_tag(&name));
        assert_eq!(
            paths.component_fn_path.to_token_stream().to_string(),
            "crate :: ui :: Button"
        );
        assert_eq!(
            paths.props_type_path.to_token_stream().to_string(),
            "crate :: ui :: ButtonProps"
        );
    }

    #[test]
    fn parses_async_marker_and_regular_props() {
        let mut element = parse_first_element(r#"<Card async title="Welcome" count={3} />"#);
        let parsed = parse_component_props("Card", element.attributes_mut());

        assert!(parsed.diagnostics.is_empty());
        assert!(parsed.async_marker_span.is_some());
        assert_eq!(parsed.props.len(), 2);
        assert_eq!(parsed.props[0].key, "title");
        assert_eq!(parsed.props[1].key, "count");
    }

    #[test]
    fn parses_shorthand_props_as_identifier_values() {
        let mut element = parse_first_element("<Button {tone} {variant} />");
        let parsed = parse_component_props("Button", element.attributes_mut());

        assert!(parsed.diagnostics.is_empty());
        assert_eq!(parsed.props.len(), 2);
        assert_eq!(parsed.props[0].key, "tone");
        assert_eq!(parsed.props[1].key, "variant");

        let tone_tokens = parsed.props[0].value_tokens.to_string();
        let variant_tokens = parsed.props[1].value_tokens.to_string();
        assert_eq!(tone_tokens, "tone");
        assert_eq!(variant_tokens, "variant");
    }

    #[test]
    fn reports_duplicate_component_props() {
        let mut element = parse_first_element("<Button kind=\"a\" kind=\"b\" />");
        let parsed = parse_component_props("Button", element.attributes_mut());

        assert_eq!(parsed.props.len(), 1);
        assert_eq!(parsed.diagnostics.len(), 1);
    }

    #[test]
    fn reports_invalid_shorthand_component_props() {
        let mut element = parse_first_element("<Button {a + b} />");
        let parsed = parse_component_props("Button", element.attributes_mut());

        assert!(parsed.props.is_empty());
        assert_eq!(parsed.diagnostics.len(), 1);
    }

    #[test]
    fn tracks_children_prop_presence_and_span() {
        let mut element = parse_first_element("<Card children={content} />");
        let parsed = parse_component_props("Card", element.attributes_mut());

        assert!(parsed.has_children_prop);
        assert!(parsed.children_key_span.is_some());
        assert_eq!(parsed.props.len(), 1);
        assert_eq!(parsed.props[0].key, "children");
    }

    #[test]
    fn reports_duplicate_async_markers() {
        let mut element = parse_first_element("<Card async async />");
        let parsed = parse_component_props("Card", element.attributes_mut());

        assert!(parsed.async_marker_span.is_some());
        assert_eq!(parsed.diagnostics.len(), 1);
    }

    #[test]
    fn reports_async_marker_value_usage() {
        let mut element = parse_first_element("<Card async={true} />");
        let parsed = parse_component_props("Card", element.attributes_mut());

        assert!(parsed.async_marker_span.is_some());
        assert_eq!(parsed.diagnostics.len(), 1);
    }

    #[test]
    fn diagnostics_keep_prop_key_spans() {
        let mut element = parse_first_element("<Card role=\"a\" role=\"b\" />");
        let parsed = parse_component_props("Card", element.attributes_mut());

        let duplicate_key_span = parsed.props[0].key_span;
        assert_eq!(parsed.props[0].key, "role");
        assert_eq!(duplicate_key_span.start().line, 1);
    }

    #[test]
    fn escapes_compile_time_attr_string_literal() {
        let expression: syn::Expr = syn::parse_quote!("Tom & Jerry < \"best\"");
        let rendered = compile_time_attr_string_literal(&expression).expect("string literal");
        assert_eq!(rendered, "Tom &amp; Jerry &lt; &quot;best&quot;");
    }

    #[test]
    fn compile_time_attr_string_literal_ignores_non_string_literals() {
        let expression: syn::Expr = syn::parse_quote!(123);
        assert!(compile_time_attr_string_literal(&expression).is_none());
    }

    #[test]
    fn escape_html_literal_matches_runtime_contract() {
        assert_eq!(escape_html_literal("<>&\"'"), "&lt;&gt;&amp;&quot;&#39;");
    }
}
