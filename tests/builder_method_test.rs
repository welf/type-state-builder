use type_state_builder::TypeStateBuilder;

// =============================================================================
// Basic builder_method tests
// =============================================================================

#[test]
fn test_basic_builder_method() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct User {
        #[builder(required, builder_method)]
        id: u64,
        #[builder(required)]
        name: String,
    }

    // Use builder_method entry point instead of builder()
    let user = User::id(42).name("Alice".to_string()).build();

    assert_eq!(user.id, 42);
    assert_eq!(user.name, "Alice");
}

#[test]
fn test_builder_method_with_optional_field() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct Config {
        #[builder(required, builder_method)]
        name: String,
        #[builder(default = 8080)]
        port: u16,
    }

    // Without setting optional field
    let config1 = Config::name("app".to_string()).build();
    assert_eq!(config1.name, "app");
    assert_eq!(config1.port, 8080);

    // With setting optional field
    let config2 = Config::name("app".to_string()).port(3000).build();
    assert_eq!(config2.name, "app");
    assert_eq!(config2.port, 3000);
}

#[test]
fn test_builder_method_with_setter_name() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct Entity {
        #[builder(required, builder_method, setter_name = "with_id")]
        id: u64,
        #[builder(required)]
        value: i32,
    }

    // Uses custom setter name as entry point
    let entity = Entity::with_id(1).value(100).build();
    assert_eq!(entity.id, 1);
    assert_eq!(entity.value, 100);
}

#[test]
fn test_builder_method_with_setter_prefix() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct Item {
        #[builder(required, builder_method, setter_prefix = "with_")]
        id: u64,
        #[builder(required)]
        name: String,
    }

    // Uses prefixed setter name as entry point
    let item = Item::with_id(5).name("test".to_string()).build();
    assert_eq!(item.id, 5);
    assert_eq!(item.name, "test");
}

#[test]
fn test_builder_method_with_multiple_required_fields() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct Record {
        #[builder(required, builder_method)]
        id: u64,
        #[builder(required)]
        name: String,
        #[builder(required)]
        value: i32,
    }

    let record = Record::id(1).name("test".to_string()).value(42).build();

    assert_eq!(record.id, 1);
    assert_eq!(record.name, "test");
    assert_eq!(record.value, 42);
}

// =============================================================================
// Const builder_method tests
// =============================================================================

#[test]
fn test_const_builder_method() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Config {
        #[builder(required, builder_method)]
        name: &'static str,
        #[builder(required)]
        version: u32,
    }

    const CONFIG: Config = Config::name("app").version(1).build();

    assert_eq!(CONFIG.name, "app");
    assert_eq!(CONFIG.version, 1);
}

#[test]
fn test_const_builder_method_with_optional() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Settings {
        #[builder(required, builder_method)]
        id: u64,
        #[builder(default = 8080)]
        port: u16,
    }

    const SETTINGS: Settings = Settings::id(42).port(3000).build();

    assert_eq!(SETTINGS.id, 42);
    assert_eq!(SETTINGS.port, 3000);
}

#[test]
fn test_const_builder_method_in_static() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct AppConfig {
        #[builder(required, builder_method)]
        name: &'static str,
        #[builder(default = 0)]
        debug_level: u8,
    }

    static APP: AppConfig = AppConfig::name("myapp").build();

    assert_eq!(APP.name, "myapp");
    assert_eq!(APP.debug_level, 0);
}

#[test]
fn test_const_builder_method_with_converter() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Data {
        #[builder(required, builder_method, converter = |s: &'static str| s.len())]
        name_len: usize,
        #[builder(required)]
        value: i32,
    }

    const DATA: Data = Data::name_len("hello").value(42).build();

    assert_eq!(DATA.name_len, 5);
    assert_eq!(DATA.value, 42);
}
