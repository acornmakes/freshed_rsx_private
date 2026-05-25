use freshed_rs_macros::html;

fn main() {
    let page = "home";
    let _out = html!(
        <main data-page={page}>
            <header><h1>{"Welcome"}</h1></header>
            <section><p>{"content"}</p></section>
        </main>
    );
}
