use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleGeneric<T> {
    #[builder(required)]
    value: T,
}

#[test]
fn test_simple_generic() {
    let instance = SimpleGeneric::<i32>::builder().value(42).build();

    assert_eq!(instance.value, 42);
}
