// Test basic generic type handling

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleGeneric<T> {
    #[builder(required)]
    value: T,
}

#[test]
fn test_simple_generic() {
    let instance = SimpleGeneric::<String>::builder()
        .value("hello".to_string())
        .build();

    assert_eq!(instance.value, "hello");
}
