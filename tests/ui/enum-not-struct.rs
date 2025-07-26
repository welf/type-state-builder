use type_state_builder::TypeStateBuilder;

// This should be an error: enums are not supported, only structs
#[derive(TypeStateBuilder)]
enum NotAStruct {
    Variant1(String),
    Variant2 { field: u32 },
}

fn main() {
    let _instance = NotAStruct::builder()
        .build();
}