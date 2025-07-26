use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct User {
    #[builder(required)]
    name: String,

    #[builder(required)]
    email: String,

    age: Option<u32>,
    active: bool, // Will use Default::default()
}

fn main() {
    // Usage - this enforces that name and email are set
    let user = User::builder()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(Some(30))
        .build();

    println!("Created user: {} ({})", user.name, user.email);
    println!("Age: {:?}, Active: {}", user.age, user.active);
}
