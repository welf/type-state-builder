use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct DuplicateRequired {
    // This should be an error: duplicate required attributes
    #[builder(required)]
    #[builder(required)]
    name: String,

    email: String,
}

fn main() {
    let _instance = DuplicateRequired::builder()
        .name("test".to_string())
        .email("test@example.com".to_string())
        .build();
}