use freshed_rs_macros::html_ide;

pub mod docs {
    pub fn element() {}
}

fn main() {
    let _ = html_ide!(<div>ok</div>,);
}
