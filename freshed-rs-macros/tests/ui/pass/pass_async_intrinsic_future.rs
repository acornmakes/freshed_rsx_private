use freshed_rs_macros::html_async;

fn main() {
    let _future = html_async!(<article><p>{"async intrinsic"}</p></article>);
}
