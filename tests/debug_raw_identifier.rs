//! Debug test for raw identifier issue

use type_state_builder::TypeStateBuilder;

#[test]
fn test_single_raw_identifier() {
    #[derive(TypeStateBuilder)]
    struct TestStruct {
        #[builder(required)]
        r#type: String,
    }

    let instance = TestStruct::builder().r#type("test".to_string()).build();

    assert_eq!(instance.r#type, "test");
}
