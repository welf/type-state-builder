use type_state_builder::TypeStateBuilder;

// This should be an error: tuple structs are not supported
#[derive(TypeStateBuilder)]
struct TupleStruct(String, u32);

fn main() {}
