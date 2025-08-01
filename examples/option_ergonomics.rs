use type_state_builder::TypeStateBuilder;

/// Example demonstrating improved ergonomics for Option<T> fields using converters
///
/// Without converters, Option fields require verbose Some(value.to_string()) calls.
/// With converters, you can pass values directly and the converter wraps them in Some().

#[derive(TypeStateBuilder, Debug, PartialEq)]
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

    // Converter for other Option types
    #[builder(converter = |value: u32| Some(value))]
    age: Option<u32>,

    // Even works with complex types
    #[builder(converter = |tags: Vec<&str>| Some(tags.into_iter().map(|s| s.to_string()).collect()))]
    interests: Option<Vec<String>>,
}

#[derive(TypeStateBuilder, Debug, PartialEq)]
struct ServerConfig {
    #[builder(required)]
    host: String,

    #[builder(required)]
    port: u16,

    // Database URL is optional but when provided, should be clean
    #[builder(converter = |url: &str| Some(url.to_string()))]
    database_url: Option<String>,

    // SSL certificate path
    #[builder(converter = |path: &str| Some(path.to_string()))]
    ssl_cert_path: Option<String>,

    // Worker count with validation
    #[builder(converter = |count: u32| Some(count.clamp(1, 100)))]
    worker_count: Option<u32>,
}

fn main() {
    println!("=== Option Ergonomics with Converters ===");

    // Compare verbose vs clean syntax
    let profile_verbose = UserProfile::builder()
        .username("alice".to_string())
        .bio(Some("Software developer".to_string())) // Verbose!
        .display_name("Alice Smith") // Clean with converter!
        .location("San Francisco") // Clean with converter!
        .age(29) // Clean with converter!
        .interests(vec!["rust", "programming"]) // Clean with converter!
        .build();

    let profile_clean = UserProfile::builder()
        .username("bob".to_string())
        .bio(Some("Product manager".to_string())) // Still verbose (no converter)
        .display_name("Bob Johnson") // Clean with converter!
        .location("New York") // Clean with converter!
        .age(35) // Clean with converter!
        .interests(vec!["management", "strategy"]) // Clean with converter!
        .build();

    println!("Verbose profile: {profile_verbose:#?}");
    println!("Clean profile: {profile_clean:#?}");

    // Verify the values are correct
    assert_eq!(profile_clean.display_name, Some("Bob Johnson".to_string()));
    assert_eq!(profile_clean.location, Some("New York".to_string()));
    assert_eq!(profile_clean.age, Some(35));
    assert_eq!(
        profile_clean.interests,
        Some(vec!["management".to_string(), "strategy".to_string()])
    );

    println!("\n=== Server Configuration ===");

    let config = ServerConfig::builder()
        .host("localhost".to_string())
        .port(8080)
        .database_url("postgresql://user:pass@localhost/mydb") // Clean!
        .ssl_cert_path("/etc/ssl/cert.pem") // Clean!
        .worker_count(150) // Will be clamped to 100 by the converter
        .build();

    println!("Server config: {config:#?}");

    assert_eq!(
        config.database_url,
        Some("postgresql://user:pass@localhost/mydb".to_string())
    );
    assert_eq!(config.ssl_cert_path, Some("/etc/ssl/cert.pem".to_string()));
    assert_eq!(config.worker_count, Some(100)); // Clamped from 150 to 100

    println!("\n=== Benefits Summary ===");
    println!("✅ No more verbose Some(value.to_string()) calls");
    println!("✅ Direct value passing with automatic Option wrapping");
    println!("✅ Can combine with validation logic (like worker_count clamping)");
    println!("✅ Works with any type that implements Into<T>");
    println!("✅ Maintains type safety and compile-time validation");

    println!("\n✓ Option ergonomics example completed successfully!");
}
