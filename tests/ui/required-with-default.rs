use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct InvalidStruct {
    // This should be an error: required fields cannot have default values
    #[builder(required, default = String::from("default"))]
    name: String,

    email: Option<String>,
}

fn main() {}
