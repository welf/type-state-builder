use type_state_builder::TypeStateBuilder;

#[test]
fn test_only_const_generic() {
    #[derive(TypeStateBuilder)]
    struct Simple<const N: usize> {
        optional_data: i32,
    }

    let instance = Simple::<3>::builder().build();
    assert_eq!(instance.optional_data, 0);
}
