//! Generic Bounds Tests
//!
//! Tests to verify that generic bounds are properly separated into where clauses

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct GenericStruct<T>
where
    T: Clone + std::fmt::Debug + Default,
{
    #[builder(required)]
    name: String,

    optional_data: T,
}

#[test]
fn test_generic_struct_with_bounds() {
    let instance = GenericStruct::builder()
        .name("test".to_string())
        .optional_data(String::from("hello")) // String implements Clone + Debug + Default
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.optional_data, "hello");
}

#[derive(TypeStateBuilder)]
struct OnlyOptionalGeneric<T>
where
    T: Clone + std::fmt::Debug + Default,
{
    field1: T,
    field2: Option<String>,
}

#[test]
fn test_only_optional_generic_with_bounds() {
    let instance = OnlyOptionalGeneric::builder()
        .field1(String::from("hello")) // String implements Clone + Debug + Default
        .field2(Some("test".to_string()))
        .build();

    assert_eq!(instance.field1, "hello");
    assert_eq!(instance.field2, Some("test".to_string()));
}
