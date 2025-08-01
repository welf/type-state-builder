use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct DuplicateSetterPrefix {
    // This should be an error: duplicate setter_prefix attributes
    #[builder(setter_prefix = "with_")]
    #[builder(setter_prefix = "set_")]
    name: String,

    email: String,
}

fn main() {}
