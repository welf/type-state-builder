use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(build_method = "create")]
struct Document {
    #[builder(required)]
    title: String,

    #[builder(required, setter_name = "set_content")]
    content: String,

    #[builder(default = 42)]
    page_count: u32,

    #[builder(default = String::from("draft"), skip_setter)]
    status: String,
}

fn main() {
    let doc = Document::builder()
        .title("My Document".to_string())
        .set_content("Document content here".to_string())
        .page_count(100)
        .create(); // Custom build method name

    println!("Created document: {}", doc.title);
    println!("Content: {}", doc.content);
    println!("Pages: {}, Status: {}", doc.page_count, doc.status);
}
