use freshed_rs_macros::html;

#[test]
fn snapshot_intrinsic_simple_element() {
    let rendered = html!(<div>divided</div>).to_string();
    insta::assert_snapshot!(rendered, @"<div>divided</div>");
}

#[test]
fn snapshot_intrinsic_expression_attribute() {
    let id = 123;
    let rendered = html!(<div data-id={id}>ok</div>).to_string();
    insta::assert_snapshot!(rendered, @"<div data-id=\"123\">ok</div>");
}

#[test]
fn snapshot_intrinsic_nested_markup() {
    let rendered = html!(<section><h1>Title</h1><p>Body</p></section>).to_string();
    insta::assert_snapshot!(rendered, @"<section><h1>Title</h1><p>Body</p></section>");
}
