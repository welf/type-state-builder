//! Const Builder Example
//!
//! Demonstrates compile-time constant construction using `#[builder(const)]`.

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct Config {
    #[builder(required)]
    name: &'static str,

    #[builder(required)]
    version: u32,

    #[builder(default = 8080)]
    port: u16,

    #[builder(default = false)]
    debug: bool,
}

// Compile-time constant
const APP_CONFIG: Config = Config::builder()
    .name("my-app")
    .version(1)
    .port(3000)
    .build();

// Static context
static DEFAULT_CONFIG: Config = Config::builder().name("default").version(0).build();

// Const function
const fn make_config(name: &'static str, version: u32) -> Config {
    Config::builder()
        .name(name)
        .version(version)
        .debug(true)
        .build()
}

const DEBUG_CONFIG: Config = make_config("debug", 999);

// Const builder with converter
#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(const)]
struct Data {
    #[builder(required, converter = |s: &'static str| s.len())]
    name_length: usize,

    #[builder(default = 0, converter = |n: i32| n * 2)]
    doubled: i32,
}

const DATA: Data = Data::builder().name_length("hello").doubled(21).build();

fn main() {
    println!("APP_CONFIG: {:?}", APP_CONFIG);
    println!("DEFAULT_CONFIG: {:?}", DEFAULT_CONFIG);
    println!("DEBUG_CONFIG: {:?}", DEBUG_CONFIG);
    println!("DATA: {:?}", DATA);

    assert_eq!(APP_CONFIG.name, "my-app");
    assert_eq!(APP_CONFIG.version, 1);
    assert_eq!(APP_CONFIG.port, 3000);

    assert_eq!(DEFAULT_CONFIG.port, 8080);

    // Verify debug field values at runtime
    let app_debug = APP_CONFIG.debug;
    let debug_debug = DEBUG_CONFIG.debug;
    assert!(!app_debug);
    assert!(debug_debug);

    assert_eq!(DATA.name_length, 5);
    assert_eq!(DATA.doubled, 42);

    println!("All assertions passed!");
}
