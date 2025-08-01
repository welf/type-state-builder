//! Simple test for custom converter closure feature

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug, PartialEq)]
struct SimpleTest {
    #[builder(required, converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
    tags: Vec<String>,

    #[builder(required)]
    name: String,
}

#[test]
fn test_simple_custom_setter() {
    let instance = SimpleTest::builder()
        .tags(vec!["tag1", "tag2"])
        .name("test".to_string())
        .build();

    assert_eq!(instance.tags, vec!["tag1".to_string(), "tag2".to_string()]);
    assert_eq!(instance.name, "test".to_string());
}
