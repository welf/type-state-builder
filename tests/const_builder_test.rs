//! Integration tests for const builder functionality
//!
//! These tests verify that `#[builder(const)]` generates compile-time
//! constant builders that can be used in const contexts.

use type_state_builder::TypeStateBuilder;

/// Test basic const builder with required fields
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct ConstConfig {
    #[builder(required)]
    name: &'static str,
    #[builder(required)]
    value: i32,
}

#[test]
fn test_const_builder_basic() {
    // Build at compile time
    const CONFIG: ConstConfig = ConstConfig::builder().name("test").value(42).build();

    assert_eq!(CONFIG.name, "test");
    assert_eq!(CONFIG.value, 42);
}

/// Test const builder with optional fields and explicit defaults
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct ConstWithDefaults {
    #[builder(required)]
    id: u32,
    #[builder(default = 100)]
    count: i32,
    #[builder(default = "default")]
    label: &'static str,
}

#[test]
fn test_const_builder_with_defaults() {
    // Use defaults at compile time
    const DEFAULT_CONFIG: ConstWithDefaults = ConstWithDefaults::builder().id(1).build();

    assert_eq!(DEFAULT_CONFIG.id, 1);
    assert_eq!(DEFAULT_CONFIG.count, 100);
    assert_eq!(DEFAULT_CONFIG.label, "default");

    // Override defaults at compile time
    const CUSTOM_CONFIG: ConstWithDefaults = ConstWithDefaults::builder()
        .id(2)
        .count(200)
        .label("custom")
        .build();

    assert_eq!(CUSTOM_CONFIG.id, 2);
    assert_eq!(CUSTOM_CONFIG.count, 200);
    assert_eq!(CUSTOM_CONFIG.label, "custom");
}

/// Test const builder with converter on required field
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct ConstWithRequiredConverter {
    #[builder(required, converter = |s: &'static str| s.len())]
    length: usize,
}

#[test]
fn test_const_builder_with_required_converter() {
    const CONVERTED: ConstWithRequiredConverter = ConstWithRequiredConverter::builder()
        .length("hello")
        .build();

    assert_eq!(CONVERTED.length, 5); // "hello".len()
}

/// Test const builder with converter on optional field
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct ConstWithOptionalConverter {
    #[builder(required)]
    id: i32,
    #[builder(default = 0, converter = |n: i32| n * 2)]
    doubled: i32,
}

#[test]
fn test_const_builder_with_optional_converter() {
    // Converter works at compile time for optional fields
    const CONVERTED: ConstWithOptionalConverter = ConstWithOptionalConverter::builder()
        .id(1)
        .doubled(21)
        .build();

    assert_eq!(CONVERTED.id, 1);
    assert_eq!(CONVERTED.doubled, 42); // 21 * 2
}

/// Test const builder with only optional fields (regular builder pattern)
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct ConstAllOptional {
    #[builder(default = 0)]
    a: i32,
    #[builder(default = 1)]
    b: i32,
    #[builder(default = 2)]
    c: i32,
}

#[test]
fn test_const_builder_all_optional() {
    const ALL_DEFAULTS: ConstAllOptional = ConstAllOptional::builder().build();
    assert_eq!(ALL_DEFAULTS.a, 0);
    assert_eq!(ALL_DEFAULTS.b, 1);
    assert_eq!(ALL_DEFAULTS.c, 2);

    const PARTIAL: ConstAllOptional = ConstAllOptional::builder().b(10).build();
    assert_eq!(PARTIAL.a, 0);
    assert_eq!(PARTIAL.b, 10);
    assert_eq!(PARTIAL.c, 2);
}

/// Test const builder with custom build method name
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const, build_method = "create")]
struct ConstCustomBuild {
    #[builder(required)]
    value: i32,
}

#[test]
fn test_const_builder_custom_build_method() {
    const CREATED: ConstCustomBuild = ConstCustomBuild::builder().value(123).create();

    assert_eq!(CREATED.value, 123);
}

/// Test const builder with setter prefix
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const, setter_prefix = "with_")]
struct ConstWithPrefix {
    #[builder(required)]
    name: &'static str,
    #[builder(default = 0)]
    age: u8,
}

#[test]
fn test_const_builder_with_setter_prefix() {
    const PREFIXED: ConstWithPrefix = ConstWithPrefix::builder()
        .with_name("Alice")
        .with_age(30)
        .build();

    assert_eq!(PREFIXED.name, "Alice");
    assert_eq!(PREFIXED.age, 30);
}

/// Verify const builders can be used in static contexts
static STATIC_CONFIG: ConstConfig = ConstConfig::builder().name("static").value(999).build();

#[test]
fn test_const_builder_in_static() {
    assert_eq!(STATIC_CONFIG.name, "static");
    assert_eq!(STATIC_CONFIG.value, 999);
}

/// Test that const builders work in const fn
const fn create_config(name: &'static str, value: i32) -> ConstConfig {
    ConstConfig::builder().name(name).value(value).build()
}

#[test]
fn test_const_builder_in_const_fn() {
    const FROM_FN: ConstConfig = create_config("from_fn", 777);
    assert_eq!(FROM_FN.name, "from_fn");
    assert_eq!(FROM_FN.value, 777);
}
