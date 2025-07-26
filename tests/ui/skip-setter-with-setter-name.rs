use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct InvalidStruct {
    name: String,
    
    // This should be an error: can't have custom setter name on skipped setters
    #[builder(default = "42", skip_setter, setter_name = "custom_name")]
    count: u32,
}

fn main() {
    let _instance = InvalidStruct::builder()
        .name("test".to_string())
        .build();
}