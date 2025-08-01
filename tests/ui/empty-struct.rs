use type_state_builder::TypeStateBuilder;

// This should be an error: empty structs are not supported
#[derive(TypeStateBuilder)]
struct EmptyStruct {}

fn main() {}
