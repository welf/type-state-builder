//! Tests for elided lifetime handling with split_for_impl
//!
//! This test verifies that the type-state builder correctly handles various
//! lifetime scenarios, including elided lifetimes and explicit lifetime parameters.

use type_state_builder::TypeStateBuilder;

#[test]
fn test_explicit_lifetime_parameter() {
    #[derive(TypeStateBuilder, Debug)]
    struct Container<'a> {
        #[builder(required)]
        data: &'a str,
        something: Option<usize>,
    }

    let text = "hello world";
    let container = Container::builder().data(text).something(Some(42)).build();

    assert_eq!(container.data, "hello world");
    assert_eq!(container.something, Some(42));
}

#[test]
fn test_multiple_lifetime_parameters() {
    #[derive(TypeStateBuilder, Debug)]
    struct MultipleLifetimes<'a, 'b> {
        #[builder(required)]
        text1: &'a str,

        #[builder(required)]
        text2: &'b str,

        count: Option<usize>,
    }

    let text1 = "first";
    let text2 = "second";

    let result = MultipleLifetimes::builder()
        .text1(text1)
        .text2(text2)
        .count(Some(10))
        .build();

    assert_eq!(result.text1, "first");
    assert_eq!(result.text2, "second");
    assert_eq!(result.count, Some(10));
}

#[test]
fn test_lifetime_bounds() {
    #[derive(TypeStateBuilder, Debug)]
    struct WithLifetimeBounds<'a, 'b, T>
    where
        'a: 'b,
        T: 'a,
    {
        #[builder(required)]
        data: &'a T,

        #[builder(required)]
        reference: &'b str,

        optional_data: Option<String>,
    }

    let text = "bounded lifetime";
    let data = String::from("test data");

    let result = WithLifetimeBounds::builder()
        .data(&data)
        .reference(text)
        .optional_data(Some("optional".to_string()))
        .build();

    assert_eq!(result.data, &data);
    assert_eq!(result.reference, "bounded lifetime");
    assert_eq!(result.optional_data, Some("optional".to_string()));
}

#[test]
fn test_elided_lifetime_in_regular_builder() {
    // Test case where struct has only optional fields with lifetimes
    // This should generate a regular builder, not type-state builder
    #[derive(TypeStateBuilder, Debug)]
    struct RegularWithLifetime<'a> {
        data: Option<&'a str>,
        count: Option<usize>,
    }

    let text = "elided";
    let result = RegularWithLifetime::builder()
        .data(Some(text))
        .count(Some(5))
        .build();

    assert_eq!(result.data, Some("elided"));
    assert_eq!(result.count, Some(5));
}

#[test]
fn test_lifetime_with_complex_types() {
    #[derive(TypeStateBuilder, Debug)]
    struct ComplexLifetimes<'a, T: Clone> {
        #[builder(required)]
        slice_data: &'a [T],

        #[builder(required)]
        string_data: &'a str,

        optional_vec: Vec<T>,
    }

    let numbers = [1, 2, 3, 4, 5];
    let text = "complex";

    let result = ComplexLifetimes::builder()
        .slice_data(&numbers)
        .string_data(text)
        .optional_vec(vec![10, 20])
        .build();

    assert_eq!(result.slice_data, &[1, 2, 3, 4, 5]);
    assert_eq!(result.string_data, "complex");
    assert_eq!(result.optional_vec, vec![10, 20]);
}

#[test]
fn test_struct_with_phantom_data_and_lifetimes() {
    use std::marker::PhantomData;

    #[derive(TypeStateBuilder, Debug)]
    struct WithPhantomLifetime<'a, T> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = PhantomData)]
        phantom: PhantomData<&'a T>,
    }

    let result = WithPhantomLifetime::<String>::builder()
        .data("phantom test".to_string())
        .build();

    assert_eq!(result.data, "phantom test");
}

#[test]
fn test_lifetime_elision_compilation() {
    // This tests that our generated code correctly handles lifetime elision
    // when the Rust compiler processes the generated impl blocks

    #[derive(TypeStateBuilder)]
    struct LifetimeElisionTest<'a> {
        #[builder(required)]
        text: &'a str,

        #[builder(required)]
        bytes: &'a [u8],

        optional_string: Option<String>,
    }

    let text = "elision test";
    let bytes = b"test bytes";

    let result = LifetimeElisionTest::builder()
        .text(text)
        .bytes(bytes)
        .optional_string(Some("optional".to_string()))
        .build();

    assert_eq!(result.text, "elision test");
    assert_eq!(result.bytes, b"test bytes");
    assert_eq!(result.optional_string, Some("optional".to_string()));
}

#[test]
fn test_nested_lifetime_references() {
    #[derive(TypeStateBuilder, Debug)]
    struct NestedLifetimes<'a> {
        #[builder(required)]
        data: &'a str,

        #[builder(required)]
        nested_ref: &'a &'a str,

        count: Option<usize>,
    }

    let text = "nested";
    let text_ref = &text;

    let result = NestedLifetimes::builder()
        .data(text)
        .nested_ref(text_ref)
        .count(Some(42))
        .build();

    assert_eq!(result.data, "nested");
    assert_eq!(*result.nested_ref, "nested");
    assert_eq!(result.count, Some(42));
}
