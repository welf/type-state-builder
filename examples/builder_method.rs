//! Builder Method Entry Point Example
//!
//! Demonstrates using `#[builder(builder_method)]` to make a field's setter
//! the entry point to the builder, replacing the `builder()` method.

use type_state_builder::TypeStateBuilder;

// Basic usage: User::id(1) instead of User::builder().id(1)
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct User {
    #[builder(required, builder_method)]
    id: u64,

    #[builder(required)]
    name: String,

    email: Option<String>,
}

// With custom setter name
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct Entity {
    #[builder(required, builder_method, setter_name = "with_id")]
    id: u64,

    #[builder(required)]
    value: i32,
}

// With setter prefix
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct Item {
    #[builder(required, builder_method, setter_prefix = "new_with_")]
    id: u64,

    #[builder(required)]
    name: String,
}

// With const builder
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct Config {
    #[builder(required, builder_method)]
    name: &'static str,

    // In const builders default values must be provided explicitly
    #[builder(default = 0)]
    version: u32,
}

const APP: Config = Config::name("myapp").version(1).build();

// With converter
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct Record {
    #[builder(required, builder_method, converter = |s: &str| s.to_uppercase())]
    id: String,

    #[builder(required)]
    value: i32,
}

fn main() {
    // Basic usage
    let user = User::id(1)
        .name("Alice".to_string())
        .email(Some("alice@example.com".to_string()))
        .build();

    println!("User: {:?}", user);
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");

    // With custom setter name
    let entity = Entity::with_id(42).value(100).build();
    println!("Entity: {:?}", entity);
    assert_eq!(entity.id, 42);

    // With setter prefix
    let item = Item::new_with_id(5).name("test".to_string()).build();
    println!("Item: {:?}", item);
    assert_eq!(item.id, 5);

    // Const builder
    println!("Config: {:?}", APP);
    assert_eq!(APP.name, "myapp");
    assert_eq!(APP.version, 1);

    // With converter
    let record = Record::id("abc").value(99).build();
    println!("Record: {:?}", record);
    assert_eq!(record.id, "ABC");

    println!("All assertions passed!");
}
