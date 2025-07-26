// Test multiple required fields only

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct MultipleRequired {
    #[builder(required)]
    name: String,

    #[builder(required)]
    age: u32,
}

#[test]
fn test_multiple_required() {
    let instance = MultipleRequired::builder()
        .name("Alice".to_string())
        .age(30)
        .build();

    assert_eq!(instance.name, "Alice");
    assert_eq!(instance.age, 30);
}
