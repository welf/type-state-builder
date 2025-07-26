// Progressive test to isolate the associated types issue

use type_state_builder::TypeStateBuilder;

trait Container {
    type Item<T>;
}

struct VecContainer;
impl Container for VecContainer {
    type Item<T> = Vec<T>;
}

// Test 1: Does the struct compile without derive?
struct TestWithoutDerive<C>
where
    C: Container,
{
    value: <C as Container>::Item<i32>,
}

// Test 2: Does it work with required field with associated type?
#[derive(TypeStateBuilder)]
struct TestWithRequired<C: Container> {
    #[builder(required)]
    value: <C as Container>::Item<i32>,
}

#[test]
fn test_without_derive() {
    let instance = TestWithoutDerive::<VecContainer> {
        value: vec![1, 2, 3],
    };

    assert_eq!(instance.value, vec![1, 2, 3]);
}

#[test]
fn test_with_required_assoc_type() {
    let instance = TestWithRequired::<VecContainer>::builder()
        .value(vec![1, 2, 3])
        .build();
    assert_eq!(instance.value, vec![1, 2, 3]);
}
