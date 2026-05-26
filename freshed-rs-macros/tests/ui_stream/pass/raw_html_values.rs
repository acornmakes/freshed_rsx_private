use freshed_rs_macros::html;
use freshed_rs_runtime::RawHtml;

fn main() {
    let mut out = String::new();
    let raw = RawHtml::new("<strong>safe</strong>");
    html!(&mut out, <div title={RawHtml::new("trusted & raw")}>{raw}</div>)
        .expect("render should succeed");
}
