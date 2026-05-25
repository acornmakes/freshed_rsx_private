use freshed_rs_macros::html;

fn main() {
    let region = "us-east";
    let active = true;
    let _out = html!(
        <div data-region={region} data-active={active}>
            <h2>{"Title"}</h2>
            <p>{if active { "on" } else { "off" }}</p>
        </div>
    );
}
