use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Person {
    #[builder(required)]
    name: String,

    #[builder(required)]
    age: u32,
}

fn main() {
    // This should fail to compile because no fields are set at all
    // Error: missing required fields 'name' and 'age'
    let person = Person::builder().build();
}
