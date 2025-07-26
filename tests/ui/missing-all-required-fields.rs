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
    // This should fail to compile because no required fields are set
    let config = Config::builder()
        .timeout(Some(5000))
        .build(); // Error: missing required fields 'api_key' and 'endpoint'
}