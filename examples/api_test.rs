use type_state_builder::TypeStateBuilder;

// Test basic functionality - required and optional fields
#[derive(TypeStateBuilder)]
struct Person {
    #[builder(required)]
    name: String,

    #[builder(required)]
    age: u32,

    email: Option<String>,
    phone: Option<String>,
}

// Test with generic types
#[derive(TypeStateBuilder)]
struct Container<T> {
    #[builder(required)]
    value: T,

    label: String,
}

// Test with lifetimes
#[derive(TypeStateBuilder)]
struct Document<'a> {
    #[builder(required)]
    title: &'a str,

    content: Option<&'a str>,
}

// Test with custom defaults and skip setter
#[derive(TypeStateBuilder)]
struct Config {
    #[builder(required)]
    host: String,

    #[builder(default = "8080")]
    port: u16,

    #[builder(default = "String::from(\"auto-generated\")", skip_setter)]
    id: String,

    debug: bool,
}

// Test with custom setter names
#[derive(TypeStateBuilder)]
struct HttpClient {
    #[builder(required, setter_name = "with_url")]
    base_url: String,

    #[builder(setter_name = "with_timeout")]
    timeout_seconds: Option<u32>,
}

// Test with custom build method name
#[derive(TypeStateBuilder)]
#[builder(build_method = "create")]
struct Database {
    #[builder(required)]
    connection_string: String,

    max_connections: Option<u32>,
}

fn main() {
    // Test basic usage
    let person = Person::builder()
        .name("John Doe".to_string())
        .age(30)
        .email(Some("john@example.com".to_string()))
        .phone(Some("555-0123".to_string()))
        .build();

    println!("Person: {} ({})", person.name, person.age);
    assert_eq!(person.email, Some("john@example.com".to_string()));
    assert_eq!(person.phone, Some("555-0123".to_string()));

    // Test generic usage
    let container = Container::<i32>::builder()
        .value(42)
        .label("Answer".to_string())
        .build();

    println!("Container: {} = {}", container.label, container.value);

    // Test with lifetimes
    let title = "My Document";
    let content = "Some content here";
    let document = Document::builder()
        .title(title)
        .content(Some(content))
        .build();

    println!("Document: {}", document.title);
    assert_eq!(document.content, Some("Some content here"));

    // Test with custom defaults
    let config = Config::builder()
        .host("localhost".to_string())
        .debug(true)
        .build();

    println!("Config: {}:{}", config.host, config.port);
    assert!(config.debug);
    assert!(!config.id.is_empty()); // Check that id has default value

    // Test with custom setter names
    let client = HttpClient::builder()
        .with_url("https://api.example.com".to_string())
        .with_timeout(Some(30))
        .build();

    println!("HTTP Client: {}", client.base_url);
    assert_eq!(client.timeout_seconds, Some(30));

    // Test with custom build method
    let db = Database::builder()
        .connection_string("postgresql://localhost/mydb".to_string())
        .max_connections(Some(10))
        .create();

    println!("Database: {}", db.connection_string);
    assert_eq!(db.max_connections, Some(10));
}
