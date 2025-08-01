use type_state_builder::TypeStateBuilder;

/// Example demonstrating the converter attribute for custom field transformations
///
/// The converter attribute allows you to specify custom conversion logic when setting
/// field values, enabling ergonomic APIs while maintaining type safety.

#[derive(TypeStateBuilder, Debug, PartialEq)]
struct User {
    #[builder(required)]
    name: String,

    #[builder(required)]
    email: String,

    // Convert a slice of string references to a Vec<String>
    #[builder(converter = |tags: &[&str]| tags.iter().map(|s| s.to_string()).collect())]
    tags: Vec<String>,

    // Convert comma-separated string to Vec<String>
    #[builder(converter = |skills: &str| skills.split(',').map(|s| s.trim().to_string()).collect())]
    skills: Vec<String>,

    // Transform age from string to number with validation
    #[builder(converter = |age_str: &str| age_str.parse::<u32>().unwrap_or(0))]
    age: u32,

    // Normalize email to lowercase
    #[builder(converter = |email: String| email.to_lowercase())]
    normalized_email: String,
}

#[derive(TypeStateBuilder, Debug)]
struct Config {
    // Convert environment variable-style string to boolean
    #[builder(converter = |enabled: &str| matches!(enabled.to_lowercase().as_str(), "true" | "1" | "yes" | "on"))]
    debug_enabled: bool,

    // Parse comma-separated hosts into Vec
    #[builder(converter = |hosts: &str| hosts.split(',').map(|h| h.trim().to_string()).collect())]
    allowed_hosts: Vec<String>,

    #[builder(default = "8080")]
    port: u16,
}

fn main() {
    // Demonstrate User builder with converters
    let user = User::builder()
        .name("Alice Smith".to_string())
        .email("ALICE@EXAMPLE.COM".to_string())
        .tags(&["developer", "rust", "backend"]) // &[&str] converted to Vec<String>
        .skills("Rust,Python,JavaScript") // &str converted to Vec<String>
        .age("29") // &str converted to u32
        .normalized_email("ALICE@EXAMPLE.COM".to_string()) // String converted to lowercase
        .build();

    println!("User: {user:#?}");

    assert_eq!(user.name, "Alice Smith");
    assert_eq!(user.email, "ALICE@EXAMPLE.COM");
    assert_eq!(user.tags, vec!["developer", "rust", "backend"]);
    assert_eq!(user.skills, vec!["Rust", "Python", "JavaScript"]);
    assert_eq!(user.age, 29);
    assert_eq!(user.normalized_email, "alice@example.com");

    // Demonstrate Config builder with converters
    let config = Config::builder()
        .debug_enabled("true") // &str converted to bool
        .allowed_hosts("localhost, 127.0.0.1") // &str converted to Vec<String>
        .build();

    println!("\nConfig: {config:#?}");

    assert!(config.debug_enabled);
    assert_eq!(config.allowed_hosts, vec!["localhost", "127.0.0.1"]);
    assert_eq!(config.port, 8080);

    // Show different boolean conversion options
    let configs = [
        Config::builder()
            .debug_enabled("true")
            .allowed_hosts("")
            .build(),
        Config::builder()
            .debug_enabled("1")
            .allowed_hosts("")
            .build(),
        Config::builder()
            .debug_enabled("yes")
            .allowed_hosts("")
            .build(),
        Config::builder()
            .debug_enabled("on")
            .allowed_hosts("")
            .build(),
        Config::builder()
            .debug_enabled("false")
            .allowed_hosts("")
            .build(),
        Config::builder()
            .debug_enabled("0")
            .allowed_hosts("")
            .build(),
    ];

    for (i, config) in configs.iter().enumerate() {
        println!("Config {}: debug_enabled = {}", i + 1, config.debug_enabled);
    }

    println!("\n✓ All converter examples completed successfully!");
}
