use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Invalid {
    #[builder(required, builder_method)]
    id: u64,
    #[builder(required, builder_method)]
    name: String,
}

fn main() {}
