use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Config {
    name: Option<String>,

    #[builder(default = "8080")]
    port: u16,

    #[builder(default = "false")]
    debug: bool,
}

fn main() {
    // All these are valid - no required fields
    let config1 = Config::builder().build();
    println!(
        "Config 1 - Port: {}, Debug: {}",
        config1.port, config1.debug
    );

    let config2 = Config::builder().name(Some("test".to_string())).build();
    println!("Config 2 - Name: {:?}", config2.name);

    let config3 = Config::builder().port(3000).debug(true).build();
    println!(
        "Config 3 - Port: {}, Debug: {}",
        config3.port, config3.debug
    );
}
