use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Config {
    #[builder(required)]
    api_key: String,
    
    #[builder(required)]
    endpoint: String,
    
    timeout: Option<u32>,
}

fn main() {
    // This should fail: trying to build without setting any required fields
    let config = Config::builder().build(); // Error: missing required fields 'api_key' and 'endpoint'
}

