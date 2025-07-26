//! Enhanced error message tests
//!
//! This file demonstrates the improved error messages when Default bounds are missing.

#[cfg(test)]
mod error_message_tests {
    use type_state_builder::TypeStateBuilder;

    // A type that doesn't implement Default
    struct MyEnum {
        value: i32,
    }

    // Test case that shows enhanced error messages
    #[cfg(FALSE)] // Comment out to see error messages
    #[derive(TypeStateBuilder)]
    struct TestStructWithError {
        optional_field: MyEnum, // This will show helpful error message
    }

    // Working solution using custom default
    #[derive(TypeStateBuilder)]
    struct TestStructWithSolution {
        #[builder(default = "MyEnum { value: 42 }")]
        optional_field: MyEnum,
    }

    #[test]
    fn test_working_solution() {
        let instance = TestStructWithSolution::builder().build();
        assert_eq!(instance.optional_field.value, 42);
    }

    #[test]
    fn test_skip_setter_feature() {
        #[derive(TypeStateBuilder)]
        struct WithSkipSetter {
            #[builder(skip_setter, default = "\"auto_value\".to_string()")]
            auto_field: String,

            optional_field: Option<String>,
        }

        let instance = WithSkipSetter::builder()
            .optional_field(Some("test".to_string()))
            .build();

        assert_eq!(instance.auto_field, "auto_value");
        assert_eq!(instance.optional_field, Some("test".to_string()));

        // Verify no setter exists for auto_field (would fail to compile if uncommented):
        // let _fail = WithSkipSetter::builder().auto_field("fail".to_string());
    }
}
