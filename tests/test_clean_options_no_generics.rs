// Test clean optional field architecture without generics

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct CleanOptions {
    #[builder(required)]
    name: String, // Required: String → Option<String> in builder

    age: u32, // Optional: u32 → u32 in builder (initialized with default)

    email: Option<String>, // Optional: Option<String> → Option<String> in builder

    #[builder(default = 42)]
    score: i32, // Optional: i32 with custom default → i32 in builder
}

#[test]
fn test_clean_options() {
    let instance = CleanOptions::builder()
        .name("Alice".to_string())
        .age(30)
        .email(Some("alice@example.com".to_string()))
        .build();

    assert_eq!(instance.name, "Alice");
    assert_eq!(instance.age, 30);
    assert_eq!(instance.email, Some("alice@example.com".to_string()));
    assert_eq!(instance.score, 42); // Custom default
}

#[test]
fn test_clean_options_defaults() {
    let instance = CleanOptions::builder().name("Bob".to_string()).build();

    assert_eq!(instance.name, "Bob");
    assert_eq!(instance.age, 0); // Default for u32
    assert_eq!(instance.email, None); // Default for Option<String>
    assert_eq!(instance.score, 42); // Custom default
}
