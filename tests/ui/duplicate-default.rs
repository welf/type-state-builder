use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct DuplicateDefault {
    name: String,

    // This should be an error: duplicate default attributes
    #[builder(default = "42")]
    #[builder(default = "100")]
    count: u32,
}

fn main() {}
