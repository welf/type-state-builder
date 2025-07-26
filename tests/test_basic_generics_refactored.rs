// Test generic type handling with complex field types

use type_state_builder::TypeStateBuilder;

#[derive(Debug, Clone, PartialEq)]
struct MyData<T> {
    value: T,
}

#[derive(TypeStateBuilder)]
struct BasicGeneric<T>
where
    T: Clone + std::fmt::Debug,
{
    #[builder(required)]
    name: String,

    #[builder(required)]
    data: MyData<T>,

    optional_field: Option<i32>,
}

#[test]
fn test_basic_generic_success() {
    let instance = BasicGeneric::<String>::builder()
        .name("test".to_string())
        .data(MyData {
            value: "hello".to_string(),
        })
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.data.value, "hello");
    assert_eq!(instance.optional_field, None);
}

#[test]
fn test_basic_generic_with_optional() {
    let instance = BasicGeneric::<i32>::builder()
        .name("test".to_string())
        .data(MyData { value: 42 })
        .optional_field(Some(99))
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.data.value, 42);
    assert_eq!(instance.optional_field, Some(99));
}
