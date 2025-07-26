use type_state_builder::TypeStateBuilder;

trait SimpleContainer {
    type Output<T>;
}

struct VecContainer;
impl SimpleContainer for VecContainer {
    type Output<T> = Vec<T>;
}

#[derive(TypeStateBuilder)]
struct GenericStruct<C: SimpleContainer> {
    #[builder(required)]
    name: String,

    #[builder(required)]
    data: <C as SimpleContainer>::Output<i32>,
}

#[test]
fn test_generic_with_associated_types() {
    let instance = GenericStruct::<VecContainer>::builder()
        .name("test".to_string())
        .data(vec![1, 2, 3])
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.data, vec![1, 2, 3]);
}
