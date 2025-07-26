//! Minimal test to isolate generic parameter issue

use type_state_builder::TypeStateBuilder;

trait Container {
    type Item<T>;
}

struct VecContainer;
impl Container for VecContainer {
    type Item<T> = Vec<T>;
}

#[derive(TypeStateBuilder)]
struct Test<C>
where
    C: Container,
{
    #[builder(required)]
    name: String,

    #[builder(skip_setter, default = "std::marker::PhantomData")]
    _phantom: std::marker::PhantomData<C>,
}

#[test]
fn test_minimal_generic() {
    let _instance = Test::<VecContainer>::builder()
        .name("test".to_string())
        .build();

    assert_eq!(_instance.name, "test");
}
