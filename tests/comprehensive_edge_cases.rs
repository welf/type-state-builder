#![allow(dead_code, clippy::type_complexity, clippy::unused_unit)]
//! Comprehensive Edge Case Test Suite for TypeStateBuilder
//!
//! This test suite covers ALL possible complex scenarios, edge cases, and combinations
//! to ensure the macro works correctly with maximum complexity.

use std::clone::Clone;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use type_state_builder::TypeStateBuilder;

// ===== COMPLEX LIFETIME SCENARIOS =====

#[test]
fn test_multiple_lifetimes_with_bounds() {
    #[derive(TypeStateBuilder)]
    struct ComplexLifetimes<'a, 'b, 'c>
    where
        'a: 'b,
        'b: 'c,
    {
        #[builder(required)]
        primary: &'a str,

        #[builder(required)]
        secondary: &'b str,

        tertiary: Option<&'c str>,
        fallback: Option<&'static str>,
    }

    let instance = ComplexLifetimes::builder()
        .primary("primary")
        .secondary("secondary")
        .tertiary(Some("tertiary"))
        .build();

    assert_eq!(instance.primary, "primary");
    assert_eq!(instance.secondary, "secondary");
    assert_eq!(instance.tertiary, Some("tertiary"));
}

#[test]
fn test_self_referential_lifetimes() {
    #[derive(TypeStateBuilder)]
    struct SelfReferential<'a> {
        #[builder(required)]
        data: &'a str,

        #[builder(required)]
        reference_to_data: &'a &'a str,

        optional_ref: Option<&'a str>,
    }

    let data = "hello";
    let ref_to_data = &data;

    let instance = SelfReferential::builder()
        .data(data)
        .reference_to_data(ref_to_data)
        .optional_ref(Some("world"))
        .build();

    assert_eq!(instance.data, "hello");
    assert_eq!(*instance.reference_to_data, "hello");
    assert_eq!(instance.optional_ref, Some("world"));
}

// ===== COMPLEX GENERIC SCENARIOS =====

#[test]
fn test_multiple_complex_generic_bounds() {
    trait CustomTrait {
        fn custom_method(&self) -> String;
    }

    impl CustomTrait for String {
        fn custom_method(&self) -> String {
            self.clone()
        }
    }

    #[derive(TypeStateBuilder)]
    struct ComplexGenerics<T, U, V>
    where
        T: Debug + Clone + CustomTrait,
        U: Hash + Eq + Debug,
        V: Into<String> + Clone,
    {
        #[builder(required)]
        primary_data: T,

        #[builder(required)]
        key_data: U,

        convertible_data: Option<V>,
        storage: Option<HashMap<U, T>>,
    }

    let mut map = HashMap::new();
    map.insert(42, "test".to_string());

    let instance = ComplexGenerics::<String, i32, &str>::builder()
        .primary_data("hello".to_string())
        .key_data(42)
        .convertible_data(Some("world"))
        .storage(Some(map))
        .build();

    assert_eq!(instance.primary_data.custom_method(), "hello");
    assert_eq!(instance.key_data, 42);
    assert_eq!(
        Into::<String>::into(instance.convertible_data.unwrap()),
        "world".to_string()
    );
}

#[test]
fn test_associated_types_and_complex_bounds() {
    trait AssociatedTypeTrait {
        type Output: Debug + Clone + PartialEq;
        fn process(&self) -> Self::Output;
    }

    impl AssociatedTypeTrait for String {
        type Output = usize;
        fn process(&self) -> Self::Output {
            self.len()
        }
    }

    #[derive(TypeStateBuilder)]
    struct WithAssociatedTypes<T: AssociatedTypeTrait> {
        #[builder(required)]
        processor: T,

        expected_output: Option<T::Output>,
        cache: Option<T::Output>,
        #[builder(skip_setter, default = "Some(42)")]
        sense_of_life: Option<u8>,
    }

    let instance = WithAssociatedTypes::builder()
        .processor("hello".to_string())
        .expected_output(Some(5))
        .build();

    assert_eq!(instance.processor.process(), 5);
    assert_eq!(instance.expected_output, Some(5));
    assert_eq!(instance.cache, None);
    assert_eq!(instance.sense_of_life, Some(42));
}

// ===== PHANTOMDATA COMPREHENSIVE TESTS =====

#[test]
fn test_phantom_data_single_lifetime() {
    #[derive(TypeStateBuilder)]
    struct WithPhantomLifetime<'a> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom: PhantomData<&'a str>,

        optional: Option<i32>,
    }

    let instance = WithPhantomLifetime::builder()
        .data("test".to_string())
        .optional(Some(42))
        .build();

    assert_eq!(instance.data, "test");
    assert_eq!(instance.optional, Some(42));
}

#[test]
fn test_phantom_data_multiple_lifetimes() {
    #[derive(TypeStateBuilder)]
    struct WithMultiplePhantomLifetimes<'a, 'b, 'c>
    where
        'a: 'b,
    {
        #[builder(required)]
        name: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom1: PhantomData<(&'a str, &'b str)>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom2: PhantomData<&'c str>,

        data: Option<String>,
    }

    let instance = WithMultiplePhantomLifetimes::builder()
        .name("test".to_string())
        .data(Some("data".to_string()))
        .build();

    assert_eq!(instance.name, "test");
    assert_eq!(instance.data, Some("data".to_string()));
}

#[test]
fn test_phantom_data_multiple_generics() {
    #[derive(TypeStateBuilder)]
    struct WithMultiplePhantomGenerics<T, U, V>
    where
        T: Debug,
        U: Clone,
        V: Hash,
    {
        #[builder(required)]
        actual_data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_tuple: PhantomData<(T, U, V)>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn: PhantomData<fn(T, U) -> V>,

        optional: Option<i32>,
    }

    let instance = WithMultiplePhantomGenerics::<String, i32, u64>::builder()
        .actual_data("test".to_string())
        .optional(Some(42))
        .build();

    assert_eq!(instance.actual_data, "test");
    assert_eq!(instance.optional, Some(42));
}

#[test]
fn test_phantom_data_mixed_lifetimes_and_generics() {
    #[derive(TypeStateBuilder)]
    struct PhantomMixed<'a, 'b, T, U>
    where
        T: Debug + 'a,
        U: Clone + 'b,
        'a: 'b,
    {
        #[builder(required)]
        real_data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_complex: PhantomData<(&'a T, &'b U)>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_with_lifetimes: PhantomData<for<'c> fn(&'c T, &'c U) -> &'c str>,

        optional_ref: Option<&'static str>,
    }

    let instance = PhantomMixed::<String, i32>::builder()
        .real_data("test".to_string())
        .optional_ref(Some("static"))
        .build();

    assert_eq!(instance.real_data, "test");
    assert_eq!(instance.optional_ref, Some("static"));
}

// ===== FUNCTION POINTERS AND CLOSURES =====

#[test]
fn test_function_pointers_with_lifetimes() {
    #[derive(TypeStateBuilder)]
    struct WithFunctionPointers<'a, T: ?Sized>
    where
        T: 'a,
    {
        #[builder(required)]
        processor: for<'b> fn(&'b T) -> String,

        #[builder(required)]
        data: &'a T,

        transformer: Option<fn(&T) -> &T>,
        #[builder(default = "None")]
        complex_fn: Option<for<'b> fn(&'b T, &'b str) -> String>,
    }

    fn process_string(s: &str) -> String {
        s.to_uppercase()
    }

    fn transform_str(_s: &str) -> &str {
        "transformed"
    }

    let data = "hello";
    let instance = WithFunctionPointers::builder()
        .processor(process_string)
        .data(data)
        .transformer(Some(transform_str as fn(&str) -> &str))
        .build();

    assert_eq!((instance.processor)(instance.data), "HELLO");
}

#[test]
fn test_closure_types_with_complex_bounds() {
    #[derive(TypeStateBuilder)]
    struct WithClosures<F, G, T>
    where
        F: Fn(&T) -> String,
        G: FnMut(T) -> T,
        T: Clone + Debug,
    {
        #[builder(required)]
        formatter: F,

        #[builder(required)]
        mutator: G,

        data: Option<T>,
        #[builder(default = "None")]
        optional_once: Option<Box<dyn FnOnce(T) -> String>>,
    }

    let instance = WithClosures::builder()
        .formatter(|x: &i32| format!("value: {x}"))
        .mutator(|x: i32| x * 2)
        .data(Some(42))
        .build();

    assert_eq!((instance.formatter)(&42), "value: 42");
}

// ===== MAXIMUM COMPLEXITY SCENARIOS =====

#[test]
fn test_maximum_complexity_all_features_combined() {
    trait ComplexTrait<T> {
        type Associated: Debug + Clone;
        fn complex_method(&self, input: T) -> Self::Associated;
    }

    impl ComplexTrait<String> for i32 {
        type Associated = String;
        fn complex_method(&self, input: String) -> Self::Associated {
            format!("{self}_{input}")
        }
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "construct_complex")]
    struct MaximumComplexity<
        'a,
        'b,
        T: ComplexTrait<String, Associated = String> + Debug + Clone + 'a,
        U: Hash + Eq + Debug + Send + Sync,
        V: Into<String> + Clone + Debug,
        const N: usize,
    >
    where
        'a: 'b,
        [T; N]: Debug,
    {
        #[builder(required, setter_name = "set_primary")]
        primary_complex: &'a T,

        #[builder(required)]
        secondary_data: U,

        #[builder(required)]
        array_data: [T; N],

        #[builder(skip_setter, default = "PhantomData")]
        phantom_complex: PhantomData<(&'a T, &'b U, fn(V) -> String)>,

        #[builder(default = "HashMap::new()")]
        storage: HashMap<U, T::Associated>,

        #[builder(setter_name = "set_convertible")]
        convertible: Option<V>,

        nested_complex: Option<BTreeMap<String, Vec<&'b T>>>,

        #[builder(default = "None")]
        function_ptr: Option<fn(&'a T, &'b str) -> T::Associated>,

        #[builder(skip_setter, default = "vec![]")]
        auto_generated: Vec<String>,
    }

    let data: i32 = 42;
    let key = "test_key".to_string();
    let array = [10, 20, 30];

    let instance = MaximumComplexity::<i32, String, &str, 3>::builder()
        .set_primary(&data)
        .secondary_data(key)
        .array_data(array)
        .set_convertible(Some("convert_me"))
        .construct_complex();

    assert_eq!(*instance.primary_complex, 42);
    assert_eq!(instance.secondary_data, "test_key");
    assert_eq!(instance.array_data, [10, 20, 30]);
    assert!(instance.auto_generated.is_empty());
}

// ===== NESTED COMPLEX TYPES =====

#[test]
fn test_deeply_nested_generic_types() {
    #[derive(TypeStateBuilder)]
    struct DeeplyNested<K, V, T>
    where
        K: Hash + Eq + Clone + Debug,
        V: Clone + Debug,
        T: Clone + Debug,
    {
        #[builder(required)]
        nested_map: HashMap<K, Vec<Option<Result<T, String>>>>,

        #[builder(required)]
        complex_optional: Option<Result<HashMap<K, V>, Box<dyn std::error::Error>>>,

        #[builder(default = "BTreeMap::new()")]
        tree_map: BTreeMap<String, Vec<HashMap<K, Option<T>>>>,

        callback: Option<Box<dyn Fn(Result<V, T>) -> Option<K>>>,
    }

    let mut nested = HashMap::new();
    nested.insert(
        "key".to_string(),
        vec![Some(Ok(42)), None, Some(Err("error".to_string()))],
    );

    let mut result_map = HashMap::new();
    result_map.insert("key".to_string(), 100);

    let instance = DeeplyNested::<String, i32, i32>::builder()
        .nested_map(nested)
        .complex_optional(Some(Ok(result_map)))
        .build();

    assert!(instance.nested_map.contains_key("key"));
    assert!(instance.complex_optional.is_some());
}

// ===== REAL-WORLD COMPLEX SCENARIOS =====

#[test]
fn test_database_connection_like_scenario() {
    trait DatabaseDriver {
        type Connection;
        type Error: Debug;
    }

    struct MockDriver;
    impl DatabaseDriver for MockDriver {
        type Connection = String;
        type Error = String;
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "establish_connection")]
    struct DatabaseConfig<'a, D, F, G>
    where
        D: DatabaseDriver,
        F: Fn(&str) -> Result<D::Connection, D::Error>,
        G: Fn(D::Error) -> String,
    {
        #[builder(required)]
        connection_string: &'a str,

        #[builder(required)]
        connector: F,

        #[builder(required)]
        error_handler: G,

        #[builder(skip_setter, default = "PhantomData")]
        phantom: PhantomData<D>,

        #[builder(default = "30")]
        timeout_seconds: u32,

        #[builder(default = "None")]
        pool_size: Option<usize>,

        retry_attempts: Option<u32>,
    }

    let conn_str = "postgresql://localhost:5432/db";

    let instance = DatabaseConfig::<MockDriver, _, _>::builder()
        .connection_string(conn_str)
        .connector(|s: &str| Ok(s.to_string()))
        .error_handler(|e| format!("Database error: {e:?}"))
        .retry_attempts(Some(3))
        .establish_connection();

    assert_eq!(instance.connection_string, "postgresql://localhost:5432/db");
    assert_eq!(instance.timeout_seconds, 30);
    assert_eq!(instance.retry_attempts, Some(3));
}

#[test]
fn test_async_context_like_scenario() {
    use std::future::Future;
    use std::pin::Pin;

    trait AsyncProcessor<T> {
        type Output;
        type Future: Future<Output = Self::Output>;

        fn process(&self, input: T) -> Self::Future;
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create_context")]
    struct AsyncContext<'a, T, P, F>
    where
        T: Clone + Send + 'a,
        P: AsyncProcessor<T>,
        F: Fn(P::Output) -> String + Send + Sync,
        P::Output: Send,
    {
        #[builder(required)]
        processor: P,

        #[builder(required)]
        formatter: F,

        #[builder(skip_setter, default = "PhantomData")]
        lifetime_phantom: PhantomData<&'a T>,

        #[builder(default = "None")]
        timeout: Option<std::time::Duration>,

        #[builder(default = "1")]
        max_concurrent: usize,

        error_callback: Option<Box<dyn Fn(String) -> () + Send + Sync>>,
    }

    struct MockProcessor;
    impl AsyncProcessor<i32> for MockProcessor {
        type Output = String;
        type Future = Pin<Box<dyn Future<Output = String>>>;

        fn process(&self, input: i32) -> Self::Future {
            Box::pin(async move { input.to_string() })
        }
    }

    let instance = AsyncContext::<i32, _, _>::builder()
        .processor(MockProcessor)
        .formatter(|output| format!("Result: {output}"))
        .max_concurrent(4)
        .create_context();

    assert_eq!(instance.max_concurrent, 4);
}

// ===== CONST GENERIC EDGE CASES =====

#[test]
fn test_multiple_const_generics_with_complex_types() {
    #[derive(TypeStateBuilder)]
    struct MultiConstGeneric<T, const N: usize, const M: usize, const FLAG: bool>
    where
        T: Copy + Debug,
    {
        #[builder(required)]
        matrix: [[T; N]; M],

        #[builder(required)]
        config_flag: bool,

        #[builder(skip_setter, default = "FLAG")]
        const_flag: bool,

        #[builder(default = "[None; N]")]
        buffer: [Option<T>; N],

        metadata: Option<String>,
    }

    let matrix = [[1, 2], [3, 4], [5, 6]];

    let instance = MultiConstGeneric::<i32, 2, 3, true>::builder()
        .matrix(matrix)
        .config_flag(false)
        .metadata(Some("test".to_string()))
        .build();

    assert_eq!(instance.matrix[0][0], 1);
    assert!(!instance.config_flag);
    assert!(instance.const_flag);
    assert_eq!(instance.metadata, Some("test".to_string()));
}

// ===== ERROR HANDLING AND EDGE CASES =====

#[test]
fn test_all_attributes_combined() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "finalize")]
    struct AllAttributesCombined<T>
    where
        T: Clone + Debug,
    {
        #[builder(required, setter_name = "set_primary")]
        primary: T,

        #[builder(setter_name = "set_secondary", default = "\"default\".to_string()")]
        secondary: String,

        #[builder(skip_setter, default = "42")]
        auto_field: i32,

        #[builder(default = "None")]
        optional_with_default: Option<String>,

        regular_optional: Option<bool>,
    }

    let instance = AllAttributesCombined::<String>::builder()
        .set_primary("test".to_string())
        .set_secondary("custom".to_string())
        .regular_optional(Some(true))
        .finalize();

    assert_eq!(instance.primary, "test");
    assert_eq!(instance.secondary, "custom");
    assert_eq!(instance.auto_field, 42);
    assert_eq!(instance.optional_with_default, None);
    assert_eq!(instance.regular_optional, Some(true));
}

// ===== STRESS TEST WITH MAXIMUM GENERIC PARAMETERS =====

#[test]
fn test_maximum_generic_parameters() {
    #[derive(TypeStateBuilder)]
    #[builder(build_method = "construct")]
    struct MaxGenerics<'a, 'b, 'c, A, B, C, D, E, F, G, H, const N1: usize, const N2: usize>
    where
        'a: 'b,
        'b: 'c,
        A: Clone + Debug + 'a,
        B: Hash + Eq,
        C: Into<String>,
        D: From<i32>,
        E: AsRef<str>,
        F: AsMut<[u8]>,
        G: Iterator<Item = i32>,
        H: Send + Sync + 'static,
    {
        #[builder(required)]
        field_a: &'a A,

        #[builder(required)]
        field_b: B,

        #[builder(skip_setter, default = "PhantomData")]
        phantom: PhantomData<(&'b C, &'c D, E, F, G, H)>,

        #[builder(default = "[0; N1]")]
        array1: [i32; N1],

        #[builder(default = "[false; N2]")]
        array2: [bool; N2],

        optional_field: Option<String>,
    }

    let data: String = "test".to_string();

    let instance = MaxGenerics::<
        String,
        i32,
        String,
        i32,
        String,
        Vec<u8>,
        std::iter::Empty<i32>,
        String,
        3,
        2,
    >::builder()
    .field_a(&data)
    .field_b(42)
    .optional_field(Some("optional".to_string()))
    .construct();

    assert_eq!(*instance.field_a, "test");
    assert_eq!(instance.field_b, 42);
    assert_eq!(instance.array1, [0, 0, 0]);
    assert_eq!(instance.array2, [false, false]);
    assert_eq!(instance.optional_field, Some("optional".to_string()));
}
