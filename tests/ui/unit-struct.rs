use type_state_builder::TypeStateBuilder;

// This should be an error: unit structs are not supported
#[derive(TypeStateBuilder)]
struct UnitStruct;

fn main() {}
