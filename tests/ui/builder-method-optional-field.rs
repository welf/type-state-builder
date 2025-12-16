use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Invalid {
    #[builder(builder_method)]
    name: Option<String>,
}

fn main() {}
