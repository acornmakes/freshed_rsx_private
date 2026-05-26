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

#[derive(Clone, Debug)]
pub enum FragmentChunk {
    Raw(String),
}

#[derive(Clone, Debug)]
enum HtmlFragmentInner {
    Chunks(Vec<FragmentChunk>),
    Materialized(String),
}

#[derive(Clone, Debug)]
pub struct HtmlFragment {
    inner: HtmlFragmentInner,
}

impl HtmlFragment {
    pub fn from_raw<S: Into<String>>(html: S) -> Self {
        Self {
            inner: HtmlFragmentInner::Materialized(html.into()),
        }
    }

    pub fn from_chunks(chunks: Vec<FragmentChunk>) -> Self {
        Self {
            inner: HtmlFragmentInner::Chunks(chunks),
        }
    }

    pub fn render_to<W: Write + ?Sized>(&self, out: &mut W) -> RenderResult {
        match &self.inner {
            HtmlFragmentInner::Chunks(chunks) => {
                for chunk in chunks {
                    match chunk {
                        FragmentChunk::Raw(raw) => out.write_str(raw)?,
                    }
                }
            }
            HtmlFragmentInner::Materialized(html) => {
                out.write_str(html)?;
            }
        }

        Ok(())
    }

    pub fn into_inner(self) -> String {
        match self.inner {
            HtmlFragmentInner::Chunks(chunks) => {
                let mut out = String::new();
                for chunk in chunks {
                    match chunk {
                        FragmentChunk::Raw(raw) => out.push_str(&raw),
                    }
                }
                out
            }
            HtmlFragmentInner::Materialized(html) => html,
        }
    }
}

impl Default for HtmlFragment {
    fn default() -> Self {
        Self::from_raw("")
    }
}

#[derive(Debug, Default)]
pub struct FragmentBuilder {
    chunks: Vec<FragmentChunk>,
}

impl FragmentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finish(self) -> HtmlFragment {
        HtmlFragment::from_chunks(self.chunks)
    }
}

impl Write for FragmentBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }

        if let Some(FragmentChunk::Raw(last)) = self.chunks.last_mut() {
            last.push_str(s);
        } else {
            self.chunks.push(FragmentChunk::Raw(s.to_string()));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct HtmlSequence {
    fragments: Vec<HtmlFragment>,
}

impl HtmlSequence {
    pub fn new(fragments: Vec<HtmlFragment>) -> Self {
        Self { fragments }
    }

    pub fn push<T: Into<HtmlFragment>>(&mut self, fragment: T) {
        self.fragments.push(fragment.into());
    }

    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }

    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    pub fn into_vec(self) -> Vec<HtmlFragment> {
        self.fragments
    }
}

pub trait CollectHtmlFragmentExt: Iterator + Sized {
    fn collect_html(self) -> HtmlSequence
    where
        Self::Item: Into<HtmlFragment>,
    {
        HtmlSequence::new(self.map(Into::into).collect())
    }

    fn collect_html_sequence(self) -> HtmlSequence
    where
        Self::Item: Into<HtmlFragment>,
    {
        self.collect_html()
    }
}

impl<I: Iterator> CollectHtmlFragmentExt for I {}

pub trait HtmlValue {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult;
}

pub fn write_text<W, T>(out: &mut W, value: T) -> RenderResult
where
    W: Write + ?Sized,
    T: HtmlValue,
{
    value.write_html(out)
}

pub fn write_attr<W, T>(out: &mut W, value: T) -> RenderResult
where
    W: Write + ?Sized,
    T: HtmlValue,
{
    value.write_html(out)
}

impl HtmlValue for RawHtml {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        out.write_str(&self.0)?;
        Ok(())
    }
}

impl HtmlValue for HtmlFragment {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        self.render_to(out)
    }
}

impl HtmlValue for HtmlSequence {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        for fragment in self.fragments {
            fragment.render_to(out)?;
        }
        Ok(())
    }
}

impl HtmlValue for &HtmlFragment {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        self.render_to(out)
    }
}

impl HtmlValue for &HtmlSequence {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        for fragment in &self.fragments {
            fragment.render_to(out)?;
        }
        Ok(())
    }
}

impl From<RawHtml> for HtmlFragment {
    fn from(value: RawHtml) -> Self {
        HtmlFragment::from_raw(value.into_inner())
    }
}

impl From<String> for HtmlFragment {
    fn from(value: String) -> Self {
        HtmlFragment::from_raw(escape_html(&value))
    }
}

impl From<&str> for HtmlFragment {
    fn from(value: &str) -> Self {
        HtmlFragment::from_raw(escape_html(value))
    }
}

impl<T> HtmlValue for T
where
    T: Display,
{
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        let escaped = escape_html(&self.to_string());
        out.write_str(&escaped)?;
        Ok(())
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
    use std::fmt::Write;

    use super::{
        CollectHtmlFragmentExt, FragmentBuilder, HtmlFragment, HtmlSequence, RawHtml, escape_html,
        write_attr, write_text,
    };

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

    #[test]
    fn html_fragment_renders_without_escaping() {
        let fragment = HtmlFragment::from_raw("<li>1</li>");
        let mut out = String::new();
        write_text(&mut out, fragment).expect("write_text should succeed");
        assert_eq!(out, "<li>1</li>");
    }

    #[test]
    fn html_sequence_renders_fragments_in_order() {
        let mut sequence = HtmlSequence::default();
        sequence.push(HtmlFragment::from_raw("<li>0</li>"));
        sequence.push(HtmlFragment::from_raw("<li>1</li>"));

        let mut out = String::new();
        write_text(&mut out, sequence).expect("write_text should succeed");
        assert_eq!(out, "<li>0</li><li>1</li>");
    }

    #[test]
    fn collect_html_sequence_gathers_iterator_items() {
        let seq = (0..3)
            .map(|i| HtmlFragment::from_raw(format!("<li>{i}</li>")))
            .collect_html_sequence();

        assert_eq!(seq.len(), 3);
        assert!(!seq.is_empty());

        let mut out = String::new();
        write_text(&mut out, seq).expect("write_text should succeed");
        assert_eq!(out, "<li>0</li><li>1</li><li>2</li>");
    }

    #[test]
    fn fragment_builder_creates_fragment_without_final_buffer_concat() {
        let mut builder = FragmentBuilder::new();
        write!(&mut builder, "<li>{}</li>", 1).expect("write should succeed");
        write!(&mut builder, "<li>{}</li>", 2).expect("write should succeed");

        let fragment = builder.finish();
        let mut out = String::new();
        write_text(&mut out, fragment).expect("write_text should succeed");
        assert_eq!(out, "<li>1</li><li>2</li>");
    }
}
