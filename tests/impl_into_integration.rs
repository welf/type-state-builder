use type_state_builder::TypeStateBuilder;

// Test struct-level impl_into
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(impl_into)]
struct StructLevelImplInto {
    #[builder(required)]
    name: String,

    #[builder(required)]
    email: String,

    age: Option<u32>,
}

// Test field-level impl_into overrides
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(impl_into)]
struct FieldLevelOverrides {
    #[builder(required)]
    name: String, // Inherits struct-level impl_into = true

    #[builder(required, impl_into = false)]
    id: String, // Override to use direct String

    #[builder(impl_into = true)]
    description: Option<String>, // Explicit impl_into = true

    #[builder(impl_into = false)]
    category: Option<String>, // Override to use direct String
}

// Test regular builder with impl_into
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(impl_into)]
struct RegularBuilderImplInto {
    name: String,  // Direct String field, can use &str with impl_into
    email: String, // Direct String field, can use &str with impl_into
    age: Option<u32>,
}

#[test]
fn test_struct_level_impl_into_basic() {
    let user = StructLevelImplInto::builder()
        .name("Alice") // &str -> String via Into
        .email("alice@example.com") // &str -> String via Into
        .age(Some(30))
        .build();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.age, Some(30));
}

#[test]
fn test_struct_level_impl_into_string_variants() {
    let owned_name = "Bob".to_string();
    let borrowed_email = "bob@example.com";

    let user = StructLevelImplInto::builder()
        .name(owned_name.clone()) // String -> String via Into
        .email(borrowed_email) // &str -> String via Into
        .build();

    assert_eq!(user.name, "Bob");
    assert_eq!(user.email, "bob@example.com");
    assert_eq!(user.age, None);
}

#[test]
fn test_field_level_overrides() {
    let user = FieldLevelOverrides::builder()
        .name("Charlie") // &str -> String (inherits impl_into)
        .id("user123".to_string()) // Must use String directly (impl_into = false)
        .description(Some("Test user".to_string())) // Must use Option<String> directly for Option fields
        .category(Some("admin".to_string())) // Must use Option<String> directly (impl_into = false)
        .build();

    assert_eq!(user.name, "Charlie");
    assert_eq!(user.id, "user123");
    assert_eq!(user.description, Some("Test user".to_string()));
    assert_eq!(user.category, Some("admin".to_string()));
}

#[test]
fn test_regular_builder_impl_into() {
    let user = RegularBuilderImplInto::builder()
        .name("Diana") // &str -> String via Into
        .email("diana@example.com") // &str -> String via Into
        .age(Some(28))
        .build();

    assert_eq!(user.name, "Diana");
    assert_eq!(user.email, "diana@example.com");
    assert_eq!(user.age, Some(28));
}

#[test]
fn test_no_impl_into_still_works() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct NoImplInto {
        #[builder(required)]
        name: String,

        age: Option<u32>,
    }

    let user = NoImplInto::builder()
        .name("Eve".to_string()) // Must use String directly
        .age(Some(25))
        .build();

    assert_eq!(user.name, "Eve");
    assert_eq!(user.age, Some(25));
}
