# TypeStateBuilder

[![Crates.io](https://img.shields.io/crates/v/type-state-builder.svg)](https://crates.io/crates/type-state-builder)
[![Documentation](https://docs.rs/type-state-builder/badge.svg)](https://docs.rs/type-state-builder)
[![CI](https://github.com/welf/type-state-builder/workflows/CI/badge.svg)](https://github.com/welf/type-state-builder/actions)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**TypeStateBuilder** is a Rust procedural macro that generates compile-time safe builders using the **type-state
pattern**. It prevents runtime errors by making it impossible to build incomplete objects, while providing an ergonomic
and intuitive API.

## 📚 Table of Contents

- [🚀 Why TypeStateBuilder?](#-why-typestatebuilder)
- [📦 Installation](#-installation)
- [🎯 Quick Start](#-quick-start)
- [🛠️ Features](#️-features)
  - [✨ Compile-Time Safety](#-compile-time-safety)
  - [🎛️ Field Types](#️-field-types)
  - [🏷️ Custom Setter Names](#️-custom-setter-names)
  - [🎯 Setter Prefixes](#-setter-prefixes)
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
type-state-builder = "0.1.0"
```

## 🎯 Quick Start

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug)]
struct User {
    #[builder(required)]
    name: String,

    #[builder(required)]
    email: String,

    age: Option<u32>, // If not set, defaults to None
    phone: Option<String>, // If not set, defaults to None
    friends: Vec<String>, // If not set, defaults to an empty vector
}

fn main() {
    // ✅ This works - all required fields are set
    let user = User::builder()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(Some(30))
        .friends(vec!["Bob".to_string(), "Charlie".to_string()])
        .build();

    println!("{user:?}");

    // ✅ Optional fields can be omitted
    let user2 = User::builder()
        .name("Bob".to_string())
        .email("bob@example.com".to_string())
        .build();

    println!("{user2:?}");
}
```

## 🛠️ Features

### ✨ Compile-Time Safety

The type-state pattern ensures that:

- **Required fields must be set** before calling `build()`
- **Missing required fields** cause compile-time errors
- **Missing optional fields** default to their `Default` trait values or to the user-defined defaults
- **No runtime panics** due to incomplete builders
- **IDE support** with accurate autocomplete and error messages

### 🎛️ Field Types

#### Required Fields

Mark fields as required with `#[builder(required)]`:

```rust
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

### 🏗️ Custom Build Method

Customize the name of the build method:

```rust
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
#[derive(TypeStateBuilder)]
#[builder(
    build_method = "create",        // Custom build method name
    setter_prefix = "with_"         // Prefix for all setter methods
)]
struct MyStruct { /* ... */ }
```

### Field-Level Attributes

Applied to individual fields:

```rust,ignore
#[derive(TypeStateBuilder)]
struct MyStruct {
    // Available attributes (WARNING: not all combinations are valid):
    #[builder(
        required,                           // Mark field as required
        setter_name = "custom_name",        // Custom setter method name
        setter_prefix = "set_",             // Prefix for this setter (overrides struct-level)
        default = "42",                     // Custom default value expression
        skip_setter                         // Don't generate a setter method
    )]
    field: i32,
}
```

### Attribute Combinations

Some attributes work together, others are mutually exclusive:

```rust
#[derive(TypeStateBuilder)]
struct Example {
    // ✅ Valid combinations
    #[builder(required, setter_name = "set_name")]
    name: String,

    #[builder(default = "42", setter_prefix = "with_")]
    count: i32,

    #[builder(default = "Uuid::new_v4()", skip_setter)]
    id: String,

    // ❌ Invalid combinations (compile errors)
    // #[builder(required, default = "0")]          // Required fields can't have defaults
    // #[builder(required, skip_setter)]            // Required fields need setters
    // #[builder(setter_prefix = "with_", skip_setter)] // Can't prefix skipped setters

    // ❌ Duplicate attributes (compile errors)
    // #[builder(required, required)]               // Duplicate 'required' attribute
    // #[builder(setter_name = "name1", setter_name = "name2")] // Duplicate 'setter_name'
    // #[builder(default = "1", default = "2")]     // Duplicate 'default' attribute
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

## 🎨 Builder Patterns

TypeStateBuilder automatically chooses the best builder pattern based on your struct:

### Type-State Builder (Recommended)

Used when your struct has required fields. Provides compile-time safety:

```rust
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

## 🔧 How It Works

TypeStateBuilder uses Rust's powerful type system to create a **compile-time state machine**. Here's the magic:

### The Type-State Pattern

When you have required fields, TypeStateBuilder generates multiple builder types representing different states. Consider
this example struct:

```rust
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
