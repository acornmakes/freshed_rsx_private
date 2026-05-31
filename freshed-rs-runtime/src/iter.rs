use crate::{HtmlFragment, HtmlValue, RenderResult};

pub struct HtmlWriterIter<I> {
    iter: I,
}

pub fn html_each<I>(iter: I) -> HtmlWriterIter<I::IntoIter>
where
    I: IntoIterator,
{
    HtmlWriterIter {
        iter: iter.into_iter(),
    }
}

impl<I> HtmlValue for HtmlWriterIter<I>
where
    I: Iterator,
    I::Item: Into<HtmlFragment>,
{
    fn write_html<W: std::fmt::Write + ?Sized>(mut self, out: &mut W) -> RenderResult {
        for item in self.iter.by_ref() {
            let fragment: HtmlFragment = item.into();
            fragment.render_to(out)?;
        }
        Ok(())
    }
}

pub trait ToHtmlIter: IntoIterator {
    /// Convert to an iterator for Fragments
    fn into_html_iter(self) -> HtmlWriterIter<Self::IntoIter>
    where
        Self: Sized,
    {
        HtmlWriterIter {
            iter: self.into_iter(),
        }
    }
}

impl<I: IntoIterator> ToHtmlIter for I {}
