// Test concrete builder types with lifetimes

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleLifetimeConcrete<'a> {
    #[builder(required)]
    data: &'a str,

    optional_value: u32,
}

#[test]
fn test_simple_lifetime_concrete() {
    let text = "hello world";
    let instance = SimpleLifetimeConcrete::builder()
        .data(text)
        .optional_value(42)
        .build();

    assert_eq!(instance.data, "hello world");
    assert_eq!(instance.optional_value, 42);
}
