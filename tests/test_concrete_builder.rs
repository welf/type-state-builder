use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct TestStruct {
    #[builder(required)]
    name: String,

    #[builder(required)]
    age: u32,

    optional_field: Option<String>,
}

#[test]
fn test_concrete_builder_approach() {
    // Test the concrete builder approach
    let instance = TestStruct::builder()
        .name("John".to_string())
        .age(30)
        .optional_field(Some("test".to_string()))
        .build();

    assert_eq!(instance.name, "John");
    assert_eq!(instance.age, 30);
    assert_eq!(instance.optional_field, Some("test".to_string()));
}

#[test]
fn test_concrete_builder_state_transitions() {
    // Test that state transitions work correctly
    let builder = TestStruct::builder();

    // Set name first
    let builder_with_name = builder.name("Alice".to_string());

    // Set age second
    let builder_with_name_and_age = builder_with_name.age(25);

    // Now we can build
    let instance = builder_with_name_and_age.build();

    assert_eq!(instance.name, "Alice");
    assert_eq!(instance.age, 25);
    assert_eq!(instance.optional_field, None);
}
