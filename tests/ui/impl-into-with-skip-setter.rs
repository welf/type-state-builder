// Test that impl_into and skip_setter attributes conflict

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct ConflictingFieldAttributes {
    #[builder(impl_into, skip_setter, default = "String::new()")]
    name: String,

    age: Option<u32>,
}

fn main() {}
