//! Integration tests for setter_prefix functionality
//!
//! This module tests the complete setter_prefix feature from attribute parsing
//! through code generation to ensure it works correctly in real usage scenarios.

use type_state_builder::TypeStateBuilder;

// Test struct-level setter_prefix
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(setter_prefix = "with_")]
struct PersonWithPrefix {
    #[builder(required)]
    name: String,

    #[builder(required)]
    age: u32,

    email: Option<String>,
    phone: Option<String>,
}

// Test field-level setter_prefix overriding struct-level
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(setter_prefix = "with_")]
struct MixedPrefixes {
    #[builder(required)]
    name: String,

    #[builder(required, setter_prefix = "set_")]
    age: u32,

    #[builder(setter_prefix = "use_")]
    email: Option<String>,

    // This field inherits struct-level prefix
    phone: Option<String>,
}

// Test field-level setter_prefix with custom setter names
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct CustomNamesWithPrefixes {
    #[builder(required, setter_name = "full_name", setter_prefix = "with_")]
    name: String,

    #[builder(required)]
    age: u32,

    #[builder(setter_name = "contact", setter_prefix = "set_")]
    email: Option<String>,
}

// Test that setter_prefix is incompatible with skip_setter (this should not compile)
// #[derive(TypeStateBuilder)]
// struct InvalidCombination {
//     #[builder(setter_prefix = "with_", skip_setter, default = "42")]
//     value: i32,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_level_setter_prefix() {
        let person = PersonWithPrefix::builder()
            .with_name("Alice".to_string())
            .with_age(30)
            .with_email(Some("alice@example.com".to_string()))
            .with_phone(Some("123-456-7890".to_string()))
            .build();

        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 30);
        assert_eq!(person.email, Some("alice@example.com".to_string()));
        assert_eq!(person.phone, Some("123-456-7890".to_string()));
    }

    #[test]
    fn test_struct_level_prefix_with_defaults() {
        let person = PersonWithPrefix::builder()
            .with_name("Bob".to_string())
            .with_age(25)
            .build();

        assert_eq!(person.name, "Bob");
        assert_eq!(person.age, 25);
        assert_eq!(person.email, None);
        assert_eq!(person.phone, None);
    }

    #[test]
    fn test_field_level_prefix_overrides_struct_level() {
        let mixed = MixedPrefixes::builder()
            .with_name("Charlie".to_string()) // Uses struct-level prefix
            .set_age(35) // Uses field-level prefix (overrides struct)
            .use_email(Some("charlie@example.com".to_string())) // Uses field-level prefix
            .with_phone(Some("987-654-3210".to_string())) // Uses struct-level prefix
            .build();

        assert_eq!(mixed.name, "Charlie");
        assert_eq!(mixed.age, 35);
        assert_eq!(mixed.email, Some("charlie@example.com".to_string()));
        assert_eq!(mixed.phone, Some("987-654-3210".to_string()));
    }

    #[test]
    fn test_field_level_prefix_without_struct_level() {
        let custom = CustomNamesWithPrefixes::builder()
            .with_full_name("Diana".to_string()) // Custom name with field-level prefix
            .age(28) // No prefix (no struct-level, no field-level)
            .set_contact(Some("diana@example.com".to_string())) // Custom name with field-level prefix
            .build();

        assert_eq!(custom.name, "Diana");
        assert_eq!(custom.age, 28);
        assert_eq!(custom.email, Some("diana@example.com".to_string()));
    }

    #[test]
    fn test_method_chaining_with_prefixes() {
        // Test that method chaining works correctly with prefixed setters
        let person = PersonWithPrefix::builder()
            .with_name("Eve".to_string())
            .with_age(32)
            .with_email(Some("eve@example.com".to_string()))
            .build();

        // Verify the builder pattern still works as expected
        assert_eq!(person.name, "Eve");
        assert_eq!(person.age, 32);
        assert_eq!(person.email, Some("eve@example.com".to_string()));
        assert_eq!(person.phone, None);
    }

    #[test]
    fn test_required_fields_with_prefixes() {
        // Test that all required fields must still be set, even with prefixes
        let person = PersonWithPrefix::builder()
            .with_name("Frank".to_string())
            .with_age(40)
            .build();

        assert_eq!(person.name, "Frank");
        assert_eq!(person.age, 40);
    }

    #[test]
    fn test_mixed_prefix_scenarios() {
        // Test complex scenario with mixed prefixes and custom names
        let mixed = MixedPrefixes::builder()
            .with_name("Grace".to_string()) // struct prefix
            .set_age(45) // field prefix overrides struct
            .use_email(Some("grace@example.com".to_string())) // field prefix
            .build();

        assert_eq!(mixed.name, "Grace");
        assert_eq!(mixed.age, 45);
        assert_eq!(mixed.email, Some("grace@example.com".to_string()));
        assert_eq!(mixed.phone, None); // Default value
    }

    // Test to ensure type safety is maintained with prefixes
    #[test]
    fn test_type_safety_with_prefixes() {
        // This test ensures that the type state builder pattern still works
        // correctly even with setter prefixes applied

        // This should compile - all required fields are set
        let _valid = PersonWithPrefix::builder()
            .with_name("Henry".to_string())
            .with_age(50)
            .build();

        // The following should NOT compile (commented out to avoid test failure):
        // let _invalid = PersonWithPrefix::builder()
        //     .with_name("Invalid".to_string())
        //     // Missing .with_age() call
        //     .build(); // This should be a compile error
    }

    #[test]
    fn test_prefix_with_different_types() {
        // Test that prefixes work with different field types
        let custom = CustomNamesWithPrefixes::builder()
            .with_full_name("Iris".to_string()) // String with prefix
            .age(22) // u32 without prefix
            .set_contact(Some("iris@example.com".to_string())) // Option<String> with prefix
            .build();

        assert_eq!(custom.name, "Iris");
        assert_eq!(custom.age, 22);
        assert_eq!(custom.email, Some("iris@example.com".to_string()));
    }
}
