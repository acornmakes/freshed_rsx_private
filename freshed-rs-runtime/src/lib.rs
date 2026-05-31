use std::{
    fmt::{self, Display, Write},
    sync::atomic::{AtomicUsize, Ordering},
};

const INLINE_TEXT_CAPACITY: usize = 48;

mod iter;
mod name_guard;

pub use iter::{HtmlWriterIter, ToHtmlIter, html_each};
pub use name_guard::NameGuard;

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
    Literal(String),
    Escaped(TextStorage),
    Raw(String),
}

#[derive(Clone, Debug)]
pub enum TextStorage {
    Inline {
        len: usize,
        bytes: [u8; INLINE_TEXT_CAPACITY],
    },
    Heap(String),
}

impl TextStorage {
    pub fn from_string(text: String) -> Self {
        if text.len() <= INLINE_TEXT_CAPACITY {
            let mut bytes = [0u8; INLINE_TEXT_CAPACITY];
            bytes[..text.len()].copy_from_slice(text.as_bytes());
            Self::Inline {
                len: text.len(),
                bytes,
            }
        } else {
            Self::Heap(text)
        }
    }

    pub fn from_str(text: &str) -> Self {
        Self::from_string(text.to_string())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Inline { len, bytes } => std::str::from_utf8(&bytes[..*len])
                .expect("inline text should always be valid UTF-8"),
            Self::Heap(text) => text,
        }
    }

    pub fn into_string(self) -> String {
        match self {
            Self::Inline { len, bytes } => std::str::from_utf8(&bytes[..len])
                .expect("inline text should always be valid UTF-8")
                .to_string(),
            Self::Heap(text) => text,
        }
    }

    pub fn push_str(&mut self, suffix: &str) {
        if suffix.is_empty() {
            return;
        }

        match self {
            Self::Inline { len, bytes } if *len + suffix.len() <= INLINE_TEXT_CAPACITY => {
                bytes[*len..*len + suffix.len()].copy_from_slice(suffix.as_bytes());
                *len += suffix.len();
            }
            _ => {
                let mut heap = self.clone().into_string();
                heap.push_str(suffix);
                *self = Self::Heap(heap);
            }
        }
    }
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

    fn into_chunks_for_merge(self) -> Vec<FragmentChunk> {
        match self.inner {
            HtmlFragmentInner::Chunks(chunks) => chunks,
            HtmlFragmentInner::Materialized(html) => vec![FragmentChunk::Literal(html)],
        }
    }

    pub fn render_to<W: Write + ?Sized>(&self, out: &mut W) -> RenderResult {
        match &self.inner {
            HtmlFragmentInner::Chunks(chunks) => {
                for chunk in chunks {
                    match chunk {
                        FragmentChunk::Literal(literal) => out.write_str(literal)?,
                        FragmentChunk::Escaped(text) => write_escaped_into(out, text.as_str())?,
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
            HtmlFragmentInner::Materialized(html) => html,
            inner => {
                let fragment = HtmlFragment { inner };
                let mut out = String::new();
                fragment
                    .render_to(&mut out)
                    .expect("rendering into string should succeed");
                out
            }
        }
    }
}

impl Default for HtmlFragment {
    fn default() -> Self {
        Self::from_raw("")
    }
}

#[derive(Clone, Debug, Default)]
pub struct Children {
    fragment: HtmlFragment,
}

impl Children {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_fragment(fragment: HtmlFragment) -> Self {
        Self { fragment }
    }

    pub fn from_static(fragment: &'static str) -> Self {
        Self {
            fragment: HtmlFragment::from_raw(fragment),
        }
    }

    pub fn render_to<W: Write + ?Sized>(&self, out: &mut W) -> RenderResult {
        self.fragment.render_to(out)
    }

    pub fn into_fragment(self) -> HtmlFragment {
        self.fragment
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

    pub fn push_chunk(&mut self, chunk: FragmentChunk) {
        push_chunk_normalized(&mut self.chunks, chunk);
    }

    pub fn push_fragment(&mut self, fragment: HtmlFragment) {
        for chunk in fragment.into_chunks_for_merge() {
            self.push_chunk(chunk);
        }
    }

    pub fn finish(self) -> HtmlFragment {
        let mut chunks = self.chunks;
        normalize_chunks(&mut chunks);
        HtmlFragment::from_chunks(chunks)
    }
}

impl Write for FragmentBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }

        self.push_chunk(FragmentChunk::Literal(s.to_string()));

        Ok(())
    }
}

fn push_chunk_normalized(chunks: &mut Vec<FragmentChunk>, chunk: FragmentChunk) {
    if let Some(last) = chunks.last_mut() {
        match (last, &chunk) {
            (FragmentChunk::Literal(left), FragmentChunk::Literal(right)) => {
                left.push_str(right);
                return;
            }
            (FragmentChunk::Raw(left), FragmentChunk::Raw(right)) => {
                left.push_str(right);
                return;
            }
            (FragmentChunk::Escaped(left), FragmentChunk::Escaped(right)) => {
                left.push_str(right.as_str());
                return;
            }
            _ => {}
        }
    }

    chunks.push(chunk);
}

fn normalize_chunks(chunks: &mut Vec<FragmentChunk>) {
    let mut normalized = Vec::with_capacity(chunks.len());
    for chunk in chunks.drain(..) {
        push_chunk_normalized(&mut normalized, chunk);
    }
    *chunks = normalized;
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

// pub trait CollectHtmlFragmentExt: Iterator + Sized {
//     fn collect_html(self) -> HtmlSequence
//     where
//         Self::Item: Into<HtmlFragment>,
//     {
//         HtmlSequence::new(self.map(Into::into).collect())
//     }

//     fn collect_html_sequence(self) -> HtmlSequence
//     where
//         Self::Item: Into<HtmlFragment>,
//     {
//         self.collect_html()
//     }
// }

// impl<I: Iterator> CollectHtmlFragmentExt for I {}

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

impl HtmlValue for Children {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        self.fragment.render_to(out)
    }
}

impl HtmlValue for &Children {
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        self.fragment.render_to(out)
    }
}

impl From<RawHtml> for HtmlFragment {
    fn from(value: RawHtml) -> Self {
        HtmlFragment::from_chunks(vec![FragmentChunk::Raw(value.into_inner())])
    }
}

impl From<String> for HtmlFragment {
    fn from(value: String) -> Self {
        HtmlFragment::from_chunks(vec![FragmentChunk::Escaped(TextStorage::from_string(
            value,
        ))])
    }
}

impl From<&str> for HtmlFragment {
    fn from(value: &str) -> Self {
        HtmlFragment::from_chunks(vec![FragmentChunk::Escaped(TextStorage::from_str(value))])
    }
}

impl From<HtmlFragment> for Children {
    fn from(value: HtmlFragment) -> Self {
        Self::from_fragment(value)
    }
}

impl From<String> for Children {
    fn from(value: String) -> Self {
        Self::from_fragment(HtmlFragment::from(value))
    }
}

impl From<&str> for Children {
    fn from(value: &str) -> Self {
        Self::from_fragment(HtmlFragment::from(value))
    }
}

impl From<RawHtml> for Children {
    fn from(value: RawHtml) -> Self {
        Self::from_fragment(HtmlFragment::from(value))
    }
}

impl From<Children> for String {
    fn from(value: Children) -> Self {
        value.into_fragment().into_inner()
    }
}

impl<T> HtmlValue for T
where
    T: Display,
{
    fn write_html<W: Write + ?Sized>(self, out: &mut W) -> RenderResult {
        let display = self.to_string();
        write_escaped_into(out, &display)?;
        Ok(())
    }
}

fn write_escaped_into<W: Write + ?Sized>(out: &mut W, input: &str) -> RenderResult {
    for ch in input.chars() {
        match ch {
            '&' => out.write_str("&amp;")?,
            '<' => out.write_str("&lt;")?,
            '>' => out.write_str("&gt;")?,
            '"' => out.write_str("&quot;")?,
            '\'' => out.write_str("&#39;")?,
            _ => out.write_char(ch)?,
        }
    }

    Ok(())
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

static ID_GENERATOR: AtomicUsize = AtomicUsize::new(0);

/// Give me an ID, starts at 0 and increases
pub fn next_id_to_string() -> String {
    let id = ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
    format!("{id}")
}

#[derive(Default)]
pub struct IdGenerator<'s> {
    val: AtomicUsize,
    prefix: &'s str,
}

impl<'s> IdGenerator<'s> {
    pub fn new(prefix: &'s str) -> Self {
        IdGenerator {
            val: AtomicUsize::default(),
            prefix,
        }
    }

    pub fn new_starting_at(prefix: &'s str, starts_at: usize) -> Self {
        IdGenerator {
            val: AtomicUsize::new(starts_at),
            prefix,
        }
    }

    fn next(&self) -> usize {
        self.val.fetch_add(1, Ordering::SeqCst)
    }

    pub fn next_id(&self) -> String {
        format!("{}{}", self.prefix, self.next())
    }

    pub fn next_id_prefix(&self, prefix: &str) -> String {
        format!("{prefix}{}", self.next())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::IdGenerator;

    use super::{
        FragmentBuilder, FragmentChunk, HtmlFragment, HtmlSequence, INLINE_TEXT_CAPACITY, RawHtml,
        TextStorage, escape_html, write_attr, write_text,
    };

    #[test]
    fn id_counter_test() {
        let id_gen = IdGenerator::new_starting_at("id", 1);
        assert_eq!(id_gen.next_id(), format!("id1"));

        assert_eq!(id_gen.next_id_prefix("cheese"), format!("cheese2"));
        assert_eq!(id_gen.next_id(), format!("id3"));
        let id4: String = id_gen.next_id();
        assert_eq!(id4, format!("id4"));
    }

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
    fn fragment_builder_creates_fragment_without_final_buffer_concat() {
        let mut builder = FragmentBuilder::new();
        write!(&mut builder, "<li>{}</li>", 1).expect("write should succeed");
        write!(&mut builder, "<li>{}</li>", 2).expect("write should succeed");

        let fragment = builder.finish();
        let mut out = String::new();
        write_text(&mut out, fragment).expect("write_text should succeed");
        assert_eq!(out, "<li>1</li><li>2</li>");
    }

    #[test]
    fn escaped_chunk_is_escaped_at_render_time() {
        let fragment = HtmlFragment::from_chunks(vec![
            FragmentChunk::Literal("<p>".to_string()),
            FragmentChunk::Escaped(TextStorage::from_str("<x>&\"'")),
            FragmentChunk::Literal("</p>".to_string()),
        ]);

        let mut out = String::new();
        write_text(&mut out, fragment).expect("write_text should succeed");
        assert_eq!(out, "<p>&lt;x&gt;&amp;&quot;&#39;</p>");
    }

    #[test]
    fn text_storage_inlines_short_values() {
        let short = TextStorage::from_str("short");
        let long = TextStorage::from_string("x".repeat(INLINE_TEXT_CAPACITY + 1));

        assert!(matches!(short, TextStorage::Inline { .. }));
        assert!(matches!(long, TextStorage::Heap(_)));
    }

    #[test]
    fn fragment_builder_normalizes_adjacent_chunks_from_fragments() {
        let mut builder = FragmentBuilder::new();
        builder.push_fragment(HtmlFragment::from_chunks(vec![FragmentChunk::Escaped(
            TextStorage::from_str("a"),
        )]));
        builder.push_fragment(HtmlFragment::from_chunks(vec![FragmentChunk::Escaped(
            TextStorage::from_str("b"),
        )]));

        let fragment = builder.finish();
        let mut out = String::new();
        write_text(&mut out, fragment).expect("write_text should succeed");
        assert_eq!(out, "ab");
    }
}
