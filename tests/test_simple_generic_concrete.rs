// Test concrete builder types with simple generics

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleGenericConcrete<T> {
    #[builder(required)]
    value: T,
}

#[test]
fn test_simple_generic_concrete() {
    let instance = SimpleGenericConcrete::<String>::builder()
        .value("hello".to_string())
        .build();

    assert_eq!(instance.value, "hello");
}
