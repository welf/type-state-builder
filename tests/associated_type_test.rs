use type_state_builder::TypeStateBuilder;

trait AssocTrait {
    type Output;
}

struct AssocImpl;
impl AssocTrait for AssocImpl {
    type Output = Vec<i32>;
}

#[derive(TypeStateBuilder)]
struct WithAssoc<T: AssocTrait> {
    #[builder(required)]
    value: T::Output,
}

#[test]
fn test_associated_type() {
    let instance = WithAssoc::<AssocImpl>::builder()
        .value(vec![1, 2, 3])
        .build();

    assert_eq!(instance.value, vec![1, 2, 3]);
}
