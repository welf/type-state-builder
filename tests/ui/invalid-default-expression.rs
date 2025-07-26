use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct InvalidDefault {
    name: String,
    
    // This should be an error: invalid default expression
    #[builder(default = "this_function_does_not_exist()")]
    count: u32,
}

fn main() {
    let _instance = InvalidDefault::builder()
        .name("test".to_string())
        .build();
}