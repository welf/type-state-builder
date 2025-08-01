//! Integration tests for custom converter functionality
//!
//! This module contains comprehensive tests for the converter feature,
//! testing both type-state and regular builders with various closure types.

use type_state_builder::TypeStateBuilder;

// Test type-state builder with required fields and custom converters

#[derive(TypeStateBuilder, Debug, PartialEq)]
struct TypeStateWithCustomConverters {
    #[builder(required, converter = |items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    tags: Vec<String>,

    #[builder(required, converter = |path: &str| std::path::PathBuf::from(path))]
    config_path: std::path::PathBuf,

    #[builder(converter = |seconds: u64| Some(std::time::Duration::from_secs(seconds)))]
    timeout: Option<std::time::Duration>,

    #[builder(converter = |text: &str| Some(text.to_uppercase()))]
    description: Option<String>,
}

#[test]
fn test_type_state_custom_converters() {
    let instance = TypeStateWithCustomConverters::builder()
        .tags(vec!["rust", "testing"]) // Vec<&str> -> Vec<String> via converter
        .config_path("/etc/config") // &str -> PathBuf via converter
        .timeout(30) // u64 -> Some(Duration) via converter
        .description("hello world") // &str -> Some(String) (uppercase) via converter
        .build();

    assert_eq!(
        instance.tags,
        vec!["rust".to_string(), "testing".to_string()]
    );
    assert_eq!(
        instance.config_path,
        std::path::PathBuf::from("/etc/config")
    );
    assert_eq!(instance.timeout, Some(std::time::Duration::from_secs(30)));
    assert_eq!(instance.description, Some("HELLO WORLD".to_string()));
}

#[test]
fn test_type_state_custom_converters_minimal() {
    // Test with only required fields
    let instance = TypeStateWithCustomConverters::builder()
        .tags(vec!["tag1", "tag2"]) // Vec<&str> -> Vec<String>
        .config_path("minimal.conf") // &str -> PathBuf
        .build();

    assert_eq!(instance.tags, vec!["tag1".to_string(), "tag2".to_string()]);
    assert_eq!(
        instance.config_path,
        std::path::PathBuf::from("minimal.conf")
    );
    assert_eq!(instance.timeout, None);
    assert_eq!(instance.description, None);
}

// Test regular builder with all optional fields and custom converters

#[derive(TypeStateBuilder, Debug, PartialEq)]
struct RegularWithCustomConverters {
    #[builder(converter = |items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    items: Vec<String>,

    #[builder(converter = |seconds: u64| std::time::Duration::from_secs(seconds))]
    delay: std::time::Duration,

    #[builder(converter = |input: &str| format!("PARSED: {input}"))]
    name: String,

    #[builder(converter = |n: i32| n * 2)]
    count: i32,
}

#[test]
fn test_regular_custom_converters() {
    let instance = RegularWithCustomConverters::builder()
        .items(vec!["item1", "item2"]) // Vec<&str> -> Vec<String>
        .delay(5) // u64 -> Duration
        .name("test name") // &str -> String (with prefix)
        .count(21) // i32 -> i32 (doubled)
        .build();

    assert_eq!(
        instance.items,
        vec!["item1".to_string(), "item2".to_string()]
    );
    assert_eq!(instance.delay, std::time::Duration::from_secs(5));
    assert_eq!(instance.name, "PARSED: test name".to_string());
    assert_eq!(instance.count, 42);
}

#[test]
fn test_regular_custom_converters_default_fallback() {
    // Test that fields not set use Default::default()
    let instance = RegularWithCustomConverters::builder()
        .name("only name")
        .build();

    assert_eq!(instance.items, Vec::<String>::new());
    assert_eq!(instance.delay, std::time::Duration::default());
    assert_eq!(instance.name, "PARSED: only name".to_string());
    assert_eq!(instance.count, 0);
}

// Test mixed usage with converters and other attributes

#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(setter_prefix = "with_")]
struct MixedCustomConverters {
    #[builder(required)]
    id: u32,

    #[builder(required, converter = |items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect(), setter_name = "categories")]
    tags: Vec<String>,

    #[builder(converter = |value: i32| Some(value), setter_prefix = "set_")]
    value: Option<i32>,

    #[builder(default = "String::from(\"default\")", converter = |text: &str| text.to_uppercase())]
    label: String,
}

#[test]
fn test_mixed_custom_converters_with_attributes() {
    let instance = MixedCustomConverters::builder()
        .with_id(123) // Regular required field with struct prefix
        .with_categories(vec!["cat1", "cat2"]) // Converter with custom setter name (struct prefix applies)
        .set_value(42) // Converter with field-level prefix
        .with_label("custom label") // Converter with struct prefix + default
        .build();

    assert_eq!(instance.id, 123);
    assert_eq!(instance.tags, vec!["cat1".to_string(), "cat2".to_string()]);
    assert_eq!(instance.value, Some(42));
    assert_eq!(instance.label, "CUSTOM LABEL".to_string());
}

#[test]
fn test_mixed_custom_converters_with_defaults() {
    let instance = MixedCustomConverters::builder()
        .with_id(456)
        .with_categories(vec!["single"]) // Single item vector (struct prefix applies)
        .build();

    assert_eq!(instance.id, 456);
    assert_eq!(instance.tags, vec!["single".to_string()]);
    assert_eq!(instance.value, None); // Default None since setter not called
    assert_eq!(instance.label, "default".to_string()); // Uses custom default
}

// Test complex generic scenarios

#[derive(TypeStateBuilder, Debug, PartialEq)]
struct GenericWithCustomConverters<T>
where
    T: Clone + std::fmt::Debug,
{
    #[builder(required)]
    data: T,

    #[builder(converter = |items: Vec<T>| items.into_iter().take(3).collect())]
    limited: Vec<T>,
}

#[test]
fn test_generic_custom_converters() {
    let instance = GenericWithCustomConverters::<String>::builder()
        .data("test".to_string())
        .limited(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ])
        .build();

    assert_eq!(instance.data, "test".to_string());
    assert_eq!(instance.limited.len(), 3); // Only first 3 items
}

// Test optional field with custom converter

#[derive(TypeStateBuilder)]
struct OptionalFieldWithCustomConverter {
    #[builder(converter = |value: i32| Some(value * 10))]
    multiplied: Option<i32>,
}

#[test]
fn test_optional_field_with_custom_converter() {
    let instance = OptionalFieldWithCustomConverter::builder()
        .multiplied(5) // 5 * 10 = 50
        .build();

    assert_eq!(instance.multiplied, Some(50));

    let instance2 = OptionalFieldWithCustomConverter::builder().build();

    assert_eq!(instance2.multiplied, None);
}

// Test edge cases

#[derive(TypeStateBuilder)]
struct EdgeCases {
    #[builder(converter = |input: Vec<String>| input.into_iter().collect::<Vec<_>>())]
    // Identity-like converter
    identity: Vec<String>,
}

#[test]
fn test_edge_cases() {
    let instance = EdgeCases::builder()
        .identity(vec!["test".to_string()])
        .build();

    assert_eq!(instance.identity, vec!["test".to_string()]);
}
