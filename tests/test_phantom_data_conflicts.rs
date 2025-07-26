// Test PhantomData field name conflict resolution

use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
struct StructWithMarkerField<'a> {
    #[builder(required)]
    data: &'a str,

    // This conflicts with the default PhantomData field name
    _marker: i32,

    optional_value: u32,
}

#[test]
fn test_phantom_data_marker_conflict() {
    let text = "hello world";
    let instance = StructWithMarkerField::builder()
        .data(text)
        .optional_value(42)
        .build();

    assert_eq!(instance.data, "hello world");
    assert_eq!(instance._marker, 0); // Default value for i32
    assert_eq!(instance.optional_value, 42);
}

#[derive(TypeStateBuilder)]
struct StructWithMultipleMarkerConflicts<T> {
    #[builder(required)]
    value: T,

    // Multiple potential conflicts
    _marker: String,
    _marker_123: f64,
}

#[test]
fn test_multiple_marker_conflicts() {
    let instance = StructWithMultipleMarkerConflicts::<i32>::builder()
        .value(42)
        .build();

    assert_eq!(instance.value, 42);
    assert_eq!(instance._marker, ""); // Default value for String
    assert_eq!(instance._marker_123, 0.0); // Default value for f64
}
