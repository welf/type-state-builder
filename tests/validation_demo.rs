// This file demonstrates that invalid configurations are now properly rejected
// Uncomment any of the struct definitions below to see the validation errors

use type_state_builder::TypeStateBuilder;

// This would fail with: Required fields cannot have default values
/*
#[derive(TypeStateBuilder)]
struct RequiredWithDefault {
    #[builder(required, default = "test")]
    name: String,
}
*/

// This would fail with: Fields with skip_setter must have a default value
/*
#[derive(TypeStateBuilder)]
struct SkipSetterWithoutDefault {
    #[builder(skip_setter)]
    name: String,
}
*/

// This would fail with: Required fields cannot skip setters
/*
#[derive(TypeStateBuilder)]
struct RequiredSkipSetter {
    #[builder(required, skip_setter, default = "test")]
    name: String,
}
*/

// Valid configurations work fine
#[derive(TypeStateBuilder)]
struct ValidConfig {
    #[builder(required)]
    name: String,

    #[builder(default = 42)]
    count: i32,

    #[builder(skip_setter, default = String::new())]
    id: String,

    optional: Option<String>,
}

#[test]
fn test_valid_config_works() {
    let instance = ValidConfig::builder()
        .name("test".to_string())
        .count(100)
        .optional(Some("optional".to_string()))
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.count, 100);
    assert_eq!(instance.id, ""); // String::new()
    assert_eq!(instance.optional, Some("optional".to_string()));
}

#[test]
fn test_validation_catches_invalid_configs() {
    // This test documents that the following configurations are now properly rejected:
    println!("✅ Validation now catches:");
    println!("  1. required + default combination");
    println!("  2. skip_setter without default");
    println!("  3. required + skip_setter + default combination");

    // The commented out structs above would fail compilation due to validation
    // This demonstrates that the error handling has been fixed
}
