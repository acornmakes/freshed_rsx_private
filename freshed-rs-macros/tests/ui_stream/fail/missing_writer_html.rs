use freshed_rs_macros::html;

fn main() {
    let mut out = String::new();
    let _ = html!(&mut out, <div>oops</div>,);
}
