use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(const, impl_into)]
struct InvalidStruct {
    // This should be an error: const and impl_into are incompatible
    name: String,
}

fn main() {}
