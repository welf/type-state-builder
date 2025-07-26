// Test raw identifiers with keywords

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct TestWithKeywords {
    #[builder(required)]
    r#type: String, // 'type' is a keyword

    r#async: Option<bool>, // 'async' is a keyword
}

#[test]
fn test_raw_identifiers() {
    let instance = TestWithKeywords::builder()
        .r#type("test".to_string())
        .r#async(Some(true))
        .build();

    assert_eq!(instance.r#type, "test");
    assert_eq!(instance.r#async, Some(true));
}
