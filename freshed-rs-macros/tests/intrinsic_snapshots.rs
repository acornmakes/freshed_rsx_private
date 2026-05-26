use freshed_rs_macros::html;

#[test]
fn snapshot_intrinsic_simple_element() {
    let mut rendered = String::new();
    html!(&mut rendered, <div>divided</div>).expect("render should succeed");
    insta::assert_snapshot!(rendered, @"<div>divided</div>");
}

#[test]
fn snapshot_intrinsic_expression_attribute() {
    let id = 123;
    let mut rendered = String::new();
    html!(&mut rendered, <div data-id={id}>ok</div>).expect("render should succeed");
    insta::assert_snapshot!(rendered, @"<div data-id=\"123\">ok</div>");
}

#[test]
fn snapshot_intrinsic_nested_markup() {
    let mut rendered = String::new();
    html!(&mut rendered, <section><h1>Title</h1><p>Body</p></section>)
        .expect("render should succeed");
    insta::assert_snapshot!(rendered, @"<section><h1>Title</h1><p>Body</p></section>");
}
