use type_state_builder::TypeStateBuilder;

// This should be an error: unions are not supported, only structs
#[derive(TypeStateBuilder)]
union NotAStruct {
    field1: u32,
    field2: f32,
}
fn main() {}
