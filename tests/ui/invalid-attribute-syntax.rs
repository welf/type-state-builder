use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct InvalidAttribute {
    // This should be an error: invalid attribute syntax
    #[builder(invalid_attribute)]
    name: String,

    email: String,
}

fn main() {}
