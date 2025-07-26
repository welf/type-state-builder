use type_state_builder::TypeStateBuilder;

trait SimpleTrait {}

struct SimpleImpl;
impl SimpleTrait for SimpleImpl {}

#[derive(TypeStateBuilder)]
struct WithWhere<T>
where
    T: SimpleTrait,
{
    #[builder(required)]
    value: String,

    #[builder(skip_setter, default = "std::marker::PhantomData")]
    _phantom: std::marker::PhantomData<T>,
}

#[test]
fn test_where_clause() {
    let instance = WithWhere::<SimpleImpl>::builder()
        .value("test".to_string())
        .build();

    assert_eq!(instance.value, "test");
}
