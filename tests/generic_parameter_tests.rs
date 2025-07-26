//! Tests for generic parameter handling in TypeStateBuilder
//!
//! This test file verifies that the TypeStateBuilder derive macro correctly handles
//! various generic parameter scenarios including bounds, constraints, and complex types.

use std::clone::Clone;
use std::fmt::Debug;
use type_state_builder::TypeStateBuilder;

#[test]
fn test_simple_generic_struct() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct SimpleGeneric<T> {
        #[builder(required)]
        value: T,

        optional: Option<String>,
    }

    let instance = SimpleGeneric::<i32>::builder().value(42).build();

    assert_eq!(instance.value, 42);
    assert_eq!(instance.optional, None);
}

#[test]
fn test_generic_with_debug_bound() {
    #[derive(TypeStateBuilder)]
    struct GenericWithDebug<T: Debug> {
        #[builder(required)]
        data: T,

        #[builder(required)]
        name: String,

        optional_field: Option<i32>,
    }

    let instance = GenericWithDebug::<String>::builder()
        .data("test".to_string())
        .name("example".to_string())
        .build();

    assert_eq!(instance.data, "test");
    assert_eq!(instance.name, "example");
    assert_eq!(instance.optional_field, None);
}

#[test]
fn test_multiple_generic_parameters() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct MultipleGenerics<T, U> {
        #[builder(required)]
        first: T,

        #[builder(required)]
        second: U,

        optional: Option<String>,
    }

    let instance = MultipleGenerics::<i32, String>::builder()
        .first(42)
        .second("hello".to_string())
        .build();

    assert_eq!(instance.first, 42);
    assert_eq!(instance.second, "hello");
    assert_eq!(instance.optional, None);
}

#[test]
fn test_generic_with_multiple_bounds() {
    #[derive(TypeStateBuilder)]
    struct GenericWithMultipleBounds<T: Debug + Clone> {
        #[builder(required)]
        value: T,

        count: Option<usize>,
    }

    let instance = GenericWithMultipleBounds::<String>::builder()
        .value("test".to_string())
        .count(Some(5))
        .build();

    assert_eq!(instance.value, "test");
    assert_eq!(instance.count, Some(5));
}

#[test]
fn test_generic_only_in_optional_fields() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct GenericOptionalOnly<T> {
        // Only optional fields - generic only used in optional field
        name: Option<String>,
        optional_value: Option<T>,
    }

    let instance = GenericOptionalOnly::<i32>::builder()
        .name(Some("test".to_string()))
        .optional_value(Some(42))
        .build();

    assert_eq!(instance.name, Some("test".to_string()));
    assert_eq!(instance.optional_value, Some(42));
}

#[test]
fn test_nested_generic_types() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct NestedGenerics<T> {
        #[builder(required)]
        data: Vec<T>,

        #[builder(required)]
        lookup: std::collections::HashMap<String, T>,

        #[builder(default = "\"default_value\".to_string()")]
        optional: String,
    }

    let mut lookup = std::collections::HashMap::new();
    lookup.insert("key".to_string(), 100);

    let instance = NestedGenerics::<i32>::builder()
        .data(vec![1, 2, 3])
        .lookup(lookup.clone())
        .build();

    assert_eq!(instance.data, vec![1, 2, 3]);
    assert_eq!(instance.lookup, lookup);
    assert_eq!(instance.optional, "default_value".to_string()); // Test custom default worked
}

#[test]
fn test_generic_with_where_clause() {
    #[derive(TypeStateBuilder)]
    struct GenericWithWhere<T>
    where
        T: Debug + Clone,
    {
        #[builder(required)]
        value: T,

        description: Option<String>,
    }

    let instance = GenericWithWhere::<String>::builder()
        .value("test".to_string())
        .description(Some("example".to_string()))
        .build();

    assert_eq!(instance.value, "test");
    assert_eq!(instance.description, Some("example".to_string()));
}

#[test]
fn test_mixed_required_optional_generics() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct MixedGenerics<T> {
        #[builder(required)]
        required_t: T,

        #[builder(required)]
        required_string: String,

        #[builder(default = "42.0")]
        optional_number: f64,
        optional_option: Option<i32>,
    }

    // Test without setting optional_number to verify custom default
    let instance = MixedGenerics::<String>::builder()
        .required_t("hello".to_string())
        .required_string("world".to_string())
        .build();

    assert_eq!(instance.required_t, "hello");
    assert_eq!(instance.required_string, "world");
    assert_eq!(instance.optional_number, 42.0); // Test custom default worked
    assert_eq!(instance.optional_option, None);
}

#[test]
fn test_const_generic_parameters() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ConstGeneric<T, const N: usize> {
        #[builder(required)]
        data: [T; N],

        #[builder(required)]
        name: String,

        optional: Option<usize>,
    }

    let instance = ConstGeneric::<i32, 3>::builder()
        .data([1, 2, 3])
        .name("array".to_string())
        .build();

    assert_eq!(instance.data, [1, 2, 3]);
    assert_eq!(instance.name, "array");
    assert_eq!(instance.optional, None);
}
