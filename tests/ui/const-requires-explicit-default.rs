use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(const)]
struct InvalidStruct {
    // This should be an error: const builder requires explicit defaults
    name: Option<String>,
}

fn main() {}
