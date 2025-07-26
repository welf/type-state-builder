use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Simple {
    #[builder(required)]
    name: String,

    optional: i32,
}

#[test]
fn test_working_builder() {
    let instance = Simple::builder().name("Alice".to_string()).build();

    assert_eq!(instance.name, "Alice");
    assert_eq!(instance.optional, 0);
}

#[test]
fn test_working_builder_with_optional() {
    let instance = Simple::builder()
        .name("Bob".to_string())
        .optional(42)
        .build();

    assert_eq!(instance.name, "Bob");
    assert_eq!(instance.optional, 42);
}
