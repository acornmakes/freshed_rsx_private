use std::fmt::{self, Display, Write};

pub type RenderResult = Result<(), RenderError>;

#[derive(Debug)]
pub enum RenderError {
    Fmt(fmt::Error),
}

impl From<fmt::Error> for RenderError {
    fn from(value: fmt::Error) -> Self {
        Self::Fmt(value)
    }
}

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

pub fn write_text<W, T>(out: &mut W, value: T) -> RenderResult
where
    W: Write + ?Sized,
    T: HtmlValue,
{
    out.write_str(&value.into_rendered_html())?;
    Ok(())
}

pub fn write_attr<W, T>(out: &mut W, value: T) -> RenderResult
where
    W: Write + ?Sized,
    T: HtmlValue,
{
    out.write_str(&value.into_rendered_html())?;
    Ok(())
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
    use super::{RawHtml, escape_html, write_attr, write_text};

    #[test]
    fn escapes_html_metacharacters() {
        assert_eq!(escape_html("<>&\"'"), "&lt;&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn renders_text_values_as_escaped_html() {
        let mut out = String::new();
        write_text(&mut out, "<span>&\"'").expect("write_text should succeed");
        assert_eq!(out, "&lt;span&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn renders_attribute_values_as_escaped_html() {
        let mut out = String::new();
        write_attr(&mut out, "Tom & Jerry").expect("write_attr should succeed");
        assert_eq!(out, "Tom &amp; Jerry");
    }

    #[test]
    fn raw_html_skips_additional_escaping() {
        let raw = RawHtml::new("<strong>safe</strong>");
        let mut out = String::new();
        write_text(&mut out, raw).expect("write_text should succeed");
        assert_eq!(out, "<strong>safe</strong>");
    }
}
