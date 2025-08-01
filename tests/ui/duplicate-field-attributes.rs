use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct DuplicateAttributes {
    // This should be an error: duplicate attributes
    #[builder(setter_name = "full_name")]
    #[builder(setter_name = "first_name")]
    name: String,

    email: String,
}

fn main() {}
