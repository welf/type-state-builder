//! Debug test without derive to see if struct is valid

trait Container {
    type Item<T>;
}

struct VecContainer;
impl Container for VecContainer {
    type Item<T> = Vec<T>;
}

struct TestStruct<C>
where
    C: Container,
{
    name: String,
    items: <C as Container>::Item<i32>,
}

#[test]
fn test_struct_compiles_without_derive() {
    let test: TestStruct<VecContainer> = TestStruct {
        name: "test".to_string(),
        items: vec![1, 2, 3],
    };

    // Assert that fields are properly set
    assert_eq!(test.name, "test");
    assert_eq!(test.items, vec![1, 2, 3]);
}
