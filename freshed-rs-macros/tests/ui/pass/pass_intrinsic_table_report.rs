use freshed_rs_macros::html;

fn main() {
    let total = 42;
    let _out = html!(
        <table>
            <thead><tr><th>{"Metric"}</th><th>{"Value"}</th></tr></thead>
            <tbody><tr><td>{"total"}</td><td>{total}</td></tr></tbody>
        </table>
    );
}
