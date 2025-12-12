use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(const)]
struct InvalidStruct {
    // This should be an error: field-level impl_into not allowed with const
    #[builder(required, impl_into)]
    name: String,
}

fn main() {}
