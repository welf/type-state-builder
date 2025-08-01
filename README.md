# TypeStateBuilder

[![Crates.io](https://img.shields.io/crates/v/type-state-builder.svg)](https://crates.io/crates/type-state-builder)
[![Documentation](https://docs.rs/type-state-builder/badge.svg)](https://docs.rs/type-state-builder)
[![CI](https://github.com/welf/type-state-builder/workflows/CI/badge.svg)](https://github.com/welf/type-state-builder/actions)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**TypeStateBuilder** is a Rust derive macro that generates compile-time safe builders using the **type-state builder
pattern**. It prevents runtime errors by making it impossible to build incomplete objects, while providing an ergonomic
and intuitive API and developer-friendly compilation errors.

## 📚 Table of Contents

- [🚀 Why TypeStateBuilder?](#-why-typestatebuilder)
- [📦 Installation](#-installation)
- [🎯 Quick Start](#-quick-start)
- [🛠️ Features](#️-features)
  - [✨ Compile-Time Safety](#-compile-time-safety)
  - [🎛️ Field Types](#️-field-types)
  - [🏷️ Custom Setter Names](#️-custom-setter-names)
  - [🎯 Setter Prefixes](#-setter-prefixes)
  - [🔄 Ergonomic Conversions](#-ergonomic-conversions)
  - [🔧 Custom Conversions](#-custom-conversions-with-converter)
  - [🏗️ Custom Build Method](#️-custom-build-method)
  - [🧬 Generics Support](#-generics-support)
- [📋 Complete Attribute Reference](#-complete-attribute-reference)
- [🎨 Builder Patterns](#-builder-patterns)
- [🔍 Examples](#-examples)
- [❓ FAQ](#-faq)
- [🔧 How It Works](#-how-it-works)
- [🦀 Minimum Supported Rust Version](#-minimum-supported-rust-version)
- [📄 License](#-license)
- [🤝 Contributing](#-contributing)
- [📬 Support](#-support)

## 🚀 Why TypeStateBuilder?

Traditional builders can fail at runtime:

```rust,ignore
struct User {
    name: String,
    email: String,
    age: Option<u32>,
    phone: Option<String>,
}

// ❌ This compiles but panics at runtime!
let user = User::builder()
    .name("Alice")
    // Forgot to set required email!
    .build(); // 💥 Panic (or runtime error if builder returns a `Result`)!
```

With TypeStateBuilder, this becomes a **compile-time error**:

```rust,ignore
// ✅ This won't even compile!
let user = User::builder()
    .name("Alice")
    // .email("alice@example.com") <- Compiler error: missing required field!
    .build(); // ❌ Compile error, not runtime panic!
```

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
type-state-builder = "0.3.0"
```

## 🎯 Quick Start

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
#[builder(impl_into)]       // Enable ergonomic conversions for all setters
struct User {
    #[builder(required)]    // Method build() is unavailable until this field is set
    name: String,           // Setter accepts impl Into<String> for ergonomic usage

    #[builder(required)]    // Method build() is unavailable until this field is set
    email: String,          // Setter accepts impl Into<String> for ergonomic usage

    #[builder(
        converter = |age: u32| Some(age)),  // Custom conversion for setter age for ergonomic usage
        default = "18"                      // If not set, defaults to 18
    )]
    age: Option<u32>,

    #[converter = |phone: &str| Some(phone.to_string())] // Convert &str to Option<String>
    phone: Option<String>,                               // If not set, defaults to None (Default::default())

    #[builder(converter = |friends: Vec<&str>|           // Convert Vec<&str> to Vec<String>
        friends
        .into_iter()
        .map(|s| s.to_string())
        .collect())]
    friends: Vec<String>,                                // If not set, defaults to an empty vector (Default::default())
}

fn main() {
    // ✅ This works - all required fields are set
    let user = User::builder()
        .name("Alice")                      // &str -> String via Into
        .email("alice@example.com")         // &str -> String via Into
        .age(30)                            // u32 -> Option<u32> via converter
        .phone("123-456-7890")              // &str -> Option<String> via converter
        .friends(vec!["Bob", "Charlie"])    // Vec<&str> -> Vec<String> via converter
        .build();

    println!("{user:?}");

    // ✅ Optional fields can be omitted
    let user2 = User::builder()
        .name("Bob")                        // &str -> String via Into
        .email("bob@example.com")           // &str -> String via Into
        // age is optional, defaults to 18
        // phone is optional, defaults to None
        // friends is optional, defaults to an empty vector
        .build();                           // build() is available since all required fields are set

    println!("{user2:?}");
}
```

## 🛠️ Features

### ✨ Compile-Time Safety

The type-state pattern ensures that:

- **Required fields must be set** before calling `build()`
- **Missing required fields** cause friendly compile-time errors
- **Missing optional fields** default to their `Default` trait values or to the user-defined defaults
- **Ergonomic API** with `impl Into<T>` for setters, allowing you to pass `&str`, `&[T]`, etc. directly
- **Custom conversion logic** with `converter` for advanced transformations for setters
- **No runtime panics** due to incomplete builders
- **IDE support** with accurate autocomplete and error messages

### 🎛️ Field Types

#### Required Fields

Mark fields as required with `#[builder(required)]`:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Config {
    #[builder(required)]
    api_key: String,

    #[builder(required)]
    endpoint: String,

    timeout: Option<u32>, // Optional - uses None as default
}

// Must set both api_key and endpoint
let config = Config::builder()
    .api_key("key123".to_string())
    .endpoint("https://api.example.com".to_string())
    .timeout(Some(5000))
    .build();
```

#### Optional Fields with Custom Defaults

Provide custom default values for optional fields:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct DatabaseConfig {
    #[builder(required)]
    host: String,

    #[builder(default = "5432")]
    port: u16,

    #[builder(default = "10")]
    max_connections: u32,

    #[builder(default = "String::from(\"postgres\")")]
    database_name: String,
}

let config = DatabaseConfig::builder()
    .host("localhost".to_string())
    // port defaults to 5432
    // max_connections defaults to 10
    // database_name defaults to "postgres"
    .build();
```

#### Skip Setter Fields

Some fields should only use their default value and not have setters:

```rust
use uuid::Uuid;
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Document {
    #[builder(required)]
    title: String,

    #[builder(required)]
    content: String,

    #[builder(default = "Uuid::new_v4()", skip_setter)]
    id: Uuid,

    #[builder(default = "chrono::Utc::now()", skip_setter)]
    created_at: chrono::DateTime<chrono::Utc>,
}

let doc = Document::builder()
    .title("My Document".to_string())
    .content("Hello, world!".to_string())
    // id and created_at are auto-generated, no setters available
    .build();
```

### 🏷️ Custom Setter Names

Customize individual setter method names:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Person {
    #[builder(required, setter_name = "full_name")]
    name: String,

    #[builder(setter_name = "years_old")]
    age: Option<u32>,
}

let person = Person::builder()
    .full_name("Alice Smith".to_string())
    .years_old(Some(30))
    .build();
```

### 🎯 Setter Prefixes

Add consistent prefixes to all setter methods:

#### Struct-Level Prefix

Apply a prefix to all setters in the struct:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(setter_prefix = "with_")]
struct ServerConfig {
    #[builder(required)]
    host: String,

    #[builder(required)]
    port: u16,

    ssl_enabled: Option<bool>,
}

let config = ServerConfig::builder()
    .with_host("localhost".to_string())
    .with_port(8080)
    .with_ssl_enabled(Some(true))
    .build();
```

#### Field-Level Prefix Override

Field-level prefixes take precedence over struct-level prefixes:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(setter_prefix = "with_")]
struct ApiClient {
    #[builder(required)]
    base_url: String,

    #[builder(required, setter_prefix = "set_")]
    api_key: String,

    timeout: Option<u32>,
}

let client = ApiClient::builder()
    .with_base_url("https://api.example.com".to_string())
    .set_api_key("secret123".to_string()) // Field-level prefix wins
    .with_timeout(Some(5000))
    .build();
```

### 🔄 Ergonomic Conversions

The `impl_into` attribute generates setter methods that accept `impl Into<FieldType>` parameters, allowing for more
ergonomic API usage by automatically converting compatible types.

#### Struct-Level `impl_into`

Apply `impl_into` to all setters in the struct:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(impl_into)]
struct ApiClient {
    #[builder(required)]
    base_url: String,

    #[builder(required)]
    api_key: String,

    timeout: Option<u32>,
    user_agent: String, // Uses Default::default()
}

// Can now use &str directly instead of String::from() or .to_string()
let client = ApiClient::builder()
    .base_url("https://api.example.com")    // &str -> String
    .api_key("secret-key")                   // &str -> String
    .timeout(Some(30))
    .user_agent("MyApp/1.0")                 // &str -> String
    .build();
```

#### Field-Level Control with Precedence Rules

Field-level `impl_into` settings override struct-level defaults:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(impl_into)]  // Default for all fields
struct Document {
    #[builder(required)]
    title: String,  // Inherits impl_into = true

    #[builder(required, impl_into = false)]
    content: String,  // Override: requires String directly

    #[builder(impl_into = true)]
    category: Option<String>,  // Explicit impl_into = true

    #[builder(impl_into = false)]
    tags: Vec<String>,  // Override: requires Vec<String> directly
}

let doc = Document::builder()
    .title("My Document")                // &str -> String (inherited)
    .content("Content".to_string())      // Must use String (override)
    .category(Some("tech".to_string()))  // impl Into for Option<String>
    .tags(vec!["rust".to_string()])      // Must use Vec<String> (override)
    .build();
```

**Key Benefits:**

- ✅ **More ergonomic**: Use `"string"` instead of `"string".to_string()`
- ✅ **Flexible control**: Apply globally or selectively
- ✅ **Type safety**: Maintains compile-time guarantees while improving ergonomics
- ✅ **Zero cost**: Conversions happen at compile time

**Important Note:** `impl_into` is incompatible with `skip_setter` since skipped fields don't have setter methods
generated.

### 🔧 Custom Conversions with `converter`

The `converter` attribute provides powerful custom transformation logic for field setters, enabling more advanced
conversions than `impl_into` can provide. Unlike `impl_into` which relies on the `Into` trait, `converter` allows you to
specify arbitrary transformation logic using closure expressions.

#### Improved Ergonomics for Option Fields

One of the most useful converter applications is improving ergonomics for `Option<T>` fields:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
struct UserProfile {
    #[builder(required)]
    username: String,

    // Without converter: must use Some("value".to_string())
    bio: Option<String>,

    // With converter: can pass string literals directly
    #[builder(converter = |value: &str| Some(value.to_string()))]
    display_name: Option<String>,

    #[builder(converter = |value: &str| Some(value.to_string()))]
    location: Option<String>,
}

let profile = UserProfile::builder()
    .username("alice".to_string())
    .bio(Some("Software developer".to_string()))      // Verbose without converter
    .display_name("Alice Smith")                       // Clean with converter!
    .location("San Francisco")                         // Clean with converter!
    .build();

assert_eq!(profile.display_name, Some("Alice Smith".to_string()));
assert_eq!(profile.location, Some("San Francisco".to_string()));
```

#### Basic Converter Usage

Use closure syntax to define custom conversion logic with explicit parameter types:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
struct User {
    #[builder(required)]
    name: String,

    // Normalize email to lowercase and trim whitespace
    #[builder(required, converter = |email: &str| email.trim().to_lowercase())]
    email: String,

    // Parse comma-separated tags into Vec<String>
    #[builder(converter = |tags: &str| tags.split(',').map(|s| s.trim().to_string()).collect())]
    interests: Vec<String>,

    // Parse age from string with fallback
    #[builder(converter = |age_str: &str| age_str.parse().unwrap_or(0))]
    age: u32,
}

let user = User::builder()
    .name("Alice".to_string())
    .email("  ALICE@EXAMPLE.COM  ")  // Will be normalized to "alice@example.com"
    .interests("rust, programming, web")  // Parsed to Vec<String>
    .age("25")  // Parsed from string to u32
    .build();

assert_eq!(user.email, "alice@example.com");
assert_eq!(user.interests, vec!["rust", "programming", "web"]);
assert_eq!(user.age, 25);
```

#### Advanced Converter Examples

Converters support complex transformation logic:

```rust
use std::collections::HashMap;
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
struct Config {
    // Convert environment-style boolean strings
    #[builder(converter = |enabled: &str| {
        matches!(enabled.to_lowercase().as_str(), "true" | "1" | "yes" | "on")
    })]
    debug_enabled: bool,

    // Parse key=value pairs into HashMap
    #[builder(converter = |pairs: &str| {
        pairs.split(',')
             .filter_map(|pair| {
                 let mut split = pair.split('=');
                 Some((split.next()?.trim().to_string(),
                      split.next()?.trim().to_string()))
             })
             .collect()
    })]
    env_vars: HashMap<String, String>,

    // Transform slice to owned Vec
    #[builder(converter = |hosts: &[&str]| {
        hosts.iter().map(|s| s.to_string()).collect()
    })]
    allowed_hosts: Vec<String>,
}

let config = Config::builder()
    .debug_enabled("true")
    .env_vars("LOG_LEVEL=debug,PORT=8080")
    .allowed_hosts(&["localhost", "127.0.0.1"])
    .build();

assert_eq!(config.debug_enabled, true);
assert_eq!(config.env_vars.get("LOG_LEVEL"), Some(&"debug".to_string()));
assert_eq!(config.allowed_hosts, vec!["localhost", "127.0.0.1"]);
```

#### Converter vs impl_into Comparison

| Feature                     | `impl_into`       | `converter`          |
| --------------------------- | ----------------- | -------------------- |
| **Type conversions**        | Only `Into` trait | Any custom logic     |
| **Parsing strings**         | ❌ Limited        | ✅ Full support      |
| **Data validation**         | ❌ No             | ✅ Custom validation |
| **Complex transformations** | ❌ No             | ✅ Full support      |
| **Multiple input formats**  | ❌ Into only      | ✅ Any input type    |
| **Performance**             | Zero-cost         | Depends on logic     |
| **Syntax**                  | Attribute flag    | Closure expression   |

#### When to Use `converter`

**Use `converter` when:**

- ✅ **Improving Option<T> ergonomics**: `|value: &str| Some(value.to_string())` instead of `Some(value.to_string())`
- ✅ Parsing structured data from strings
- ✅ Normalizing or validating input data
- ✅ Complex data transformations
- ✅ Converting between incompatible types
- ✅ Custom business logic in setters

**Use `impl_into` when:**

- ✅ Simple type conversions (String/&str, PathBuf/&Path)
- ✅ The Into trait already provides the conversion you need
- ✅ You want zero-cost abstractions

#### Converter Syntax and Benefits

**Important:** The `converter` attribute requires closure expressions, **not function references**:

```rust,ignore
// ✅ Correct - inline closure expression
#[builder(converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
tags: Vec<String>,

// ❌ Incorrect - cannot reference external functions
// #[builder(converter = some_function)] // This doesn't work!
```

**Why closure expressions?**

The closure syntax provides several benefits:

- ✅ **IDE Support**: Full autocomplete, syntax highlighting, and type checking
- ✅ **Type Inference**: Parameter types are explicitly declared and validated
- ✅ **Compile-Time Validation**: Syntax errors caught immediately
- ✅ **Refactoring Safety**: Changes are tracked by your IDE and compiler
- ✅ **Documentation**: The conversion logic is visible at the field definition

#### Important Notes

- **Default values** must be passed as string literals: `#[builder(default = "Vec::new()")]`
- **Converter expressions** use closure syntax: `#[builder(converter = |param: Type| expression)]`
- `converter` is incompatible with `skip_setter` and `impl_into` (different approaches to setter generation)
- The closure parameter type must be explicitly specified for proper code generation

### 🏗️ Custom Build Method

Customize the name of the build method:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(build_method = "create")]
struct User {
    #[builder(required)]
    name: String,

    email: Option<String>,
}

let user = User::builder()
    .name("Alice".to_string())
    .create(); // Custom build method name
```

### 🧬 Generics Support

TypeStateBuilder works seamlessly with generic types, lifetimes, and complex bounds:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
struct Container<T, U>
where
    T: Clone + Send,
    U: std::fmt::Debug,
{
    #[builder(required)]
    primary: T,

    #[builder(required)]
    secondary: U,

    metadata: Option<String>,
}

let container = Container::<String, i32>::builder()
    .primary("Hello".to_string())
    .secondary(42)
    .metadata(Some("test".to_string()))
    .build();
```

#### Lifetime Support

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
struct Document<'a> {
    #[builder(required)]
    title: &'a str,

    #[builder(required)]
    content: &'a str,

    tags: Option<Vec<&'a str>>,
}

let title = "My Document";
let content = "Hello, world!";

let doc = Document::builder()
    .title(title)
    .content(content)
    .tags(Some(vec!["rust", "tutorial"]))
    .build();
```

### 💬 Developer-Friendly Error Messages

TypeStateBuilder generates descriptive type names that make compiler errors immediately actionable. Instead of cryptic
type names, you get clear guidance about what's missing:

#### Missing Required Fields

```rust,ignore
let user = User::builder()
    .name("Alice".to_string())
    .build(); // ❌ Trying to build without setting email
```

**Error message:**

```text
error[E0599]: no method named `build` found for struct `UserBuilder_HasName_MissingEmail`
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^
           Clear indication: name is set ✅, email is missing ❌
```

#### Multiple Missing Fields

```rust,ignore
let config = ServerConfig::builder()
    .build(); // ❌ Missing both required fields
```

**Error message:**

```text
error[E0599]: no method named `build` found for struct `ServerConfigBuilder_MissingHost_MissingPort`
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
           Shows exactly what needs to be set: host and port
```

These descriptive error messages help you:

- ✅ **See progress made**: `HasName` shows what's already set
- ✅ **Identify missing fields**: `MissingEmail` shows what's needed
- ✅ **Get immediate guidance**: No guessing what went wrong
- ✅ **IDE support**: Meaningful autocomplete and hover information

## 📋 Complete Attribute Reference

### Struct-Level Attributes

Applied to the struct itself:

```rust,ignore
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(
    build_method = "create",        // Custom build method name
    setter_prefix = "with_",        // Prefix for all setter methods
    impl_into                       // Generate setters with impl Into<T> parameters
)]
struct MyStruct { /* ... */ }
```

### Field-Level Attributes

Applied to individual fields:

```rust,ignore
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct MyStruct {
    // Available attributes (WARNING: not all combinations are valid):
    #[builder(
        required,                           // Mark field as required
        setter_name = "custom_name",        // Custom setter method name
        setter_prefix = "set_",             // Prefix for this setter (overrides struct-level)
        default = "42",                     // Custom default value expression (string literal)
        skip_setter,                        // Don't generate a setter method
        impl_into,                          // Generate setter with impl Into<T> parameter
        impl_into = false,                  // Override struct-level impl_into for this field
        converter = |param: Type| expr      // Custom conversion logic (closure expression)
    )]
    field: i32,
}
```

### Attribute Combinations

Some attributes work together, others are mutually exclusive:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Example {
    // ✅ Valid combinations
    #[builder(required, setter_name = "set_name")]
    name: String,

    #[builder(default = "42", setter_prefix = "with_")]
    count: i32,

    #[builder(default = "Uuid::new_v4()", skip_setter)]
    id: String,

    #[builder(converter = |value: &str| value.trim().to_string(), setter+prefix = "with_")]
    description: String,

    // ❌ Invalid combinations (compile errors)
    // #[builder(required, default = "0")]          // Required fields can't have defaults (ambiguous)
    // #[builder(required, skip_setter)]            // Required fields need setters (ambiguous)
    // #[builder(setter_prefix = "with_", skip_setter)] // Can't prefix skipped setters (ambiguous)
    // #[builder(impl_into, skip_setter)]           // Can't use impl_into with skipped setters (ambiguous)
    // #[builder(converter = |x| x, skip_setter)]   // Can't use converter with skipped setters (ambiguous)
    // #[builder(converter = |x| x, impl_into)]     // Can't use converter with impl_into (ambiguous)

    // ❌ Duplicate attributes (compile errors)
    // #[builder(required, required)]               // Duplicate 'required' attribute
    // #[builder(setter_name = "name1", setter_name = "name2")] // Duplicate 'setter_name'
    // #[builder(default = "1", default = "2")]     // Duplicate 'default' attribute
    // #[builder(converter = |x| x, converter = |y| y)] // Duplicate 'converter' attribute
}
```

#### Compilation Error Examples

**Duplicate Attributes:**

```rust,ignore
#[builder(setter_name = "first_name")]
#[builder(setter_name = "full_name")]  // ❌ Error!
name: String,
```

**Error message:**

```text
error: Duplicate setter_name attribute. Only one setter_name is allowed per field
```

**Conflicting Logic:**

```rust,ignore
#[builder(required, default = "empty")]  // ❌ Error!
name: String,
```

**Error message:**

```text
error: Required fields cannot have default values. Remove #[builder(default = "...")]
       or make the field optional by removing #[builder(required)]
```

**Incompatible Conversions:**

```rust,ignore
#[builder(impl_into, skip_setter)]  // ❌ Error!
field: String,
```

**Error message:**

```text
error: Field-level impl_into is incompatible with skip_setter. Remove one of these attributes.
```

## 🎨 Builder Patterns

TypeStateBuilder automatically chooses the best builder pattern based on your struct:

### Type-State Builder (Recommended)

Used when your struct has required fields. Provides compile-time safety:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct User {
    #[builder(required)]
    name: String,

    #[builder(required)]
    email: String,

    age: Option<u32>,
}

// Compile-time enforced order - must set required fields first
let user = User::builder()  // Initial state: neither field set
    .name("Alice".to_string())  // Transition: name is now set
    .email("alice@example.com".to_string())  // Transition: both fields set
    .age(Some(30))  // Optional fields can be set anytime
    .build();  // ✅ Now build() is available
```

### Regular Builder

Used when your struct has only optional fields. Immediate `build()` availability:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Config {
    timeout: Option<u32>, // Defaults to Default::default(), (`None` if not set)
    retries: u32, // Defaults to Default::default(), (`0` if not set)
    debug: bool, // Defaults to Default::default(), (`false` if not set)
}

// build() is available immediately since all fields are optional
let config = Config::builder()
    .timeout(Some(5000))
    .build();  // ✅ Can build anytime
```

## 🔍 Examples

### Web API Configuration

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
#[builder(setter_prefix = "with_")]
struct ApiConfig {
    #[builder(required)]
    base_url: String,

    #[builder(required)]
    api_key: String,

    #[builder(default = "30")]
    timeout_seconds: u32,

    #[builder(default = "3")]
    max_retries: u32,

    #[builder(default = "false")]
    debug_mode: bool,

    #[builder(default = "String::from(\"application/json\")", setter_name = "content_type")]
    accept_header: String,
}

fn main() {
    let config = ApiConfig::builder()
        .with_base_url("https://api.example.com".to_string())
        .with_api_key("secret123".to_string())
        .with_timeout_seconds(60)
        .content_type("application/xml".to_string())
        .build();

    println!("{:?}", config);
}
```

### Database Connection

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
#[builder(build_method = "connect")]
struct DatabaseConnection {
    #[builder(required)]
    host: String,

    #[builder(required)]
    database: String,

    #[builder(default = "5432")]
    port: u16,

    #[builder(default = "10")]
    max_connections: u32,

    username: Option<String>,
    password: Option<String>,

    #[builder(default = "true")]
    ssl_enabled: bool,

    #[builder(default = "chrono::Utc::now()", skip_setter)]
    created_at: chrono::DateTime<chrono::Utc>,
}

fn main() {
    let db = DatabaseConnection::builder()
        .host("localhost".to_string())
        .database("myapp".to_string())
        .port(5433)
        .username(Some("admin".to_string()))
        .password(Some("secret".to_string()))
        .connect();

    println!("Connected to: {db:?}");
}
```

### Generic Data Container

```rust
use type_state_builder::TypeStateBuilder;
use std::collections::HashMap;

#[derive(TypeStateBuilder, Debug)]
struct DataContainer<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    #[builder(required)]
    primary_data: HashMap<K, V>,

    #[builder(default = "HashMap::new()")]
    metadata: HashMap<String, String>,

    #[builder(default = "Vec::new()")]
    tags: Vec<String>,

    cached_count: Option<usize>,
}

fn main() {
    let mut data = HashMap::new();
    data.insert("key1".to_string(), 100i32);
    data.insert("key2".to_string(), 200i32);

    let container = DataContainer::<String, i32>::builder()
        .primary_data(data)
        .tags(vec!["important".to_string(), "cache".to_string()])
        .cached_count(Some(2))
        .build();

    println!("{:?}", container);
}
```

### Configuration Builder with Ergonomic Conversions

```rust
use type_state_builder::TypeStateBuilder;
use std::collections::HashMap;

#[derive(TypeStateBuilder, Debug)]
#[builder(impl_into, setter_prefix = "with_")]
struct AppConfig {
    #[builder(required)]
    app_name: String,

    #[builder(required)]
    version: String,

    #[builder(default = "8080")]
    port: u16,

    #[builder(default = "String::from(\"localhost\")")]
    host: String,

    #[builder(impl_into = false, setter_name = "environment_vars")]
    env_vars: HashMap<String, String>,

    #[builder(default = "Vec::new()")]
    features: Vec<String>,

    debug_mode: Option<bool>,

    #[builder(default = "chrono::Utc::now()", skip_setter)]
    created_at: chrono::DateTime<chrono::Utc>,
}

fn main() {
    // Ergonomic usage with impl_into
    let config = AppConfig::builder()
        .with_app_name("MyApp")              // &str -> String via Into
        .with_version("1.0.0")               // &str -> String via Into
        .with_port(3000)
        .with_host("0.0.0.0")                // &str -> String via Into
        .environment_vars({                  // Must use HashMap directly (impl_into = false)
            let mut vars = HashMap::new();
            vars.insert("RUST_LOG".to_string(), "debug".to_string());
            vars
        })
        .with_features(vec!["api".to_string(), "web".to_string()])
        .with_debug_mode(Some(true))
        .build();

    println!("App config: {config:?}");
}
```

## ❓ FAQ

### Q: When should I use TypeStateBuilder vs other builder libraries?

**A:** Use TypeStateBuilder when:

- ✅ You have required fields that must be set
- ✅ You want compile-time safety instead of runtime panics
- ✅ You're building configuration objects, API clients, or data structures
- ✅ You want zero runtime overhead

### Q: Does TypeStateBuilder work with existing Rust features?

**A:** Yes! TypeStateBuilder works with:

- ✅ Generic types and lifetime parameters
- ✅ Complex where clauses and trait bounds
- ✅ Derive macros like `Debug`, `Clone`, `PartialEq`
- ✅ Serde serialization/deserialization
- ✅ Custom implementations and methods

### Q: What's the performance impact?

**A:** Zero! TypeStateBuilder:

- ✅ Has **zero runtime overhead** - it's all compile-time
- ✅ Generates efficient code equivalent to manual builders
- ✅ Uses Rust's zero-cost abstractions
- ✅ The type-state transitions are compile-time only

### Q: Can I mix required and optional fields?

**A:** Absolutely! That's the main use case:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct MixedStruct {
    #[builder(required)] must_set: String,
    #[builder(required)] also_required: i32,
    optional_field: Option<String>,
    #[builder(default = "42")] with_default: u32,
}
```

### Q: What if I need to set fields conditionally?

**A:** Use optional fields and set them based on conditions:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct ConditionalConfig {
    #[builder(required)]
    mode: String,

    debug_info: Option<String>,
    prod_settings: Option<HashMap<String, String>>,
}

let is_debug = true;
let mut builder = ConditionalConfig::builder()
    .mode("production".to_string());

if is_debug {
    builder = builder.debug_info(Some("Debug enabled".to_string()));
}

let config = builder.build();
```

### Q: What's the difference between regular setters, `impl_into`, and `converter`?

**A:** Each provides different levels of flexibility and functionality:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Example {
    #[builder(required)]
    name: String,                    // Regular: exact type matching

    #[builder(required, impl_into)]
    title: String,                   // impl_into: Into trait conversions

    #[builder(required, converter = |email: &str| email.trim().to_lowercase())]
    email: String,                   // converter: custom transformation logic
}

let example = Example::builder()
    .name("Alice".to_string())       // Regular: must use String
    .title("Engineer")               // impl_into: can use &str via Into
    .email("  ALICE@EXAMPLE.COM  ")  // converter: custom normalization
    .build();

assert_eq!(example.email, "alice@example.com"); // Normalized!
```

**Comparison:**

| Feature              | Regular     | `impl_into`    | `converter`        |
| -------------------- | ----------- | -------------- | ------------------ |
| **Type flexibility** | Exact match | Into trait     | Any input type     |
| **Custom logic**     | ❌ No       | ❌ No          | ✅ Full support    |
| **Performance**      | Zero-cost   | Zero-cost      | Depends on logic   |
| **Syntax**           | Simple      | Attribute flag | Closure expression |

**When to use each:**

- **Regular**: When you want exact type control
- **`impl_into`**: For simple ergonomic conversions (String/&str, PathBuf/&Path)
- **`converter`**: For parsing, validation, normalization, or complex transformations

## 🔧 How It Works

TypeStateBuilder uses Rust's powerful type system to create a **compile-time state machine**. Here's the magic:

### The Type-State Pattern

When you have required fields, TypeStateBuilder generates multiple builder types representing different states. Consider
this example struct:

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct User {
    #[builder(required)]
    name: String,

    #[builder(required)]
    email: String,

    age: Option<u32>,
}
```

For this struct with 2 required fields, TypeStateBuilder generates states like:

```rust
// Different builder states with descriptive type names:
// - UserBuilder_MissingName_MissingEmail: Neither field set
// - UserBuilder_HasName_MissingEmail: Name set, email missing
// - UserBuilder_MissingName_HasEmail: Email set, name missing
// - UserBuilder_HasName_HasEmail: Both fields set (can build!)
```

### Setter Method Magic

Each setter method transitions between states:

```rust,ignore
impl UserBuilder_MissingName_MissingEmail {
    // Can set either field from initial state
    pub fn name(self, value: String) -> UserBuilder_HasName_MissingEmail { /* ... */ }
    pub fn email(self, value: String) -> UserBuilder_MissingName_HasEmail { /* ... */ }
}

impl UserBuilder_HasName_HasEmail {
    // Only this final state has the build() method!
    pub fn build(self) -> User { /* ... */ }
}
```

### Smart Builder Selection

TypeStateBuilder automatically chooses the right pattern:

- **Type-State Builder**: When you have required fields (compile-time safety)
- **Regular Builder**: When all fields are optional (immediate `build()` availability)

### Zero Runtime Cost

All the type-state magic happens at compile time:

- ✅ No runtime state tracking
- ✅ No runtime validation
- ✅ No performance overhead
- ✅ Generated code is as efficient as hand-written builders

The end result? You get the safety of compile-time validation with the performance of hand-optimized code!

## 🦀 Minimum Supported Rust Version

TypeStateBuilder supports Rust **1.70.0** and later.

The MSRV is checked in our CI and we will not bump it without a minor version release. We reserve the right to bump the
MSRV in minor releases if required by dependencies or to enable significant improvements.

**Why 1.70.0?**

- Required for stable proc-macro features used in code generation
- Needed for advanced generic parameter handling
- Ensures compatibility with modern Rust ecosystem

---

## 📄 License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT
license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to
discuss what you would like to change.

## 📬 Support

If you have questions or need help, please:

- 📖 Check the [documentation](https://docs.rs/type-state-builder)
- 🐛 [Open an issue](https://github.com/welf/type-state-builder/issues) for bugs
- 💡 [Start a discussion](https://github.com/welf/type-state-builder/discussions) for questions

---

**Happy Building! 🚀**
