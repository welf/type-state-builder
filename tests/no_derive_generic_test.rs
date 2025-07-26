trait SimpleContainer {
    type Output<T>;
}

struct VecContainer;
impl SimpleContainer for VecContainer {
    type Output<T> = Vec<T>;
}

// Test without derive to see if the struct definition itself works
struct GenericStruct<C>
where
    C: SimpleContainer,
{
    name: String,
    data: <C as SimpleContainer>::Output<i32>,
}

#[test]
fn test_generic_without_derive() {
    let instance = GenericStruct::<VecContainer> {
        name: "test".to_string(),
        data: vec![1, 2, 3],
    };

    assert_eq!(instance.name, "test");
    assert_eq!(instance.data, vec![1, 2, 3]);
}
