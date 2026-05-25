mod to_html;

use proc_macro::TokenStream;

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    to_html::html_inner(tokens, false)
}

/// Same as html but also emit IDE helper statements.
#[proc_macro]
pub fn html_ide(tokens: TokenStream) -> TokenStream {
    to_html::html_inner(tokens, true)
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
