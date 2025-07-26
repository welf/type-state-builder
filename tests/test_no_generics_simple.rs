// Verify that non-generic structs work to isolate the issue

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct SimpleNoGenerics {
    #[builder(required)]
    name: String,
}

#[test]
fn test_no_generics() {
    let instance = SimpleNoGenerics::builder().name("test".to_string()).build();

    assert_eq!(instance.name, "test");
}
