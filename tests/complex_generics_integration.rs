// Comprehensive integration tests for complex generic scenarios

use type_state_builder::TypeStateBuilder;

// Test multiple generic parameters with bounds
#[derive(TypeStateBuilder, Debug, Clone, PartialEq)]
struct MultiGeneric<T, U, V>
where
    T: Clone + Send,
    U: std::fmt::Debug,
    V: Default,
{
    #[builder(required)]
    first: T,

    #[builder(required)]
    second: U,

    third: V,

    #[builder(default = "None")]
    optional: Option<String>,
}

// Test generic with lifetimes and complex nested types
#[derive(TypeStateBuilder, Debug)]
struct ComplexLifetime<'a, 'b, T>
where
    T: 'a + Clone,
{
    #[builder(required)]
    data: &'a T,

    #[builder(required)]
    metadata: &'b str,

    #[builder(default = "Vec::new()")]
    tags: Vec<&'a str>,

    nested: Option<Box<T>>,
}

// Test deeply nested generic types
#[derive(TypeStateBuilder, Debug)]
struct NestedGenerics<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    #[builder(required)]
    primary_map: std::collections::HashMap<K, V>,

    #[builder(default = "std::collections::BTreeMap::new()")]
    secondary_map: std::collections::BTreeMap<String, Vec<V>>,

    cache: Option<std::collections::HashMap<K, std::collections::HashSet<V>>>,
}

// Test phantom data scenarios
#[derive(TypeStateBuilder, Debug)]
struct PhantomGeneric<T, U> {
    #[builder(required)]
    value: String,

    #[builder(default = "42")]
    number: i32,

    _phantom: std::marker::PhantomData<(T, U)>,
}

// Test const generics
#[derive(TypeStateBuilder, Debug)]
struct ConstGeneric<T, const N: usize>
where
    T: Copy + Default,
{
    #[builder(required)]
    data: [T; N],

    #[builder(default = "Vec::new()")]
    dynamic_data: Vec<T>,
}

// Test generic structs with custom trait bounds
trait CustomTrait {
    fn custom_method(&self) -> String;
}

#[derive(TypeStateBuilder, Debug)]
struct TraitBounded<T: CustomTrait + Clone> {
    #[builder(required)]
    item: T,

    #[builder(default = "Vec::new()")]
    collection: Vec<T>,
}

// Test associated types
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(TypeStateBuilder, Debug)]
struct WithAssociatedTypes<I>
where
    I: Iterator,
    I::Item: Clone,
{
    #[builder(required)]
    iterator: I,

    current: Option<I::Item>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_multi_generic_with_bounds() {
        let instance = MultiGeneric::<String, i32, bool>::builder()
            .first("hello".to_string())
            .second(42)
            .third(true)
            .build();

        assert_eq!(instance.first, "hello");
        assert_eq!(instance.second, 42);
        assert!(instance.third);
        assert_eq!(instance.optional, None);
    }

    #[test]
    fn test_complex_lifetime_generics() {
        let data = 42i32;
        let metadata = "test metadata";
        let tags = vec!["tag1", "tag2"];

        let instance = ComplexLifetime::builder()
            .data(&data)
            .metadata(metadata)
            .tags(tags)
            .nested(Some(Box::new(100i32)))
            .build();

        assert_eq!(*instance.data, 42);
        assert_eq!(instance.metadata, "test metadata");
        assert_eq!(instance.tags, vec!["tag1", "tag2"]);
        assert_eq!(*instance.nested.unwrap(), 100);
    }

    #[test]
    fn test_nested_generic_collections() {
        let mut primary = HashMap::new();
        primary.insert("key1".to_string(), 42);
        primary.insert("key2".to_string(), 84);

        let mut cache = HashMap::new();
        let mut set = std::collections::HashSet::new();
        set.insert(100);
        cache.insert("cache_key".to_string(), set);

        let instance = NestedGenerics::builder()
            .primary_map(primary)
            .cache(Some(cache))
            .build();

        assert_eq!(instance.primary_map.get("key1"), Some(&42));
        assert!(instance.secondary_map.is_empty());
        assert!(instance.cache.is_some());
    }

    #[test]
    fn test_phantom_data_generics() {
        let instance = PhantomGeneric::<String, i32>::builder()
            .value("test".to_string())
            .build();

        assert_eq!(instance.value, "test");
        assert_eq!(instance.number, 42);
        // PhantomData doesn't store actual data
    }

    #[test]
    fn test_const_generics() {
        let data = [1, 2, 3, 4, 5];

        let instance = ConstGeneric::<i32, 5>::builder()
            .data(data)
            .dynamic_data(vec![10, 20, 30])
            .build();

        assert_eq!(instance.data, [1, 2, 3, 4, 5]);
        assert_eq!(instance.dynamic_data, vec![10, 20, 30]);
    }

    #[test]
    fn test_builder_with_generics_defaults() {
        // Test that default values work correctly with generic types
        let instance = MultiGeneric::<String, i32, Vec<String>>::builder()
            .first("test".to_string())
            .second(123)
            .build();

        assert_eq!(instance.first, "test");
        assert_eq!(instance.second, 123);
        assert_eq!(instance.third, Vec::<String>::default()); // Default for Vec<String>
        assert_eq!(instance.optional, None);
    }

    #[test]
    fn test_complex_generic_chaining() {
        // Test that method chaining works correctly with complex generics
        let mut map = HashMap::new();
        map.insert(1, "one".to_string());

        let instance = NestedGenerics::builder().primary_map(map).build();

        assert_eq!(instance.primary_map.get(&1), Some(&"one".to_string()));
        assert!(instance.secondary_map.is_empty());
        assert!(instance.cache.is_none());
    }

    // Test that we can use the builder with different generic parameter combinations
    #[test]
    fn test_different_generic_combinations() {
        // Test with different type combinations
        let _instance1 = MultiGeneric::<i32, String, bool>::builder()
            .first(42)
            .second("hello".to_string())
            .build();

        let _instance2 = MultiGeneric::<Vec<u8>, f64, Option<i32>>::builder()
            .first(vec![1, 2, 3])
            .second(std::f64::consts::PI)
            .build();
    }

    // Test structures with custom traits
    #[derive(Clone, Debug)]
    struct TestItem(String);

    impl CustomTrait for TestItem {
        fn custom_method(&self) -> String {
            format!("custom_{}", self.0)
        }
    }

    #[test]
    fn test_trait_bounded_struct() {
        let item = TestItem("test".to_string());
        let instance = TraitBounded::builder()
            .item(item.clone())
            .collection(vec![item.clone()])
            .build();

        assert_eq!(instance.item.custom_method(), "custom_test");
        assert_eq!(instance.collection.len(), 1);
        assert_eq!(instance.collection[0].custom_method(), "custom_test");
    }

    // Simple iterator implementation for testing
    struct TestIterator {
        items: Vec<String>,
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.items.len() {
                let item = self.items[self.current].clone();
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    #[test]
    fn test_with_associated_types() {
        let iterator = TestIterator {
            items: vec!["a".to_string(), "b".to_string()],
            current: 0,
        };

        let initial_item = Some("start".to_string());
        let mut instance = WithAssociatedTypes::builder()
            .iterator(iterator)
            .current(initial_item.clone())
            .build();

        // Verify the iterator and current fields are properly set
        assert_eq!(
            instance.iterator.items,
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(instance.iterator.current, 0); // Verify iterator state
        assert_eq!(instance.current, initial_item);

        // Test the iterator's next method
        let first_item = instance.iterator.next();
        assert_eq!(first_item, Some("a".to_string()));
        assert_eq!(instance.iterator.current, 1);
    }
}
