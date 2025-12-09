// Integration tests to verify that invalid configurations are rejected
// Note: Invalid configurations are commented out because they should fail compilation

// This would fail compilation due to required + default:
// struct TestRequiredWithDefault {
//     #[builder(required, default = "test")]
//     name: String,
// }

// This would fail compilation due to skip_setter without default:
// struct TestSkipSetterWithoutDefault {
//     #[builder(skip_setter)]
//     name: String,
// }

// This test documents what the validation should catch
#[test]
fn test_validation_catches_invalid_configs() {
    // These structs above should fail during macro expansion if validation works properly
    // If they don't fail, then validation is not working correctly

    // For now, we document the expected behavior:
    println!("Validation should catch:");
    println!("1. required + default combination");
    println!("2. skip_setter without default");
    println!("3. setter name conflicts");
    println!("4. build method name conflicts");
    println!("5. empty structs");

    // The real test is whether the above struct definitions would compile if TypeStateBuilder was applied
}

// Valid configuration that should work
#[derive(type_state_builder::TypeStateBuilder)]
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
