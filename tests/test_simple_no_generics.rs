// Test the PhantomData architecture with no generics

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleNoGenerics {
    #[builder(required)]
    name: String,
}

#[test]
fn test_simple_no_generics() {
    let instance = SimpleNoGenerics::builder()
        .name("hello".to_string())
        .build();

    assert_eq!(instance.name, "hello");
}
