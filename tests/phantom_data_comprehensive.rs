//! Comprehensive PhantomData Testing
//!
//! This test file covers every possible PhantomData scenario to ensure
//! our macro handles phantom types correctly in all situations.

use std::collections::HashMap;
use std::marker::PhantomData;
use type_state_builder::TypeStateBuilder;

// ===== PHANTOM DATA WITH SINGLE TYPES =====

#[test]
fn test_phantom_data_single_type() {
    #[derive(TypeStateBuilder)]
    struct SinglePhantom<T> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom: PhantomData<T>,
    }

    let instance = SinglePhantom::<i32>::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

#[test]
fn test_phantom_data_single_lifetime() {
    #[derive(TypeStateBuilder)]
    struct SinglePhantomLifetime<'a> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom: PhantomData<&'a str>,
    }

    let instance = SinglePhantomLifetime::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

// ===== PHANTOM DATA WITH TUPLES =====

#[test]
fn test_phantom_data_tuple_types() {
    #[derive(TypeStateBuilder)]
    struct TuplePhantom<T, U, V> {
        #[builder(required)]
        real_data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_tuple: PhantomData<(T, U, V)>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_nested: PhantomData<((T, U), V)>,

        optional: Option<i32>,
    }

    let instance = TuplePhantom::<String, i32, f64>::builder()
        .real_data("test".to_string())
        .optional(Some(42))
        .build();

    assert_eq!(instance.real_data, "test");
    assert_eq!(instance.optional, Some(42));
}

#[test]
fn test_phantom_data_tuple_lifetimes() {
    #[derive(TypeStateBuilder)]
    struct TuplePhantomLifetimes<'a, 'b, 'c> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_lifetime_tuple: PhantomData<(&'a str, &'b str, &'c str)>,

        optional: Option<&'static str>,
    }

    let instance = TuplePhantomLifetimes::builder()
        .data("test".to_string())
        .optional(Some("static"))
        .build();

    assert_eq!(instance.data, "test");
    assert_eq!(instance.optional, Some("static"));
}

// ===== PHANTOM DATA WITH FUNCTION TYPES =====

#[test]
fn test_phantom_data_function_types() {
    #[derive(TypeStateBuilder)]
    struct PhantomFunctions<T, U, V> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn: PhantomData<fn(T) -> U>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_multi: PhantomData<fn(T, U) -> V>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_ptr: PhantomData<*const fn(T) -> U>,
    }

    let instance = PhantomFunctions::<i32, String, f64>::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

#[test]
fn test_phantom_data_closure_types() {
    #[derive(TypeStateBuilder)]
    struct PhantomClosures<T, U> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn: PhantomData<Box<dyn Fn(T) -> U>>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_mut: PhantomData<Box<dyn FnMut(T) -> U>>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_once: PhantomData<Box<dyn FnOnce(T) -> U>>,
    }

    let instance = PhantomClosures::<i32, String>::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

// ===== PHANTOM DATA WITH COMPLEX NESTED TYPES =====

#[test]
fn test_phantom_data_complex_nested() {
    #[derive(TypeStateBuilder)]
    struct ComplexNested<T, U, V> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_vec: PhantomData<Vec<T>>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_map: PhantomData<HashMap<T, U>>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_nested: PhantomData<Vec<HashMap<T, Option<U>>>>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_result: PhantomData<Result<T, V>>,
    }

    let instance = ComplexNested::<String, i32, f64>::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

// ===== PHANTOM DATA WITH LIFETIME BOUNDS =====

#[test]
fn test_phantom_data_lifetime_bounds() {
    #[derive(TypeStateBuilder)]
    struct PhantomLifetimeBounds<'a, 'b, T>
    where
        'a: 'b,
        T: 'a,
    {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_bounded: PhantomData<&'a T>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_lifetime: PhantomData<fn(&'a T) -> &'b str>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_complex: PhantomData<for<'c> fn(&'c T) -> &'c str>,
    }

    let instance = PhantomLifetimeBounds::<String>::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

// ===== PHANTOM DATA WITH TRAIT BOUNDS =====

#[test]
fn test_phantom_data_trait_bounds() {
    use std::clone::Clone;
    use std::fmt::Debug;

    #[derive(TypeStateBuilder)]
    struct PhantomTraitBounds<T, U>
    where
        T: Debug + Clone,
        U: Send + Sync,
    {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_debug: PhantomData<T>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_send: PhantomData<U>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_bound: PhantomData<fn(T) -> U>,
    }

    let instance = PhantomTraitBounds::<String, i32>::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

// ===== PHANTOM DATA WITH CONST GENERICS =====

#[test]
fn test_phantom_data_const_generics() {
    #[derive(TypeStateBuilder)]
    struct PhantomConstGenerics<T, const N: usize> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_array: PhantomData<[T; N]>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_fn_array: PhantomData<fn([T; N]) -> String>,

        actual_array: Option<[i32; N]>,
    }

    let instance = PhantomConstGenerics::<String, 3>::builder()
        .data("test".to_string())
        .actual_array(Some([1, 2, 3]))
        .build();

    assert_eq!(instance.data, "test");
    assert_eq!(instance.actual_array, Some([1, 2, 3]));
}

// ===== MULTIPLE PHANTOM DATA FIELDS =====

#[test]
fn test_multiple_phantom_data_fields() {
    #[derive(TypeStateBuilder)]
    struct MultiplePhantomFields<'a, 'b, T, U, V, const N: usize> {
        #[builder(required)]
        real_data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom1: PhantomData<T>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom2: PhantomData<U>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom3: PhantomData<V>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_lifetime1: PhantomData<&'a str>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_lifetime2: PhantomData<&'b str>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_const: PhantomData<[T; N]>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_complex: PhantomData<fn(&'a T, &'b U) -> [V; N]>,

        optional: Option<i32>,
    }

    let instance = MultiplePhantomFields::<String, i32, f64, 2>::builder()
        .real_data("test".to_string())
        .optional(Some(42))
        .build();

    assert_eq!(instance.real_data, "test");
    assert_eq!(instance.optional, Some(42));
}

// ===== PHANTOM DATA IN REQUIRED FIELDS (SHOULD USE SKIP_SETTER) =====

#[test]
fn test_phantom_data_with_required_must_skip_setter() {
    #[derive(TypeStateBuilder)]
    struct PhantomRequired<T> {
        #[builder(required)]
        real_field: String,

        // PhantomData should always use skip_setter, even if logically "required"
        #[builder(skip_setter, default = "PhantomData")]
        phantom_field: PhantomData<T>,
    }

    let instance = PhantomRequired::<i32>::builder()
        .real_field("test".to_string())
        .build();

    assert_eq!(instance.real_field, "test");
}

// ===== PHANTOM DATA WITH ALL CUSTOM ATTRIBUTES =====

#[test]
fn test_phantom_data_with_custom_build_method() {
    // Type alias to reduce complexity
    type ComplexPhantomType<'a, T, U> = PhantomData<(&'a T, fn(T) -> U)>;

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "construct")]
    struct PhantomWithCustomBuild<'a, T, U> {
        #[builder(required, setter_name = "set_data")]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_complex: ComplexPhantomType<'a, T, U>,

        #[builder(default = "\"auto\".to_string()")]
        auto_field: String,
    }

    let instance = PhantomWithCustomBuild::<i32, String>::builder()
        .set_data("test".to_string())
        .construct();

    assert_eq!(instance.data, "test");
    assert_eq!(instance.auto_field, "auto");
}

// ===== PHANTOM DATA WITH HIGHER RANKED TRAIT BOUNDS =====

#[test]
fn test_phantom_data_higher_ranked_trait_bounds() {
    #[derive(TypeStateBuilder)]
    struct PhantomHRTB<T> {
        #[builder(required)]
        data: String,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_hrtb: PhantomData<for<'a> fn(&'a T) -> &'a str>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom_complex_hrtb: PhantomData<for<'a, 'b> fn(&'a T, &'b str) -> &'a str>,
    }

    let instance = PhantomHRTB::<String>::builder()
        .data("test".to_string())
        .build();

    assert_eq!(instance.data, "test");
}

// ===== STRESS TEST: PHANTOM DATA ONLY STRUCT =====

#[test]
fn test_struct_with_only_phantom_data_and_defaults() {
    #[derive(TypeStateBuilder)]
    struct OnlyPhantomAndDefaults<T, U, V> {
        #[builder(skip_setter, default = "PhantomData")]
        phantom1: PhantomData<T>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom2: PhantomData<U>,

        #[builder(skip_setter, default = "PhantomData")]
        phantom3: PhantomData<V>,

        #[builder(default = "\"default\".to_string()")]
        auto_string: String,

        #[builder(default = "42")]
        auto_number: i32,
    }

    // This should work as a regular builder since no required fields
    let instance = OnlyPhantomAndDefaults::<String, i32, f64>::builder().build();

    assert_eq!(instance.auto_string, "default");
    assert_eq!(instance.auto_number, 42);
}
