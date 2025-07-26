//! Basic Functionality Tests
//!
//! Tests the core type-state builder pattern functionality with simple structs

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleStruct {
    #[builder(required)]
    name: String,

    #[builder(required)]
    age: u32,

    optional_field: Option<String>,
}

#[test]
fn test_simple_struct_builder() {
    let instance = SimpleStruct::builder()
        .name("Alice".to_string())
        .age(30)
        .build();

    assert_eq!(instance.name, "Alice");
    assert_eq!(instance.age, 30);
    assert_eq!(instance.optional_field, None);
}

#[test]
fn test_simple_struct_builder_with_optional() {
    let instance = SimpleStruct::builder()
        .name("Bob".to_string())
        .age(25)
        .optional_field(Some("test".to_string()))
        .build();

    assert_eq!(instance.name, "Bob");
    assert_eq!(instance.age, 25);
    assert_eq!(instance.optional_field, Some("test".to_string()));
}

#[test]
fn test_method_chaining_order() {
    // Test that required fields can be set in any order
    let instance1 = SimpleStruct::builder()
        .name("Alice".to_string())
        .age(30)
        .build();

    let instance2 = SimpleStruct::builder()
        .age(30)
        .name("Alice".to_string())
        .build();

    assert_eq!(instance1.name, instance2.name);
    assert_eq!(instance1.age, instance2.age);
}

#[derive(TypeStateBuilder)]
struct OnlyOptionalStruct {
    field1: i32,
    field2: String,
}

#[test]
fn test_only_optional_fields() {
    let instance = OnlyOptionalStruct::builder().build();

    assert_eq!(instance.field1, 0); // Default::default()
    assert_eq!(instance.field2, ""); // Default::default()
}

#[test]
fn test_only_optional_fields_with_values() {
    let instance = OnlyOptionalStruct::builder()
        .field1(42)
        .field2("test".to_string())
        .build();

    assert_eq!(instance.field1, 42);
    assert_eq!(instance.field2, "test");
}

#[derive(TypeStateBuilder)]
struct OnlyRequiredStruct {
    #[builder(required)]
    field1: i32,

    #[builder(required)]
    field2: String,
}

#[test]
fn test_only_required_fields() {
    let instance = OnlyRequiredStruct::builder()
        .field1(42)
        .field2("test".to_string())
        .build();

    assert_eq!(instance.field1, 42);
    assert_eq!(instance.field2, "test");
}

#[derive(TypeStateBuilder)]
struct MixedStruct {
    #[builder(required)]
    required1: String,

    optional1: i32,

    #[builder(required)]
    required2: bool,

    optional2: Vec<String>,
}

#[test]
fn test_mixed_required_optional() {
    let instance = MixedStruct::builder()
        .required1("test".to_string())
        .required2(true)
        .build();

    assert_eq!(instance.required1, "test");
    assert!(instance.required2);
    assert_eq!(instance.optional1, 0);
    assert_eq!(instance.optional2, Vec::<String>::new());
}

#[test]
fn test_mixed_with_optional_values() {
    let instance = MixedStruct::builder()
        .required1("test".to_string())
        .optional1(42)
        .required2(false)
        .optional2(vec!["a".to_string(), "b".to_string()])
        .build();

    assert_eq!(instance.required1, "test");
    assert_eq!(instance.optional1, 42);
    assert!(!instance.required2);
    assert_eq!(instance.optional2, vec!["a".to_string(), "b".to_string()]);
}
