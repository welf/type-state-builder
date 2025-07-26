use std::fmt::Debug;
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Example<'a, T: Debug> {
    #[builder(required)]
    name: String,

    #[builder(required)]
    data: &'a T,

    optional_field: Option<String>,
}

#[test]
fn test_example_with_lifetime_and_bounds() {
    let value = 42i32;
    let instance = Example::<i32>::builder()
        .name("test".to_string())
        .data(&value)
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.data, &42);
    assert_eq!(instance.optional_field, None);
}
