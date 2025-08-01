use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct InvalidStruct {
    // This should be an error: required fields cannot skip setters
    #[builder(required, skip_setter)]
    name: String,

    email: Option<String>,
}

fn main() {}
