// Component pipeline tracking (Phase 0 baseline):
// - preserve intrinsic html! behavior via regression snapshots
// - keep compile-fail harness in place for future diagnostic tests
// - route generated symbol naming through the __fr_* convention
mod to_html;

use proc_macro::TokenStream;
use to_html::MacroMode;

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::Html)
}

/// Same as html but also emit IDE helper statements.
#[proc_macro]
pub fn html_ide(tokens: TokenStream) -> TokenStream {
    to_html::compile(tokens, MacroMode::HtmlIde)
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
