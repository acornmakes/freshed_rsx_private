use freshed_rs_macros::{html, html_async, html_async_ctx, html_ctx};
use freshed_rs_runtime::RawHtml;

fn main() {
    let text = "<span>5 & 6</span>";
    let title = "A & B < C";
    let _sync = html!(<div title={title}>{text}</div>);
    let _async = html_async!(<div title={title}>{text}</div>);

    let ctx = ("tenant", 1usize);
    let _sync_ctx = html_ctx!(ctx, <div title={RawHtml::new("trusted & raw")}>{RawHtml::new("<strong>safe</strong>")}</div>);
    let _async_ctx = html_async_ctx!(ctx, <div title={RawHtml::new("trusted & raw")}>{RawHtml::new("<strong>safe</strong>")}</div>);
}
