use freshed_rs_macros::html_async;

fn main() {
    let mut out = String::new();
    futures::executor::block_on(html_async!(
        &mut out,
        <article data-id={7}><h1>{"hello"}</h1><p>{"stream"}</p></article>
    ))
    .expect("render should succeed");
}
