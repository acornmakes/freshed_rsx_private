use freshed_rs_macros::html;

fn main() {
    let _ = html!(<div>ok</div>);
    compile_error!("intentional trybuild compile_fail baseline");
}
