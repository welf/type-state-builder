// Test PhantomData integration with simple cases

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleNoPhantomData {
    #[builder(required)]
    name: String,

    optional_value: u32,
}

#[test]
fn test_simple_no_phantom_data() {
    let instance = SimpleNoPhantomData::builder()
        .name("John".to_string())
        .optional_value(42)
        .build();

    assert_eq!(instance.name, "John");
    assert_eq!(instance.optional_value, 42);
}

#[derive(TypeStateBuilder)]
struct SimpleWithLifetime<'a> {
    #[builder(required)]
    data: &'a str,

    optional_value: u32,
}

#[test]
fn test_simple_with_lifetime() {
    let text = "hello world";
    let instance = SimpleWithLifetime::builder()
        .data(text)
        .optional_value(42)
        .build();

    assert_eq!(instance.data, "hello world");
    assert_eq!(instance.optional_value, 42);
}

#[derive(TypeStateBuilder)]
struct SimpleWithGeneric<T> {
    #[builder(required)]
    value: T,
}

#[test]
fn test_simple_with_generic() {
    let instance = SimpleWithGeneric::<String>::builder()
        .value("hello".to_string())
        .build();

    assert_eq!(instance.value, "hello");
}
