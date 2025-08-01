use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct TestStruct {
    #[builder(converter = |value: String| value.to_uppercase(), converter = |value: String| value.to_lowercase())]
    name: String,
}

