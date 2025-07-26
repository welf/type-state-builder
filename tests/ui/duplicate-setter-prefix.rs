use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct DuplicateSetterPrefix {
    // This should be an error: duplicate setter_prefix attributes
    #[builder(setter_prefix = "with_")]
    #[builder(setter_prefix = "set_")]
    name: String,

    email: String,
}

fn main() {
    let _instance = DuplicateSetterPrefix::builder()
        .with_name("test".to_string())
        .email("test@example.com".to_string())
        .build();
}