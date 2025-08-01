use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Person {
    #[builder(required)]
    name: String,

    #[builder(required)]
    age: u32,
}

fn main() {
    // This should fail: trying to build without setting any required fields
    let person = Person::builder().build();
}
