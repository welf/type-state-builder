//! Tests for custom build method names
//!
//! This file tests the #[builder(build_method = "name")] attribute functionality

use type_state_builder::TypeStateBuilder;

#[test]
fn test_custom_build_method_name_with_required_fields() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create")]
    struct Config {
        #[builder(required)]
        name: String,

        #[builder(required)]
        version: u32,

        debug_mode: Option<bool>,
    }

    let config = Config::builder()
        .name("MyApp".to_string())
        .version(42)
        .debug_mode(Some(true))
        .create(); // Using custom build method name!

    assert_eq!(config.name, "MyApp");
    assert_eq!(config.version, 42);
    assert_eq!(config.debug_mode, Some(true));
}

#[test]
fn test_custom_build_method_name_with_only_optional_fields() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "finish")]
    struct Settings {
        timeout: Option<u64>,
        retries: Option<usize>,
        #[builder(default = true)]
        enabled: bool,
    }

    let settings = Settings::builder()
        .timeout(Some(5000))
        .retries(Some(3))
        .finish(); // Using custom build method name!

    assert_eq!(settings.timeout, Some(5000));
    assert_eq!(settings.retries, Some(3));
    assert!(settings.enabled); // Uses custom default
}

#[test]
fn test_custom_build_method_name_with_generics() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "construct")]
    struct Container<T> {
        #[builder(required)]
        data: T,

        capacity: Option<usize>,
    }

    let container = Container::<String>::builder()
        .data("hello".to_string())
        .capacity(Some(100))
        .construct(); // Using custom build method name!

    assert_eq!(container.data, "hello");
    assert_eq!(container.capacity, Some(100));
}

#[test]
fn test_custom_build_method_name_with_const_generics() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "build_array")]
    struct ArrayWrapper<T, const N: usize> {
        #[builder(required)]
        data: [T; N],

        name: Option<String>,
    }

    let wrapper = ArrayWrapper::<i32, 3>::builder()
        .data([1, 2, 3])
        .name(Some("numbers".to_string()))
        .build_array(); // Using custom build method name!

    assert_eq!(wrapper.data, [1, 2, 3]);
    assert_eq!(wrapper.name, Some("numbers".to_string()));
}

#[test]
fn test_default_build_method_still_works() {
    #[derive(TypeStateBuilder)]
    struct DefaultStruct {
        #[builder(required)]
        value: String,

        optional: Option<i32>,
    }

    let instance = DefaultStruct::builder()
        .value("test".to_string())
        .optional(Some(42))
        .build(); // Default build method name still works

    assert_eq!(instance.value, "test");
    assert_eq!(instance.optional, Some(42));
}

#[test]
fn test_snake_case_build_method_name() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "build_instance")]
    struct SnakeCaseTest {
        #[builder(required)]
        field: String,
    }

    let instance = SnakeCaseTest::builder()
        .field("test".to_string())
        .build_instance(); // Snake case method name

    assert_eq!(instance.field, "test");
}

#[test]
fn test_custom_build_method_with_skip_setter() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "finalize")]
    struct WithSkipSetter {
        #[builder(required)]
        name: String,

        #[builder(skip_setter, default = "auto_generated".to_string())]
        id: String,

        optional_field: Option<i32>,
    }

    let instance = WithSkipSetter::builder()
        .name("test".to_string())
        .optional_field(Some(123))
        .finalize(); // Custom build method with skip_setter field

    assert_eq!(instance.name, "test");
    assert_eq!(instance.id, "auto_generated"); // Auto-generated field
    assert_eq!(instance.optional_field, Some(123));
}
