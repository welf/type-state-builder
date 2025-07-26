//! Debug test for generic associated types

use type_state_builder::TypeStateBuilder;

trait Container {
    type Item<T>;
    #[allow(dead_code)]
    fn create<T>(item: T) -> Self::Item<T>;
}

struct VecContainer;
impl Container for VecContainer {
    type Item<T> = Vec<T>;
    fn create<T>(item: T) -> Self::Item<T> {
        vec![item]
    }
}

#[derive(TypeStateBuilder)]
struct TestStruct<C: Container> {
    #[builder(required)]
    name: String,

    #[builder(required)]
    items: <C as Container>::Item<i32>,
}

#[test]
fn test_simple_generic_with_associated_type() {
    let instance = TestStruct::<VecContainer>::builder()
        .name("test".to_string())
        .items(vec![1, 2, 3])
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.items, vec![1, 2, 3]);
}
