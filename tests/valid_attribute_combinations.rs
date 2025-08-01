//! Comprehensive tests for all valid attribute combinations
//!
//! This test suite verifies that all valid attribute combinations work correctly
//! across different scenarios including struct-level and field-level interactions.
//!
//! ## Valid Attribute Combinations Matrix
//!
//! | Attribute | Compatible With | Incompatible With |
//! |-----------|----------------|-------------------|
//! | `required` | `setter_name`, `setter_prefix`, `impl_into`, `converter` | `default`, `skip_setter` |
//! | `setter_name` | `required`, `setter_prefix`, `impl_into`, `converter`, `default` | `skip_setter` |
//! | `setter_prefix` | `required`, `setter_name`, `impl_into`, `converter`, `default` | `skip_setter` |
//! | `default` | `setter_name`, `setter_prefix`, `impl_into`, `converter`, `skip_setter` | `required` |
//! | `skip_setter` | `default` | `required`, `setter_name`, `setter_prefix`, `impl_into`, `converter` |
//! | `impl_into` | `required`, `setter_name`, `setter_prefix`, `default` | `skip_setter`, `converter` |
//! | `converter` | `required`, `setter_name`, `setter_prefix`, `default` | `skip_setter`, `impl_into` |

use type_state_builder::TypeStateBuilder;

#[cfg(test)]
mod field_level_attributes {
    use super::*;

    // Test 1: Basic required field combinations
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RequiredBasic {
        #[builder(required)]
        name: String,
    }

    #[test]
    fn test_required_basic() {
        let instance = RequiredBasic::builder().name("test".to_string()).build();
        assert_eq!(instance.name, "test");
    }

    // Test 2: Required with custom setter name
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RequiredWithSetterName {
        #[builder(required, setter_name = "set_title")]
        name: String,
    }

    #[test]
    fn test_required_with_setter_name() {
        let instance = RequiredWithSetterName::builder()
            .set_title("test".to_string())
            .build();
        assert_eq!(instance.name, "test");
    }

    // Test 3: Required with custom setter prefix
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RequiredWithSetterPrefix {
        #[builder(required, setter_prefix = "with_")]
        name: String,
    }

    #[test]
    fn test_required_with_setter_prefix() {
        let instance = RequiredWithSetterPrefix::builder()
            .with_name("test".to_string())
            .build();
        assert_eq!(instance.name, "test");
    }

    // Test 4: Required with setter name AND prefix
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RequiredWithSetterNameAndPrefix {
        #[builder(required, setter_name = "title", setter_prefix = "set_")]
        name: String,
    }

    #[test]
    fn test_required_with_setter_name_and_prefix() {
        let instance = RequiredWithSetterNameAndPrefix::builder()
            .set_title("test".to_string())
            .build();
        assert_eq!(instance.name, "test");
    }

    // Test 5: Optional field with default
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct OptionalWithDefault {
        #[builder(default = "\"default_value\".to_string()")]
        name: String,
    }

    #[test]
    fn test_optional_with_default() {
        let instance = OptionalWithDefault::builder().build();
        assert_eq!(instance.name, "default_value");

        let instance = OptionalWithDefault::builder()
            .name("custom".to_string())
            .build();
        assert_eq!(instance.name, "custom");
    }

    // Test 6: Optional with default and setter name
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct OptionalWithDefaultAndSetterName {
        #[builder(default = "42", setter_name = "set_value")]
        count: i32,
    }

    #[test]
    fn test_optional_with_default_and_setter_name() {
        let instance = OptionalWithDefaultAndSetterName::builder().build();
        assert_eq!(instance.count, 42);

        let instance = OptionalWithDefaultAndSetterName::builder()
            .set_value(100)
            .build();
        assert_eq!(instance.count, 100);
    }

    // Test 7: Optional with default, setter name, and prefix
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct OptionalWithDefaultSetterNameAndPrefix {
        #[builder(default = "true", setter_name = "enabled", setter_prefix = "with_")]
        active: bool,
    }

    #[test]
    fn test_optional_with_default_setter_name_and_prefix() {
        let instance = OptionalWithDefaultSetterNameAndPrefix::builder().build();
        assert!(instance.active);

        let instance = OptionalWithDefaultSetterNameAndPrefix::builder()
            .with_enabled(false)
            .build();
        assert!(!instance.active);
    }

    // Test 8: Skip setter with default
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct SkipSetterWithDefault {
        name: String,
        #[builder(skip_setter, default = "\"auto_generated\".to_string()")]
        id: String,
    }

    #[test]
    fn test_skip_setter_with_default() {
        let instance = SkipSetterWithDefault::builder()
            .name("test".to_string())
            .build();
        assert_eq!(instance.name, "test");
        assert_eq!(instance.id, "auto_generated");
    }

    // Test 9: Skip setter without explicit default (uses Default::default())
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct SkipSetterWithoutDefault {
        name: String,
        #[builder(skip_setter, default = "Default::default()")]
        count: i32, // Default::default() = 0
    }

    #[test]
    fn test_skip_setter_without_default() {
        let instance = SkipSetterWithoutDefault::builder()
            .name("test".to_string())
            .build();
        assert_eq!(instance.name, "test");
        assert_eq!(instance.count, 0);
    }

    // Test 10: impl_into field-level (overriding struct setting)
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct FieldLevelImplInto {
        #[builder(impl_into = true)]
        name: String,
        #[builder(impl_into = false)]
        exact_type: String,
        regular_field: String,
    }

    #[test]
    fn test_field_level_impl_into() {
        let instance = FieldLevelImplInto::builder()
            .name("test") // Can use &str due to impl_into = true
            .exact_type("exact".to_string()) // Must use String due to impl_into = false
            .regular_field("regular".to_string())
            .build();
        assert_eq!(instance.name, "test");
        assert_eq!(instance.exact_type, "exact");
        assert_eq!(instance.regular_field, "regular");
    }

    // Test 11: impl_into with setter customization
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ImplIntoWithCustomization {
        #[builder(impl_into = true, setter_name = "set_title", setter_prefix = "with_")]
        name: String,
    }

    #[test]
    fn test_impl_into_with_customization() {
        let instance = ImplIntoWithCustomization::builder()
            .with_set_title("test") // Uses impl Into<String>
            .build();
        assert_eq!(instance.name, "test");
    }

    // Test 12: impl_into with default
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ImplIntoWithDefault {
        #[builder(impl_into = true, default = "\"default\".to_string()")]
        name: String,
    }

    #[test]
    fn test_impl_into_with_default() {
        let instance = ImplIntoWithDefault::builder().build();
        assert_eq!(instance.name, "default");

        let instance = ImplIntoWithDefault::builder()
            .name("custom") // Can use &str
            .build();
        assert_eq!(instance.name, "custom");
    }

    // Test 13: Required with impl_into
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RequiredWithImplInto {
        #[builder(required, impl_into = true)]
        name: String,
    }

    #[test]
    fn test_required_with_impl_into() {
        let instance = RequiredWithImplInto::builder()
            .name("test") // Can use &str
            .build();
        assert_eq!(instance.name, "test");
    }
}

#[cfg(test)]
mod converter_combinations {
    use super::*;

    // Test 14: Basic converter
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct BasicConverter {
        #[builder(converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
        tags: Vec<String>,
    }

    #[test]
    fn test_basic_converter() {
        let instance = BasicConverter::builder()
            .tags(vec!["rust", "builder"])
            .build();
        assert_eq!(
            instance.tags,
            vec!["rust".to_string(), "builder".to_string()]
        );
    }

    // Test 15: Required converter
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RequiredConverter {
        #[builder(required, converter = |value: &str| value.to_uppercase())]
        name: String,
    }

    #[test]
    fn test_required_converter() {
        let instance = RequiredConverter::builder().name("test").build();
        assert_eq!(instance.name, "TEST");
    }

    // Test 16: Converter with custom setter name
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ConverterWithSetterName {
        #[builder(converter = |value: i32| value * 2, setter_name = "double_value")]
        number: i32,
    }

    #[test]
    fn test_converter_with_setter_name() {
        let instance = ConverterWithSetterName::builder().double_value(21).build();
        assert_eq!(instance.number, 42);
    }

    // Test 17: Converter with setter prefix
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ConverterWithSetterPrefix {
        #[builder(converter = |value: f32| value as i32, setter_prefix = "set_")]
        value: i32,
    }

    #[test]
    fn test_converter_with_setter_prefix() {
        let instance = ConverterWithSetterPrefix::builder().set_value(42.7).build();
        assert_eq!(instance.value, 42);
    }

    // Test 18: Converter with setter name AND prefix
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ConverterWithSetterNameAndPrefix {
        #[builder(converter = |values: (i32, i32)| values.0 + values.1, setter_name = "sum", setter_prefix = "with_")]
        total: i32,
    }

    #[test]
    fn test_converter_with_setter_name_and_prefix() {
        let instance = ConverterWithSetterNameAndPrefix::builder()
            .with_sum((10, 32))
            .build();
        assert_eq!(instance.total, 42);
    }

    // Test 19: Converter with default
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ConverterWithDefault {
        #[builder(converter = |value: Option<String>| value.unwrap_or_else(|| "fallback".to_string()), default = "\"default\".to_string()")]
        name: String,
    }

    #[test]
    fn test_converter_with_default() {
        let instance = ConverterWithDefault::builder().build();
        assert_eq!(instance.name, "default");

        let instance = ConverterWithDefault::builder()
            .name(Some("custom".to_string()))
            .build();
        assert_eq!(instance.name, "custom");

        let instance = ConverterWithDefault::builder().name(None).build();
        assert_eq!(instance.name, "fallback");
    }

    // Test 20: Required converter with all compatible attributes
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RequiredConverterComplete {
        #[builder(required, converter = |value: Vec<i32>| format!("{value:?}"), setter_name = "numbers", setter_prefix = "with_")]
        formatted: String,
    }

    #[test]
    fn test_required_converter_complete() {
        let instance = RequiredConverterComplete::builder()
            .with_numbers(vec![1, 2, 3])
            .build();
        assert_eq!(instance.formatted, "[1, 2, 3]");
    }

    // Test 21: Complex converter expression
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ComplexConverter {
        #[builder(converter = |value: std::collections::HashMap<String, i32>| {
            value.into_iter()
                .filter(|(_, count)| *count > 0)
                .map(|(name, count)| format!("{name}:{count}"))
                .collect()
        })]
        items: Vec<String>,
    }

    #[test]
    fn test_complex_converter() {
        let mut map = std::collections::HashMap::new();
        map.insert("valid".to_string(), 5);
        map.insert("invalid".to_string(), 0);
        map.insert("another".to_string(), 3);

        let instance = ComplexConverter::builder().items(map).build();

        // Sort for consistent testing since HashMap iteration order is not guaranteed
        let mut result = instance.items;
        result.sort();
        assert_eq!(result, vec!["another:3", "valid:5"]);
    }
}

#[cfg(test)]
mod struct_level_attributes {
    use super::*;

    // Test 22: Struct-level build method name
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(build_method = "create")]
    struct CustomBuildMethod {
        name: String,
    }

    #[test]
    fn test_custom_build_method() {
        let instance = CustomBuildMethod::builder()
            .name("test".to_string())
            .create();
        assert_eq!(instance.name, "test");
    }

    // Test 23: Struct-level setter prefix
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(setter_prefix = "with_")]
    struct StructSetterPrefix {
        name: String,
        age: i32,
    }

    #[test]
    fn test_struct_setter_prefix() {
        let instance = StructSetterPrefix::builder()
            .with_name("test".to_string())
            .with_age(25)
            .build();
        assert_eq!(instance.name, "test");
        assert_eq!(instance.age, 25);
    }

    // Test 24: Struct-level impl_into
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(impl_into)]
    struct StructImplInto {
        name: String,
        path: std::path::PathBuf,
    }

    #[test]
    fn test_struct_impl_into() {
        let instance = StructImplInto::builder()
            .name("test") // &str -> String
            .path("/tmp/test") // &str -> PathBuf
            .build();
        assert_eq!(instance.name, "test");
        assert_eq!(instance.path, std::path::PathBuf::from("/tmp/test"));
    }

    // Test 25: All struct-level attributes combined
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(build_method = "create", setter_prefix = "with_", impl_into)]
    struct AllStructAttributes {
        name: String,
        count: i32,
    }

    #[test]
    fn test_all_struct_attributes() {
        let instance = AllStructAttributes::builder()
            .with_name("test") // Uses prefix and impl_into
            .with_count(42)
            .create(); // Uses custom build method
        assert_eq!(instance.name, "test");
        assert_eq!(instance.count, 42);
    }
}

#[cfg(test)]
mod struct_field_interactions {
    use super::*;

    // Test 26: Struct impl_into with field-level overrides
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(impl_into)]
    struct StructImplIntoWithOverrides {
        #[builder(impl_into = false)] // Override: no impl_into
        exact_name: String,
        inherit_name: String, // Inherits: impl_into = true
        #[builder(impl_into = true)] // Explicit: impl_into = true
        explicit_name: String,
    }

    #[test]
    fn test_struct_impl_into_with_overrides() {
        let instance = StructImplIntoWithOverrides::builder()
            .exact_name("exact".to_string()) // Must use String
            .inherit_name("inherit") // Can use &str (inherited)
            .explicit_name("explicit") // Can use &str (explicit)
            .build();
        assert_eq!(instance.exact_name, "exact");
        assert_eq!(instance.inherit_name, "inherit");
        assert_eq!(instance.explicit_name, "explicit");
    }

    // Test 27: Struct setter prefix with field-level overrides
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(setter_prefix = "with_")]
    struct StructPrefixWithOverrides {
        inherit_field: String, // Uses struct prefix
        #[builder(setter_prefix = "set_")] // Override struct prefix
        override_field: String,
        #[builder(setter_name = "custom_method")] // Custom name (struct prefix still applies)
        custom_field: String,
    }

    #[test]
    fn test_struct_prefix_with_overrides() {
        let instance = StructPrefixWithOverrides::builder()
            .with_inherit_field("inherit".to_string()) // Uses struct prefix
            .set_override_field("override".to_string()) // Uses field prefix
            .with_custom_method("custom".to_string()) // Uses custom name with struct prefix
            .build();
        assert_eq!(instance.inherit_field, "inherit");
        assert_eq!(instance.override_field, "override");
        assert_eq!(instance.custom_field, "custom");
    }

    // Test 28: Complex interaction: struct attributes + field attributes + converters
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(build_method = "construct", setter_prefix = "with_", impl_into)]
    struct ComplexInteraction {
        #[builder(required, converter = |value: &str| value.to_uppercase(), setter_name = "title")]
        name: String,
        #[builder(impl_into = false, setter_prefix = "set_")]
        exact_value: String,
        #[builder(default = "42")]
        count: i32,
        #[builder(skip_setter, default = "\"auto\".to_string()")]
        id: String,
    }

    #[test]
    fn test_complex_interaction() {
        let instance = ComplexInteraction::builder()
            .with_title("test") // Required, converter, custom name, struct prefix
            .set_exact_value("exact".to_string()) // Field prefix, no impl_into
            .with_count(100) // Optional, struct prefix + impl_into
            .construct(); // Custom build method

        assert_eq!(instance.name, "TEST"); // Converted to uppercase
        assert_eq!(instance.exact_value, "exact");
        assert_eq!(instance.count, 100);
        assert_eq!(instance.id, "auto"); // Skip setter, default value
    }

    // Test 29: Mixed required and optional with various combinations
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(setter_prefix = "with_")]
    struct MixedFieldTypes {
        #[builder(required)]
        required_basic: String,
        #[builder(required, setter_name = "set_id")]
        required_custom: i32,
        #[builder(default = "\"optional\".to_string()")]
        optional_default: String,
        #[builder(converter = |value: Vec<i32>| value.into_iter().sum())]
        optional_converter: i32,
        #[builder(skip_setter, default = "true")]
        skip_field: bool,
    }

    #[test]
    fn test_mixed_field_types() {
        let instance = MixedFieldTypes::builder()
            .with_required_basic("basic".to_string())
            .with_set_id(123)
            .with_optional_converter(vec![10, 20, 30])
            .build();

        assert_eq!(instance.required_basic, "basic");
        assert_eq!(instance.required_custom, 123);
        assert_eq!(instance.optional_default, "optional"); // Default value
        assert_eq!(instance.optional_converter, 60); // Sum of vec![10, 20, 30]
        assert!(instance.skip_field); // Skip setter default
    }

    // Test 30: Edge case: Only optional fields with defaults and skip setters
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct OnlyOptionalFields {
        #[builder(default = "\"default1\".to_string()")]
        field1: String,
        #[builder(skip_setter, default = "\"skip_default\".to_string()")]
        field2: String,
        #[builder(converter = |value: Option<i32>| value.unwrap_or(0))]
        field3: i32,
    }

    #[test]
    fn test_only_optional_fields() {
        // Test with no setters called (all defaults)
        let instance = OnlyOptionalFields::builder().build();
        assert_eq!(instance.field1, "default1");
        assert_eq!(instance.field2, "skip_default");
        assert_eq!(instance.field3, 0); // Default::default()

        // Test with some setters called
        let instance = OnlyOptionalFields::builder()
            .field1("custom1".to_string())
            .field3(Some(42))
            .build();
        assert_eq!(instance.field1, "custom1");
        assert_eq!(instance.field2, "skip_default"); // Skip setter, always default
        assert_eq!(instance.field3, 42);
    }
}

#[cfg(test)]
mod generics_with_attributes {
    use super::*;

    // Test 31: Generic struct with various attribute combinations
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(setter_prefix = "with_")]
    struct GenericWithAttributes<T>
    where
        T: Clone + std::fmt::Debug,
    {
        #[builder(required)]
        data: T,
        #[builder(converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
        tags: Vec<String>,
        #[builder(default = "42")]
        count: i32,
    }

    #[test]
    fn test_generic_with_attributes() {
        let instance = GenericWithAttributes::<String>::builder()
            .with_data("test".to_string())
            .with_tags(vec!["rust", "generic"])
            .with_count(100)
            .build();

        assert_eq!(instance.data, "test");
        assert_eq!(
            instance.tags,
            vec!["rust".to_string(), "generic".to_string()]
        );
        assert_eq!(instance.count, 100);
    }

    // Test 32: Lifetime parameters with attributes
    #[derive(TypeStateBuilder, Debug)]
    #[builder(impl_into)]
    struct LifetimeWithAttributes<'a> {
        #[builder(required)]
        data: &'a str,
        #[builder(converter = |value: Vec<&str>| value.join(","))]
        combined: String,
    }

    #[test]
    fn test_lifetime_with_attributes() {
        let data = "test data";
        let instance = LifetimeWithAttributes::builder()
            .data(data) // Can use &str directly due to impl_into
            .combined(vec!["a", "b", "c"])
            .build();

        assert_eq!(instance.data, "test data");
        assert_eq!(instance.combined, "a,b,c");
    }

    // Test 33: Const generics with attributes
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(build_method = "create")]
    struct ConstGenericWithAttributes<const N: usize> {
        #[builder(required)]
        data: [i32; N],
        #[builder(converter = |value: Vec<i32>| value.into_iter().sum())]
        sum: i32,
    }

    #[test]
    fn test_const_generic_with_attributes() {
        let instance = ConstGenericWithAttributes::<3>::builder()
            .data([1, 2, 3])
            .sum(vec![10, 20, 30])
            .create();

        assert_eq!(instance.data, [1, 2, 3]);
        assert_eq!(instance.sum, 60);
    }
}

#[cfg(test)]
mod edge_cases_and_special_scenarios {
    use core::f64;

    use super::*;

    // Test 34: Raw identifiers with attributes
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct RawIdentifiers {
        #[builder(required, setter_name = "set_type")]
        r#type: String,
        #[builder(converter = |value: &str| format!("async_{value}"))]
        r#async: String,
    }

    #[test]
    fn test_raw_identifiers() {
        let instance = RawIdentifiers::builder()
            .set_type("test".to_string())
            .r#async("operation")
            .build();

        assert_eq!(instance.r#type, "test");
        assert_eq!(instance.r#async, "async_operation");
    }

    // Test 35: Empty struct with only skip setter fields
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct OnlySkipSetters {
        #[builder(skip_setter, default = "\"auto1\".to_string()")]
        field1: String,
        #[builder(skip_setter, default = "42")]
        field2: i32,
        #[builder(skip_setter, default = "true")]
        field3: bool,
    }

    #[test]
    fn test_only_skip_setters() {
        let instance = OnlySkipSetters::builder().build();
        assert_eq!(instance.field1, "auto1");
        assert_eq!(instance.field2, 42);
        assert!(instance.field3);
    }

    // Test 36: Single required field with all compatible attributes
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(build_method = "finish", setter_prefix = "use_")]
    struct SingleRequiredComplete {
        #[builder(
            required,
            impl_into = true,
            setter_name = "value",
            setter_prefix = "set_"
        )]
        data: String,
    }

    #[test]
    fn test_single_required_complete() {
        let instance = SingleRequiredComplete::builder()
            .set_value("test") // Field prefix overrides struct prefix, impl_into allows &str
            .finish(); // Custom build method
        assert_eq!(instance.data, "test");
    }

    // Test 37: All attribute combinations in a single struct
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(build_method = "construct", setter_prefix = "with_", impl_into)]
    struct AllCombinations {
        // Required + converter + custom name + field prefix
        #[builder(required, converter = |value: &str| value.to_uppercase(), setter_name = "title", setter_prefix = "set_")]
        name: String,

        // Required + impl_into override + custom name
        #[builder(required, impl_into = false, setter_name = "exact_age")]
        age: i32,

        // Optional + default + converter + struct prefix inherited
        #[builder(default = "vec![]", converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
        tags: Vec<String>,

        // Optional + impl_into override + field prefix
        #[builder(impl_into = true, setter_prefix = "use_")]
        path: String,

        // Skip setter + default
        #[builder(skip_setter, default = "\"generated\".to_string()")]
        id: String,

        // Plain optional with struct-level impl_into
        description: String,
    }

    #[test]
    fn test_all_combinations() {
        let instance = AllCombinations::builder()
            .set_title("test") // Required, converter, custom name, field prefix
            .with_exact_age(25) // Required, no impl_into, custom name, struct prefix
            .with_tags(vec!["rust", "test"]) // Optional, converter, struct prefix
            .use_path("/tmp") // Optional, impl_into, field prefix
            .with_description("desc") // Optional, struct impl_into, struct prefix
            .construct(); // Custom build method

        assert_eq!(instance.name, "TEST"); // Converted to uppercase
        assert_eq!(instance.age, 25);
        assert_eq!(instance.tags, vec!["rust".to_string(), "test".to_string()]);
        assert_eq!(instance.path, "/tmp");
        assert_eq!(instance.id, "generated"); // Skip setter, default
        assert_eq!(instance.description, "desc");
    }

    // Test 38: Complex converters with various types
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ComplexConverterTypes {
        // Convert tuple to struct field
        #[builder(converter = |value: (String, i32, bool)| format!("{}:{}:{}", value.0, value.1, value.2))]
        formatted: String,

        // Convert Result to Option
        #[builder(converter = |value: Result<String, String>| value.ok())]
        maybe_value: Option<String>,

        // Convert closure result
        #[builder(converter = |value: Box<dyn Fn(i32) -> i32>| value(42))]
        computed: i32,

        // Convert with complex generic bounds
        #[builder(converter = |value: std::collections::BTreeMap<String, Vec<i32>>| {
            value.into_iter()
                .map(|(k, v)| (k, v.into_iter().sum::<i32>()))
                .collect()
        })]
        summary: std::collections::BTreeMap<String, i32>,
    }

    #[test]
    fn test_complex_converter_types() {
        let mut btree = std::collections::BTreeMap::new();
        btree.insert("group1".to_string(), vec![1, 2, 3]);
        btree.insert("group2".to_string(), vec![10, 20]);

        let instance = ComplexConverterTypes::builder()
            .formatted(("test".to_string(), 42, true))
            .maybe_value(Ok("success".to_string()))
            .computed(Box::new(|x| x * 2) as Box<dyn Fn(i32) -> i32>)
            .summary(btree)
            .build();

        assert_eq!(instance.formatted, "test:42:true");
        assert_eq!(instance.maybe_value, Some("success".to_string()));
        assert_eq!(instance.computed, 84); // 42 * 2

        let mut expected_summary = std::collections::BTreeMap::new();
        expected_summary.insert("group1".to_string(), 6); // 1+2+3
        expected_summary.insert("group2".to_string(), 30); // 10+20
        assert_eq!(instance.summary, expected_summary);
    }

    // Test 39: Nested generic types with attributes
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(impl_into)]
    struct NestedGenerics<T>
    where
        T: Clone + std::fmt::Debug,
    {
        #[builder(required)]
        data: Vec<Option<T>>,

        #[builder(converter = |value: Vec<f64>| value.into_iter().map(|u| format!("{u}")).collect())]
        formatted: Vec<String>,

        #[builder(default = "std::collections::HashMap::new()")]
        map: std::collections::HashMap<String, T>,
    }

    #[test]
    fn test_nested_generics() {
        let instance = NestedGenerics::<i32>::builder()
            .data(vec![Some(1), None, Some(3)]) // Vec<Option<i32>>
            .formatted(vec![1.5, 2.7, f64::consts::PI]) // Vec<f64> -> Vec<String>
            .build();

        assert_eq!(instance.data, vec![Some(1), None, Some(3)]);
        assert_eq!(
            instance.formatted,
            vec![
                "1.5".to_string(),
                "2.7".to_string(),
                f64::consts::PI.to_string()
            ]
        );
        assert!(instance.map.is_empty()); // Default HashMap
    }

    // Test 40: All possible boolean combinations for flags
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(impl_into)] // Enable impl_into at struct level
    struct BooleanCombinations {
        #[builder(required, impl_into = true)] // Explicit overrides
        field1: String,

        #[builder(impl_into = false)] // Explicit false
        field2: String,

        #[builder(skip_setter, default = "\"default\".to_string()")]
        field3: String,

        // Implicit optional field (skip_setter = false is default)
        field4: String,
    }

    #[test]
    fn test_boolean_combinations() {
        let instance = BooleanCombinations::builder()
            .field1("test1") // Can use &str due to impl_into = true
            .field2("test2".to_string()) // Must use String due to impl_into = false
            .field4("test4".to_string()) // Optional field with setter
            .build();

        assert_eq!(instance.field1, "test1");
        assert_eq!(instance.field2, "test2");
        assert_eq!(instance.field3, "default"); // Skip setter
        assert_eq!(instance.field4, "test4");
    }
}

#[cfg(test)]
mod documentation_examples {
    use super::*;

    // Test 41: Realistic user profile example
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(setter_prefix = "with_")]
    struct UserProfile {
        #[builder(required)]
        username: String,

        #[builder(required, impl_into = true)]
        email: String,

        #[builder(converter = |age_str: &str| age_str.parse::<u32>().unwrap_or(0))]
        age: u32,

        #[builder(default = "vec![]", converter = |interests: &[&str]| interests.iter().map(|s| s.to_string()).collect())]
        interests: Vec<String>,

        #[builder(skip_setter, default = "1234567890")]
        created_at: u64,

        #[builder(default = "true")]
        active: bool,
    }

    #[test]
    fn test_user_profile_example() {
        let instance = UserProfile::builder()
            .with_username("alice".to_string())
            .with_email("alice@example.com") // Uses impl_into
            .with_age("25") // Converter from &str to u32
            .with_interests(&["rust", "programming"]) // Converter from &[&str]
            .with_active(false)
            .build();

        assert_eq!(instance.username, "alice");
        assert_eq!(instance.email, "alice@example.com");
        assert_eq!(instance.age, 25);
        assert_eq!(
            instance.interests,
            vec!["rust".to_string(), "programming".to_string()]
        );
        assert_eq!(instance.created_at, 1234567890); // Fixed timestamp
        assert!(!instance.active);
    }

    // Test 42: Configuration builder example
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(build_method = "finalize", impl_into)]
    struct ServerConfig {
        #[builder(required, setter_name = "host")]
        hostname: String,

        #[builder(required)]
        port: u16,

        #[builder(converter = |env: &str| matches!(env.to_lowercase().as_str(), "true" | "1" | "yes" | "on"))]
        ssl_enabled: bool,

        #[builder(converter = |paths: Vec<&str>| paths.into_iter().map(std::path::PathBuf::from).collect(), default = "vec![]")]
        cert_paths: Vec<std::path::PathBuf>,

        #[builder(default = "1")]
        worker_threads: usize,

        #[builder(skip_setter, default = "String::from(\"server-v1.0\")")]
        version: String,
    }

    #[test]
    fn test_server_config_example() {
        let instance = ServerConfig::builder()
            .host("localhost") // Custom setter name, impl_into
            .port(8080u16)
            .ssl_enabled("true") // Converter from env string
            .cert_paths(vec!["/etc/ssl/cert.pem", "/etc/ssl/key.pem"]) // Converter to PathBuf
            .worker_threads(4usize)
            .finalize(); // Custom build method

        assert_eq!(instance.hostname, "localhost");
        assert_eq!(instance.port, 8080);
        assert!(instance.ssl_enabled);
        assert_eq!(
            instance.cert_paths,
            vec![
                std::path::PathBuf::from("/etc/ssl/cert.pem"),
                std::path::PathBuf::from("/etc/ssl/key.pem")
            ]
        );
        assert_eq!(instance.worker_threads, 4);
        assert_eq!(instance.version, "server-v1.0"); // Skip setter
    }
}
