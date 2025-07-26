use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct InvalidStruct {
    name: String,
    
    // This should be an error: can't have setter prefix on skipped setters
    #[builder(default = "42", skip_setter, setter_prefix = "with_")]
    count: u32,
}

fn main() {
    let _instance = InvalidStruct::builder()
        .name("test".to_string())
        .build();
}