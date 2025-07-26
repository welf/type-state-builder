use type_state_builder::TypeStateBuilder;

#[test]
fn test_simple_const_generic() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct SimpleConstGeneric<const N: usize> {
        #[builder(required)]
        data: [i32; N],
    }

    let instance = SimpleConstGeneric::<3>::builder().data([1, 2, 3]).build();

    assert_eq!(instance.data, [1, 2, 3]);
}
