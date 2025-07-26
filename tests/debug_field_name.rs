//! Debug test to see what field names we get

use type_state_builder::TypeStateBuilder;

#[test]
fn test_normal_field_name() {
    #[derive(TypeStateBuilder)]
    struct NormalStruct {
        #[builder(required)]
        normal_field: String,
    }

    let instance = NormalStruct::builder()
        .normal_field("test".to_string())
        .build();

    assert_eq!(instance.normal_field, "test");
}
