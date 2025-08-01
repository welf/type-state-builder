//! Demonstration of the `impl_into` attribute for ergonomic setter methods.
//!
//! This example showcases how the `impl_into` attribute can make builder APIs
//! more ergonomic by allowing automatic type conversions via the `Into` trait.

use std::collections::HashMap;
use std::path::PathBuf;
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder, Debug, PartialEq)]
#[builder(impl_into)] // Apply impl_into to all setters
struct WebServerConfig {
    #[builder(required)]
    host: String, // Can accept &str via Into

    #[builder(required)]
    port: u16,

    #[builder(default = "String::from(\"web\")")]
    name: String, // Can accept &str via Into

    #[builder(default = "PathBuf::from(\"/var/log/app.log\")")]
    log_file: PathBuf, // Can accept &str, &Path via Into

    #[builder(default = "Vec::new()")]
    allowed_origins: Vec<String>, // Can accept &str slice via Into

    #[builder(impl_into = false)] // Override: must use HashMap directly
    headers: HashMap<String, String>,

    ssl_enabled: Option<bool>,
}

#[derive(TypeStateBuilder, Debug, PartialEq)]
struct UserProfile {
    #[builder(required, impl_into)] // Explicit impl_into for required field
    username: String,

    #[builder(required)] // No impl_into: must use String directly
    user_id: String,

    #[builder(impl_into = true)] // Explicit impl_into for optional field
    display_name: Option<String>,

    #[builder(default = "Vec::new()", impl_into = false)] // Override: no impl_into
    tags: Vec<String>,

    #[builder(default = "false")]
    is_admin: bool,
}

fn main() {
    println!("=== WebServerConfig with struct-level impl_into ===");

    // Ergonomic usage with automatic conversions
    let server_config = WebServerConfig::builder()
        .host("localhost") // &str -> String
        .port(8080u16) // u16 literal
        .name("api-server") // &str -> String
        .log_file("/tmp/server.log") // &str -> PathBuf
        .allowed_origins(vec!["https://example.com".to_string()])
        .headers({
            // Must use HashMap (impl_into = false)
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("X-API-Version".to_string(), "1.0".to_string());
            headers
        })
        .ssl_enabled(Some(true))
        .build();

    println!("{server_config:#?}");

    // Test that the server config was built correctly
    assert_eq!(server_config.host, "localhost");
    assert_eq!(server_config.port, 8080u16);
    assert_eq!(server_config.name, "api-server");
    assert_eq!(server_config.log_file, PathBuf::from("/tmp/server.log"));
    assert_eq!(server_config.allowed_origins, vec!["https://example.com"]);
    assert_eq!(
        server_config.headers.get("Content-Type"),
        Some(&"application/json".to_string())
    );
    assert_eq!(server_config.ssl_enabled, Some(true));

    println!("\n=== UserProfile with mixed impl_into settings ===");

    let profile = UserProfile::builder()
        .username("alice") // &str -> String (explicit impl_into)
        .user_id("user_12345".to_string()) // Must use String (no impl_into)
        .display_name(Some("Alice Smith".to_string())) // Must use Option<String>
        .tags(vec!["developer".to_string(), "rust".to_string()]) // Must use Vec<String>
        .is_admin(false)
        .build();

    println!("{profile:#?}");

    // Test that the user profile was built correctly
    assert_eq!(profile.username, "alice");
    assert_eq!(profile.user_id, "user_12345");
    assert_eq!(profile.display_name, Some("Alice Smith".to_string()));
    assert_eq!(profile.tags, vec!["developer", "rust"]);
    assert!(!profile.is_admin);

    println!("\n=== Demonstrating type flexibility ===");

    // Show different ways to provide values
    let flexible_config = WebServerConfig::builder()
        .host("0.0.0.0".to_string()) // String -> String (also works)
        .port(3000u16) // u16 literal
        .name("flexible-server") // &str -> String
        .log_file(PathBuf::from("/custom/path.log")) // PathBuf -> PathBuf (also works)
        .allowed_origins(vec!["*".to_string()])
        .headers(HashMap::new()) // Empty HashMap
        .build();

    println!("{flexible_config:#?}");

    // Test the flexible config
    assert_eq!(flexible_config.host, "0.0.0.0");
    assert_eq!(flexible_config.port, 3000u16);
    assert_eq!(flexible_config.name, "flexible-server");
    assert_eq!(flexible_config.log_file, PathBuf::from("/custom/path.log"));
    assert_eq!(flexible_config.allowed_origins, vec!["*"]);
    assert!(flexible_config.headers.is_empty());
    assert_eq!(flexible_config.ssl_enabled, None);

    println!("\n=== Benefits of impl_into ===");
    println!("✅ More ergonomic: Use '\"string\"' instead of '\"string\".to_string()'");
    println!("✅ Flexible: Accepts String, &str, Cow<str>, etc.");
    println!("✅ Zero cost: Conversions happen at compile time");
    println!("✅ Type safe: Only accepts types that implement Into<FieldType>");
    println!("✅ Optional override: Field-level settings can override struct-level defaults");

    // Demonstrate converter for advanced transformations beyond impl_into
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    struct ContactInfo {
        #[builder(required, impl_into)]
        name: String, // impl_into: accepts &str, String, etc.

        // converter allows custom transformation logic that impl_into cannot provide
        #[builder(required, converter = |email: &str| email.trim().to_lowercase())]
        email: String, // Normalize email (trim + lowercase)

        // Parse comma-separated tags - impl_into can't parse strings like this
        #[builder(converter = |tags: &str| tags.split(',').map(|s| s.trim().to_string()).collect())]
        tags: Vec<String>,

        // Parse age from string - more flexible than impl_into
        #[builder(converter = |age: &str| age.parse().unwrap_or(0))]
        age: u32,
    }

    println!("\n=== Converter vs impl_into comparison ===");

    let contact = ContactInfo::builder()
        .name("Alice") // impl_into: &str -> String
        .email("  ALICE@EXAMPLE.COM  ") // converter: normalize email
        .tags("developer, rust, backend") // converter: parse comma-separated
        .age("25") // converter: parse string to number
        .build();

    println!("{contact:#?}");

    assert_eq!(contact.name, "Alice");
    assert_eq!(contact.email, "alice@example.com"); // Normalized!
    assert_eq!(contact.tags, vec!["developer", "rust", "backend"]); // Parsed!
    assert_eq!(contact.age, 25); // Converted from string!

    println!("\n=== When to use impl_into vs converter ===");
    println!("📋 Use `impl_into` when:");
    println!("   • You want simple type conversions (String/&str, PathBuf/&Path)");
    println!("   • The Into trait already provides the conversion you need");
    println!("   • You want zero-cost abstractions");
    println!("   • You want zero-cost abstractions\n");
    println!("🔧 Use `converter` when:");
    println!("   • You need custom transformation logic (parsing, validation, normalization)");
    println!("   • You want to parse structured data from strings");
    println!("   • You need conversions that Into trait cannot provide");
}
