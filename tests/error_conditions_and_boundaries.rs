//! Error Conditions and Boundary Case Testing
//!
//! This file tests various error conditions and boundary cases to ensure
//! our macro provides helpful error messages and handles edge cases gracefully.

use type_state_builder::TypeStateBuilder;

// ===== VALID BOUNDARY CASES =====

#[test]
fn test_struct_with_single_required_field() {
    #[derive(TypeStateBuilder)]
    struct SingleRequired {
        #[builder(required)]
        field: String,
    }

    let instance = SingleRequired::builder().field("test".to_string()).build();

    assert_eq!(instance.field, "test");
}

#[test]
fn test_struct_with_single_optional_field() {
    #[derive(TypeStateBuilder)]
    struct SingleOptional {
        field: Option<String>,
    }

    let instance = SingleOptional::builder()
        .field(Some("test".to_string()))
        .build();

    assert_eq!(instance.field, Some("test".to_string()));
}

#[test]
fn test_struct_with_only_skip_setter_fields() {
    #[derive(TypeStateBuilder)]
    struct OnlySkipSetter {
        #[builder(skip_setter, default = "\"auto1\".to_string()")]
        field1: String,

        #[builder(skip_setter, default = "42")]
        field2: i32,

        #[builder(skip_setter, default = "true")]
        field3: bool,
    }

    // Should work as regular builder since no required fields
    let instance = OnlySkipSetter::builder().build();

    assert_eq!(instance.field1, "auto1");
    assert_eq!(instance.field2, 42);
    assert!(instance.field3);
}

// ===== COMPLEX VALID SCENARIOS =====

#[test]
fn test_struct_with_very_long_names() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "construct_with_very_long_method_name")]
    struct StructWithVeryLongNameAndManyWordsForTesting {
        #[builder(required, setter_name = "set_very_long_field_name_that_tests_naming")]
        very_long_field_name_that_should_work_fine: String,

        #[builder(default = "\"very_long_default_value_for_testing_purposes\".to_string()")]
        another_very_long_optional_field_name: String,
    }

    let instance = StructWithVeryLongNameAndManyWordsForTesting::builder()
        .set_very_long_field_name_that_tests_naming("test".to_string())
        .construct_with_very_long_method_name();

    assert_eq!(instance.very_long_field_name_that_should_work_fine, "test");
    assert_eq!(
        instance.another_very_long_optional_field_name,
        "very_long_default_value_for_testing_purposes"
    );
}

#[test]
fn test_struct_with_keywords_in_names() {
    #[derive(TypeStateBuilder)]
    struct StructWithKeywords {
        #[builder(required)]
        r#type: String, // 'type' is a keyword

        #[builder(required)]
        r#match: i32, // 'match' is a keyword

        r#impl: Option<bool>, // 'impl' is a keyword
    }

    let instance = StructWithKeywords::builder()
        .r#type("test".to_string())
        .r#match(42)
        .r#impl(Some(true))
        .build();

    assert_eq!(instance.r#type, "test");
    assert_eq!(instance.r#match, 42);
    assert_eq!(instance.r#impl, Some(true));
}

// ===== NUMERIC AND BOUNDARY VALUE TESTS =====

#[test]
fn test_numeric_boundary_values() {
    #[derive(TypeStateBuilder)]
    struct NumericBoundaries {
        #[builder(required)]
        max_i64: i64,

        #[builder(required)]
        min_i64: i64,

        #[builder(default = "0")]
        zero_value: i32,

        #[builder(default = "f64::INFINITY")]
        infinity: f64,

        #[builder(default = "f64::NEG_INFINITY")]
        neg_infinity: f64,
    }

    let instance = NumericBoundaries::builder()
        .max_i64(i64::MAX)
        .min_i64(i64::MIN)
        .build();

    assert_eq!(instance.max_i64, i64::MAX);
    assert_eq!(instance.min_i64, i64::MIN);
    assert_eq!(instance.zero_value, 0);
    assert_eq!(instance.infinity, f64::INFINITY);
    assert_eq!(instance.neg_infinity, f64::NEG_INFINITY);
}

// ===== UNICODE AND SPECIAL CHARACTER TESTS =====

#[test]
fn test_unicode_field_names_and_values() {
    #[derive(TypeStateBuilder)]
    struct UnicodeStruct {
        #[builder(required)]
        名前: String, // Japanese for "name"

        #[builder(default = "\"🚀\".to_string()")]
        emoji_field: String,

        #[builder(default = "\"Здравствуй мир\".to_string()")]
        cyrillic_field: String,
    }

    let instance = UnicodeStruct::builder().名前("テスト".to_string()).build();

    assert_eq!(instance.名前, "テスト");
    assert_eq!(instance.emoji_field, "🚀");
    assert_eq!(instance.cyrillic_field, "Здравствуй мир");
}

// ===== COMPLEX DEFAULT EXPRESSIONS =====

#[test]
fn test_complex_default_expressions() {
    #[derive(TypeStateBuilder)]
    struct ComplexDefaults {
        #[builder(required)]
        name: String,

        #[builder(default = "vec![1, 2, 3, 4, 5]")]
        numbers: Vec<i32>,

        #[builder(default = "std::collections::HashMap::new()")]
        map: std::collections::HashMap<String, i32>,

        #[builder(default = "Some(42)")]
        option_with_value: Option<i32>,

        #[builder(default = "None")]
        option_none: Option<String>,

        #[builder(default = "Ok(\"success\".to_string())")]
        result_ok: Result<String, String>,

        #[builder(default = "std::time::SystemTime::now()")]
        timestamp: std::time::SystemTime,
    }

    let instance = ComplexDefaults::builder().name("test".to_string()).build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.numbers, vec![1, 2, 3, 4, 5]);
    assert!(instance.map.is_empty());
    assert_eq!(instance.option_with_value, Some(42));
    assert_eq!(instance.option_none, None);
    assert!(instance.result_ok.is_ok());
    // Verify timestamp was set to current time (should be recent)
    assert!(instance.timestamp.elapsed().unwrap().as_secs() < 1);
}

// ===== STRESS TEST: MAXIMUM FIELD COUNT =====

#[test]
fn test_many_fields_struct() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create")]
    struct ManyFields {
        #[builder(required)]
        field01: String,
        #[builder(required)]
        field02: String,
        #[builder(required)]
        field03: String,
        #[builder(required)]
        field04: String,
        #[builder(required)]
        field05: String,

        field06: Option<i32>,
        field07: Option<i32>,
        field08: Option<i32>,
        field09: Option<i32>,
        field10: Option<i32>,

        #[builder(default = "1")]
        auto01: i32,
        #[builder(default = "2")]
        auto02: i32,
        #[builder(default = "3")]
        auto03: i32,
        #[builder(default = "4")]
        auto04: i32,
        #[builder(default = "5")]
        auto05: i32,

        #[builder(skip_setter, default = "\"skip1\".to_string()")]
        skip01: String,
        #[builder(skip_setter, default = "\"skip2\".to_string()")]
        skip02: String,
        #[builder(skip_setter, default = "\"skip3\".to_string()")]
        skip03: String,
        #[builder(skip_setter, default = "\"skip4\".to_string()")]
        skip04: String,
        #[builder(skip_setter, default = "\"skip5\".to_string()")]
        skip05: String,
    }

    let instance = ManyFields::builder()
        .field01("1".to_string())
        .field02("2".to_string())
        .field03("3".to_string())
        .field04("4".to_string())
        .field05("5".to_string())
        .field06(Some(6))
        .field07(Some(7))
        .create();

    assert_eq!(instance.field01, "1");
    assert_eq!(instance.field02, "2");
    assert_eq!(instance.field03, "3");
    assert_eq!(instance.field04, "4");
    assert_eq!(instance.field05, "5");
    assert_eq!(instance.field06, Some(6));
    assert_eq!(instance.field07, Some(7));
    assert_eq!(instance.field08, None); // Default Option value
    assert_eq!(instance.field09, None); // Default Option value
    assert_eq!(instance.field10, None); // Default Option value
    assert_eq!(instance.auto01, 1);
    assert_eq!(instance.auto02, 2);
    assert_eq!(instance.auto03, 3);
    assert_eq!(instance.auto04, 4);
    assert_eq!(instance.auto05, 5);
    assert_eq!(instance.skip01, "skip1");
    assert_eq!(instance.skip02, "skip2");
    assert_eq!(instance.skip03, "skip3");
    assert_eq!(instance.skip04, "skip4");
    assert_eq!(instance.skip05, "skip5");
}

// ===== NESTED STRUCT SCENARIOS =====

#[test]
fn test_nested_builder_structs() {
    #[derive(TypeStateBuilder)]
    struct InnerStruct {
        #[builder(required)]
        inner_field: String,

        inner_optional: Option<i32>,
    }

    #[derive(TypeStateBuilder)]
    struct OuterStruct {
        #[builder(required)]
        outer_field: String,

        #[builder(required)]
        inner: InnerStruct,

        outer_optional: Option<bool>,
    }

    let inner = InnerStruct::builder()
        .inner_field("inner".to_string())
        .inner_optional(Some(42))
        .build();

    let outer = OuterStruct::builder()
        .outer_field("outer".to_string())
        .inner(inner)
        .outer_optional(Some(true))
        .build();

    assert_eq!(outer.outer_field, "outer");
    assert_eq!(outer.inner.inner_field, "inner");
    assert_eq!(outer.inner.inner_optional, Some(42));
    assert_eq!(outer.outer_optional, Some(true));
}

// ===== RECURSIVE TYPE SCENARIOS (with Box) =====

#[test]
fn test_recursive_type_with_box() {
    #[derive(TypeStateBuilder)]
    struct Node {
        #[builder(required)]
        value: i32,

        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    let leaf1 = Node::builder().value(1).build();

    let leaf2 = Node::builder().value(3).build();

    let root = Node::builder()
        .value(2)
        .left(Some(Box::new(leaf1)))
        .right(Some(Box::new(leaf2)))
        .build();

    assert_eq!(root.value, 2);
    assert_eq!(root.left.as_ref().unwrap().value, 1);
    assert_eq!(root.right.as_ref().unwrap().value, 3);
}

// ===== TRAIT OBJECT SCENARIOS =====

#[test]
fn test_trait_objects() {
    trait TestTrait {
        fn test_method(&self) -> String;
    }

    struct TestImpl;
    impl TestTrait for TestImpl {
        fn test_method(&self) -> String {
            "test".to_string()
        }
    }

    #[derive(TypeStateBuilder)]
    struct WithTraitObjects {
        #[builder(required)]
        trait_obj: Box<dyn TestTrait>,

        #[builder(required)]
        sync_trait_obj: Box<dyn TestTrait + Send + Sync>,

        optional_trait: Option<Box<dyn TestTrait>>,
    }

    let instance = WithTraitObjects::builder()
        .trait_obj(Box::new(TestImpl))
        .sync_trait_obj(Box::new(TestImpl))
        .optional_trait(Some(Box::new(TestImpl)))
        .build();

    assert_eq!(instance.trait_obj.test_method(), "test");
    assert_eq!(instance.sync_trait_obj.test_method(), "test");
    assert_eq!(
        instance.optional_trait.as_ref().unwrap().test_method(),
        "test"
    );
}

// ===== ZERO-SIZED TYPE SCENARIOS =====

#[test]
fn test_zero_sized_types() {
    #[derive(Debug, PartialEq)]
    struct ZeroSized;

    #[derive(TypeStateBuilder)]
    struct WithZeroSized {
        #[builder(required)]
        data: String,

        #[builder(default = "ZeroSized")]
        zero_sized: ZeroSized,

        #[builder(default = "()")]
        unit: (),
    }

    let instance = WithZeroSized::builder().data("test".to_string()).build();

    assert_eq!(instance.data, "test");
    assert_eq!(instance.zero_sized, ZeroSized);
    // Verify unit field is the unit value
    assert!(matches!(instance.unit, ()));
}

// ===== RAW POINTER SCENARIOS =====

#[test]
fn test_raw_pointers() {
    #[derive(TypeStateBuilder)]
    struct WithRawPointers {
        #[builder(required)]
        data: String,

        #[builder(default = "std::ptr::null()")]
        raw_ptr: *const i32,

        #[builder(default = "std::ptr::null_mut()")]
        raw_mut_ptr: *mut i32,
    }

    let instance = WithRawPointers::builder().data("test".to_string()).build();

    assert_eq!(instance.data, "test");
    assert!(instance.raw_ptr.is_null());
    assert!(instance.raw_mut_ptr.is_null());
}

// ===== LIFETIME ELISION SCENARIOS =====

#[test]
fn test_lifetime_elision() {
    #[derive(TypeStateBuilder)]
    struct WithLifetimeElision<'a> {
        #[builder(required)]
        text: &'a str,

        #[builder(required)]
        bytes: &'a [u8],

        optional_slice: Option<&'a [i32]>,
    }

    let text = "hello";
    let bytes = b"world";
    let numbers = [1, 2, 3];

    let instance = WithLifetimeElision::builder()
        .text(text)
        .bytes(bytes)
        .optional_slice(Some(&numbers))
        .build();

    assert_eq!(instance.text, "hello");
    assert_eq!(instance.bytes, b"world");
    assert_eq!(instance.optional_slice, Some(&[1, 2, 3][..]));
}

// ===== GENERIC ASSOCIATED TYPE SCENARIOS =====

#[test]
fn test_generic_associated_types() {
    trait Container {
        type Item<T>;
        fn create<T>(item: T) -> Self::Item<T>;
    }

    struct VecContainer;
    impl Container for VecContainer {
        type Item<T> = Vec<T>;
        fn create<T>(item: T) -> Self::Item<T> {
            vec![item]
        }
    }

    #[derive(TypeStateBuilder)]
    struct WithGenericAssociated<C: Container> {
        #[builder(required)]
        name: String,

        #[builder(required)]
        int_items: <C as Container>::Item<i32>,

        string_items: Option<<C as Container>::Item<String>>,
    }

    let instance = WithGenericAssociated::<VecContainer>::builder()
        .name("test".to_string())
        .int_items(vec![1, 2, 3])
        .string_items(Some(vec!["a".to_string(), "b".to_string()]))
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.int_items, vec![1, 2, 3]);
    assert_eq!(
        instance.string_items,
        Some(vec!["a".to_string(), "b".to_string()])
    );

    // Test the Container trait's create method
    let created_item = VecContainer::create(42);
    assert_eq!(created_item, vec![42]);
}

// ===== CONST EVALUATION SCENARIOS =====

#[test]
fn test_const_evaluation_defaults() {
    const COMPUTED_VALUE: i32 = 10 + 20 + 30;
    const ARRAY_SIZE: usize = 5;

    #[derive(TypeStateBuilder)]
    struct WithConstEval {
        #[builder(required)]
        name: String,

        #[builder(default = "COMPUTED_VALUE")]
        computed: i32,

        #[builder(default = "[0; ARRAY_SIZE]")]
        array: [i32; ARRAY_SIZE],

        #[builder(default = "ARRAY_SIZE")]
        size: usize,
    }

    let instance = WithConstEval::builder().name("test".to_string()).build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.computed, 60);
    assert_eq!(instance.array, [0; 5]);
    assert_eq!(instance.size, 5);
}
