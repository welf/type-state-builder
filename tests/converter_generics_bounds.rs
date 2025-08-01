//! Comprehensive tests for converter with generics and bounds
//!
//! This module tests converter functionality with complex generic scenarios:
//! - Generic types with bounds
//! - Associated types  
//! - Where clauses
//! - Complex trait bounds
//! - Multiple generic parameters with cross-constraints

use std::collections::HashMap;
use std::hash::Hash;
use type_state_builder::TypeStateBuilder;

// Basic generic with simple bounds
#[derive(TypeStateBuilder, Debug)]
struct GenericWithBounds<T: Clone + std::fmt::Debug> {
    #[builder(required)]
    data: T,

    #[builder(converter = |values: Vec<T>| values.into_iter().take(3).collect())]
    limited: Vec<T>,
}

#[test]
fn test_generic_with_bounds() {
    let instance = GenericWithBounds::<String>::builder()
        .data("test".to_string())
        .limited(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ])
        .build();

    assert_eq!(instance.data, "test".to_string());
    assert_eq!(instance.limited.len(), 3);
}

// Multiple generic parameters with cross-constraints
#[derive(TypeStateBuilder, Debug)]
struct MultipleGenerics<T, U, V>
where
    T: Clone + PartialEq,
    U: std::fmt::Display + Clone,
    V: From<T> + std::fmt::Debug + Default,
{
    #[builder(required)]
    first: T,

    #[builder(required)]
    second: U,

    #[builder(converter = |value: T| V::from(value))]
    converted: V,

    #[builder(converter = |pairs: Vec<(T, U)>| {
        pairs.into_iter()
            .map(|(t, u)| {
                // Use Clone constraint to clone t for comparison, then discard
                let _cloned = t.clone();
                format!("{u}")
            })
            .collect()
    })]
    formatted: Vec<String>,
}

#[test]
fn test_multiple_generics() {
    let instance = MultipleGenerics::<i32, String, i64>::builder()
        .first(42)
        .second("test".to_string())
        .converted(100i32)
        .formatted(vec![(1, "one".to_string()), (2, "two".to_string())])
        .build();

    assert_eq!(instance.first, 42);
    assert_eq!(instance.second, "test".to_string());
    assert_eq!(instance.converted, 100i64);
    assert_eq!(
        instance.formatted,
        vec!["one".to_string(), "two".to_string()]
    );
}

// Generic with associated types
trait Processor {
    type Item;
    type Output;

    fn process(item: Self::Item) -> Self::Output;
}

struct StringProcessor;
impl Processor for StringProcessor {
    type Item = String;
    type Output = usize;

    fn process(item: Self::Item) -> Self::Output {
        item.len()
    }
}

#[derive(TypeStateBuilder, Debug)]
struct AssociatedTypes<P: Processor> {
    #[builder(required)]
    processor_data: P::Item,

    #[builder(converter = |items: Vec<P::Item>| {
        items.into_iter().map(P::process).collect()
    })]
    processed: Vec<P::Output>,
}

#[test]
fn test_associated_types() {
    let instance = AssociatedTypes::<StringProcessor>::builder()
        .processor_data("test".to_string())
        .processed(vec![
            "hello".to_string(),
            "world".to_string(),
            "rust".to_string(),
        ])
        .build();

    assert_eq!(instance.processor_data, "test".to_string());
    assert_eq!(instance.processed, vec![5, 5, 4]); // lengths of the strings
}

// Complex where clauses with lifetime bounds
#[derive(TypeStateBuilder, Debug)]
struct ComplexWhereClauses<'a, T, U, K, V>
where
    T: Clone + 'a,
    U: std::fmt::Display + Clone,
    K: Hash + Eq + Clone,
    V: Clone + std::fmt::Debug,
    HashMap<K, V>: Clone,
{
    #[builder(required)]
    reference: &'a T,

    #[builder(required)]
    display_item: U,

    #[builder(converter = |pairs: Vec<(K, V)>| pairs.into_iter().collect())]
    map_data: HashMap<K, V>,

    #[builder(converter = |values: Vec<V>| values.into_iter().take(2).collect())]
    limited_values: Vec<V>,
}

#[test]
fn test_complex_where_clauses() {
    let data = String::from("reference_data");
    let instance = ComplexWhereClauses::builder()
        .reference(&data)
        .display_item(42.5f64)
        .map_data(vec![
            ("key1".to_string(), 100i32),
            ("key2".to_string(), 200i32),
        ])
        .limited_values(vec![1i32, 2i32, 3i32, 4i32])
        .build();

    assert_eq!(instance.reference, &data);
    assert_eq!(instance.display_item, 42.5f64);
    assert_eq!(instance.map_data.len(), 2);
    assert_eq!(instance.limited_values, vec![1, 2]);
}

// Converter with complex closure that uses generic bounds
#[derive(TypeStateBuilder, Debug)]
struct ConverterWithComplexBounds<T, U>
where
    T: IntoIterator<Item = U> + Clone,
    U: PartialOrd + Clone + std::fmt::Debug,
{
    #[builder(required)]
    source: T,

    #[builder(converter = |input: T| {
        let mut items: Vec<U> = input.into_iter().collect();
        items.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        items
    })]
    sorted: Vec<U>,

    #[builder(converter = |collections: Vec<T>| {
        collections.into_iter()
            .flat_map(|c| c.into_iter())
            .collect::<Vec<_>>()
            .len()
    })]
    total_count: usize,
}

#[test]
fn test_converter_with_complex_bounds() {
    let instance = ConverterWithComplexBounds::<Vec<i32>, i32>::builder()
        .source(vec![1, 2, 3])
        .sorted(vec![3, 1, 4, 1, 5])
        .total_count(vec![vec![1, 2], vec![3, 4, 5], vec![6]])
        .build();

    assert_eq!(instance.source, vec![1, 2, 3]);
    assert_eq!(instance.sorted, vec![1, 1, 3, 4, 5]);
    assert_eq!(instance.total_count, 6); // Total items across all collections
}

// Test with const generics and complex converter
#[derive(TypeStateBuilder, Debug, PartialEq)]
struct ConstGenericWithConverter<T, const N: usize>
where
    T: Clone + std::fmt::Debug + Default + Copy,
{
    #[builder(required)]
    array: [T; N],

    #[builder(converter = |values: Vec<T>| {
        let mut result = [T::default(); N];
        for (i, value) in values.into_iter().take(N).enumerate() {
            result[i] = value;
        }
        result
    }, default = "[T::default(); N]")]
    filled_array: [T; N],

    #[builder(converter = |input: Vec<[T; N]>| {
        input.into_iter()
            .flat_map(|arr| arr.into_iter())
            .take(N * 2)
            .collect()
    })]
    flattened: Vec<T>,
}

#[test]
fn test_const_generic_with_converter() {
    let instance = ConstGenericWithConverter::<i32, 3>::builder()
        .array([1, 2, 3])
        .filled_array(vec![10, 20, 30, 40, 50]) // Only first 3 will be used
        .flattened(vec![[100, 200, 300], [400, 500, 600]])
        .build();

    assert_eq!(instance.array, [1, 2, 3]);
    assert_eq!(instance.filled_array, [10, 20, 30]);
    assert_eq!(instance.flattened, vec![100, 200, 300, 400, 500, 600]);
}

// Test with lifetimes and generic bounds together
#[derive(TypeStateBuilder, Debug)]
struct LifetimeWithGenerics<'a, 'b, T, U>
where
    T: Clone + 'a,
    U: std::fmt::Display + 'b,
{
    #[builder(required)]
    ref_a: &'a T,

    #[builder(required)]
    ref_b: &'b U,

    #[builder(converter = |items: Vec<&'a T>| items.len())]
    ref_count: usize,

    #[builder(converter = |displays: Vec<&'b U>| {
        displays.into_iter()
            .map(|d| format!("{d}"))
            .collect::<Vec<_>>()
            .join(", ")
    })]
    combined_display: String,
}

#[test]
fn test_lifetime_with_generics() {
    let data_a = vec![1, 2, 3];
    let data_b = "test_string";
    let other_b = "another";

    let instance = LifetimeWithGenerics::builder()
        .ref_a(&data_a)
        .ref_b(&data_b)
        .ref_count(vec![&data_a, &data_a])
        .combined_display(vec![&data_b, &other_b])
        .build();

    assert_eq!(instance.ref_a, &vec![1, 2, 3]);
    assert_eq!(instance.ref_b, &"test_string");
    assert_eq!(instance.ref_count, 2);
    assert_eq!(instance.combined_display, "test_string, another");
}
