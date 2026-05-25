use freshed_rs_macros::html;

fn main() {
    let name = "alice";
    let _out = html!(
        <form method={"post"}>
            <input type={"text"} value={name} />
            <button type={"submit"}>{"Save"}</button>
        </form>
    );
}
