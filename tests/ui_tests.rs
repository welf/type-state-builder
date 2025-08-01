/// UI tests for TypeStateBuilder macro
///
/// These tests verify that the macro properly rejects invalid code with appropriate
/// compile-time error messages. They use the `trybuild` crate to test compilation
/// failures and ensure error messages are helpful and accurate.
///
/// To run UI tests locally: `cargo test --features ui-tests`

#[test]
#[cfg(feature = "ui-tests")]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
