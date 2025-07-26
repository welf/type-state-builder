use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct Container<T: Clone>
where
    T: Send,
{
    #[builder(required)]
    value: T,

    #[builder(required)]
    name: String,

    tags: Vec<String>,
}

fn main() {
    let container = Container::builder()
        .value(42)
        .name("test".to_string())
        .tags(vec!["tag1".to_string()])
        .build();

    println!(
        "Container '{}' has value: {}",
        container.name, container.value
    );
    println!("Tags: {:?}", container.tags);
}
