use std::fmt::Display;

pub struct RawHtml(String);

impl RawHtml {
    pub fn new<S: Into<String>>(html: S) -> Self {
        Self(html.into())
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

pub trait HtmlValue {
    fn into_rendered_html(self) -> String;
}

pub fn render_text<T: HtmlValue>(value: T) -> String {
    value.into_rendered_html()
}

pub fn render_attr<T: HtmlValue>(value: T) -> String {
    value.into_rendered_html()
}

impl HtmlValue for RawHtml {
    fn into_rendered_html(self) -> String {
        self.0
    }
}

impl<T> HtmlValue for T
where
    T: Display,
{
    fn into_rendered_html(self) -> String {
        escape_html(&self.to_string())
    }
}

pub fn escape_html(input: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::{RawHtml, escape_html, render_attr, render_text};

    #[test]
    fn escapes_html_metacharacters() {
        assert_eq!(escape_html("<>&\"'"), "&lt;&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn renders_text_values_as_escaped_html() {
        assert_eq!(render_text("<span>&\"'"), "&lt;span&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn renders_attribute_values_as_escaped_html() {
        assert_eq!(render_attr("Tom & Jerry"), "Tom &amp; Jerry");
    }

    #[test]
    fn raw_html_skips_additional_escaping() {
        let raw = RawHtml::new("<strong>safe</strong>");
        assert_eq!(render_text(raw), "<strong>safe</strong>");
    }
}
