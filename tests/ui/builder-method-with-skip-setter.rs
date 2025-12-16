use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Invalid {
    #[builder(required, builder_method, skip_setter)]
    id: u64,
}

fn main() {}
