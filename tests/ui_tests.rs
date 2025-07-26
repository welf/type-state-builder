/// UI tests for TypeStateBuilder macro
///
/// These tests verify that the macro properly rejects invalid code with appropriate
/// compile-time error messages. They use the `trybuild` crate to test compilation
/// failures and ensure error messages are helpful and accurate.

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
