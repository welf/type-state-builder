use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct TestGeneric<T> {
    #[builder(required)]
    name: String,

    #[builder(required)]
    data: T,
}

#[test]
fn test_generic_with_state_markers() {
    let instance = TestGeneric::<i32>::builder()
        .name("test".to_string())
        .data(42i32)
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.data, 42);
}
