// Verify that the generic parameter fix works correctly

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct FixedGeneric<T> {
    #[builder(required)]
    value: T,
}

#[test]
fn test_fixed_generic() {
    let instance = FixedGeneric::<String>::builder()
        .value("hello".to_string())
        .build();

    assert_eq!(instance.value, "hello");
}
