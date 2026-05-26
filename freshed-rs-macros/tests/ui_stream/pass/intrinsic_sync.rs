use freshed_rs_macros::html;

fn main() {
    let mut out = String::new();
    html!(&mut out, <div data-id={42}>{"ok"}</div>).expect("render should succeed");
}
