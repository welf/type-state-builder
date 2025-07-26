use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct DuplicateSkipSetter {
    name: String,

    // This should be an error: duplicate skip_setter attributes
    #[builder(skip_setter, default = "42")]
    #[builder(skip_setter)]
    count: u32,
}

fn main() {
    let _instance = DuplicateSkipSetter::builder()
        .name("test".to_string())
        .build();
}