#[test]
fn ui_compile_harness() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui_stream/pass/*.rs");
    t.compile_fail("tests/ui_stream/fail/*.rs");
}
