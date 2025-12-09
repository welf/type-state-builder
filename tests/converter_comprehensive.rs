//! Comprehensive tests for converter functionality
//!
//! This module tests the converter attribute with various complex scenarios including:
//! - Basic converter functionality
//! - Generics, lifetimes, and const generics
//! - Complex type transformations
//! - Error conditions and validation

use type_state_builder::TypeStateBuilder;

// Basic converter functionality
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct BasicConverter {
    #[builder(required, converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
    tags: Vec<String>,

    #[builder(converter = |value: &str| value.to_uppercase())]
    name: String,

    #[builder(converter = |value: i32| value * 2)]
    count: i32,
}

#[test]
fn test_basic_converter() {
    let instance = BasicConverter::builder()
        .tags(vec!["rust", "converter"])
        .name("test")
        .count(21)
        .build();

    assert_eq!(
        instance.tags,
        vec!["rust".to_string(), "converter".to_string()]
    );
    assert_eq!(instance.name, "TEST".to_string());
    assert_eq!(instance.count, 42);
}

// Converter with generics
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct GenericConverter<T>
where
    T: Clone + std::fmt::Debug,
{
    #[builder(required)]
    data: T,

    #[builder(converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
    labels: Vec<String>,
}

#[test]
fn test_generic_converter() {
    let instance = GenericConverter::<i32>::builder()
        .data(42)
        .labels(vec!["label1", "label2"])
        .build();

    assert_eq!(instance.data, 42);
    assert_eq!(
        instance.labels,
        vec!["label1".to_string(), "label2".to_string()]
    );
}

// Converter with lifetimes
#[derive(TypeStateBuilder, Debug)]
struct LifetimeConverter<'a> {
    #[builder(required)]
    data: &'a str,

    #[builder(converter = |value: Vec<&str>| value.join(","))]
    combined: String,
}

#[test]
fn test_lifetime_converter() {
    let data = "test data";
    let instance = LifetimeConverter::builder()
        .data(data)
        .combined(vec!["a", "b", "c"])
        .build();

    assert_eq!(instance.data, "test data");
    assert_eq!(instance.combined, "a,b,c");
}

// Converter with const generics
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct ConstGenericConverter<const N: usize> {
    #[builder(required)]
    data: [i32; N],

    #[builder(converter = |value: Vec<i32>| value.into_iter().sum())]
    sum: i32,
}

#[test]
fn test_const_generic_converter() {
    let instance = ConstGenericConverter::<3>::builder()
        .data([1, 2, 3])
        .sum(vec![10, 20, 30])
        .build();

    assert_eq!(instance.data, [1, 2, 3]);
    assert_eq!(instance.sum, 60);
}

// Complex converter with multiple generic parameters
#[derive(TypeStateBuilder, Debug)]
struct ComplexConverter<'a, T, U, const N: usize>
where
    T: Clone + std::fmt::Debug,
    U: std::fmt::Display,
{
    #[builder(required)]
    reference: &'a str,

    #[builder(required)]
    data: T,

    #[builder(required)]
    display_data: U,

    #[builder(required)]
    array: [i32; N],

    #[builder(converter = |value: Vec<&str>| value.into_iter().map(|s| format!("converted_{s}")).collect())]
    processed: Vec<String>,
}

#[test]
fn test_complex_converter() {
    let reference = "test_ref";
    let instance = ComplexConverter::<String, f64, 2>::builder()
        .reference(reference)
        .data("test".to_string())
        .display_data(std::f64::consts::PI)
        .array([1, 2])
        .processed(vec!["item1", "item2"])
        .build();

    assert_eq!(instance.reference, "test_ref");
    assert_eq!(instance.data, "test".to_string());
    assert_eq!(instance.display_data, std::f64::consts::PI);
    assert_eq!(instance.array, [1, 2]);
    assert_eq!(
        instance.processed,
        vec!["converted_item1".to_string(), "converted_item2".to_string()]
    );
}

// Converter with complex closure expressions
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct ComplexClosureConverter {
    #[builder(converter = |value: Vec<(String, i32)>| {
        value.into_iter()
            .filter(|(_, count)| *count > 0)
            .map(|(name, count)| format!("{name}:{count}"))
            .collect()
    })]
    items: Vec<String>,

    #[builder(converter = |value: std::collections::HashMap<String, i32>| {
        value.values().sum()
    })]
    total: i32,
}

#[test]
fn test_complex_closure_converter() {
    let mut map = std::collections::HashMap::new();
    map.insert("a".to_string(), 10);
    map.insert("b".to_string(), 20);
    map.insert("c".to_string(), 30);

    let instance = ComplexClosureConverter::builder()
        .items(vec![
            ("valid".to_string(), 5),
            ("invalid".to_string(), 0),
            ("another".to_string(), 3),
        ])
        .total(map)
        .build();

    assert_eq!(
        instance.items,
        vec!["valid:5".to_string(), "another:3".to_string()]
    );
    assert_eq!(instance.total, 60);
}

// Converter with other attributes
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(setter_prefix = "with_")]
struct ConverterWithAttributes {
    #[builder(required, converter = |value: &str| value.to_uppercase(), setter_name = "title")]
    name: String,

    #[builder(converter = |value: Vec<i32>| value.into_iter().max().unwrap_or(0), setter_prefix = "set_")]
    max_value: i32,

    #[builder(converter = |value: Option<String>| value.unwrap_or_else(|| "default".to_string()), default = String::new())]
    description: String,
}

#[test]
fn test_converter_with_attributes() {
    let instance = ConverterWithAttributes::builder()
        .with_title("hello world")
        .set_max_value(vec![1, 5, 3, 9, 2])
        .with_description(Some("custom desc".to_string()))
        .build();

    assert_eq!(instance.name, "HELLO WORLD".to_string());
    assert_eq!(instance.max_value, 9);
    assert_eq!(instance.description, "custom desc".to_string());
}

#[test]
fn test_converter_with_default_fallback() {
    let instance = ConverterWithAttributes::builder()
        .with_title("test")
        .build();

    assert_eq!(instance.name, "TEST".to_string());
    assert_eq!(instance.max_value, 0); // Default from converter when not set
    assert_eq!(instance.description, "".to_string()); // Custom default
}

// Regular builder with converters
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct RegularBuilderConverter {
    #[builder(converter = |value: Vec<&str>| value.join("-"))]
    joined: String,

    #[builder(converter = |value: (i32, i32)| value.0 + value.1)]
    sum: i32,
}

#[test]
fn test_regular_builder_converter() {
    let instance = RegularBuilderConverter::builder()
        .joined(vec!["a", "b", "c"])
        .sum((10, 20))
        .build();

    assert_eq!(instance.joined, "a-b-c".to_string());
    assert_eq!(instance.sum, 30);
}

#[test]
fn test_regular_builder_converter_defaults() {
    let instance = RegularBuilderConverter::builder()
        .joined(vec!["only", "this"])
        .build();

    assert_eq!(instance.joined, "only-this".to_string());
    assert_eq!(instance.sum, 0); // Default::default()
}
