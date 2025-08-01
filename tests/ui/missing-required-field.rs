use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct User {
    #[builder(required)]
    name: String,

    #[builder(required)]
    email: String,

    age: Option<u32>,
}

fn main() {
    // This should fail: trying to build without setting the required 'email' field
    let user = User::builder()
        .name("Alice".to_string())
        .build(); // Error: missing required field 'email'
}
