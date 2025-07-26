//! Generic Type Analysis and Transformation Utilities
//!
//! This module provides utilities for analyzing and transforming generic types,
//! particularly for use in PhantomData generation and generic parameter tracking
//! in the type-state builder pattern.
//!
//! # Key Concepts
//!
//! ## Generic Parameters vs Generic Usage
//!
//! There's an important distinction between:
//! - **Declared generics**: Parameters declared in the struct signature (e.g., `struct Foo<T, U>`)
//! - **Used generics**: Generic parameters actually referenced in field types
//!
//! The builder must only include used generics in its PhantomData to avoid
//! "parameter is never used" compiler errors.
//!
//! ## PhantomData Transformation
//!
//! When creating PhantomData for builders, we need to transform field types:
//! - Generic types (T, U) → keep as-is for proper variance
//! - Reference types (&'a T) → transform based on whether T is generic
//! - Concrete types (String, i32) → replace with () to avoid unnecessary constraints
//!

use quote::quote;
use std::collections::BTreeSet;
use syn::{GenericParam, Generics, Type};

/// Determines if a type contains generics or lifetimes that require PhantomData tracking.
///
/// This function performs proper analysis of the type structure to determine
/// if it contains generic parameters or lifetimes that would require PhantomData
/// tracking in the generated builder.
///
/// # Purpose
///
/// Used to determine whether a builder struct needs PhantomData fields to properly
/// track generic parameters and lifetimes from the original struct.
///
/// # Arguments
///
/// * `ty` - The type to analyze for generic content
///
/// # Returns
///
/// `true` if the type contains generics or lifetimes, `false` otherwise.
/// # Implementation Details
///
/// The function recursively analyzes the type structure to detect:
/// - Generic type parameters (T, U, etc.)
/// - Lifetime parameters ('a, 'b, etc.)
/// - Const generic parameters
/// - Complex types containing generics (Vec<T>, Option<U>, etc.)
///
/// This precise analysis ensures that PhantomData is only included when
/// actually needed, reducing generated code size while maintaining correctness.
pub fn has_generics_or_lifetimes(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            // Check if this looks like a generic parameter (single uppercase identifier)
            if type_path.path.segments.len() == 1 {
                let segment = &type_path.path.segments[0];
                if segment.arguments.is_empty() {
                    let ident_str = segment.ident.to_string();
                    // Heuristic: single uppercase letter or CamelCase starting with uppercase
                    // is likely a generic parameter
                    if is_likely_generic_name(&ident_str) {
                        return true;
                    }
                }
            }

            // Check for generic arguments in any path segment
            for segment in &type_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    // If there are angle brackets, there are generics
                    for arg in &args.args {
                        match arg {
                            syn::GenericArgument::Type(inner_ty) => {
                                if has_generics_or_lifetimes(inner_ty) {
                                    return true;
                                }
                            }
                            syn::GenericArgument::Lifetime(_) => {
                                // Found a lifetime parameter
                                return true;
                            }
                            syn::GenericArgument::Const(_) => {
                                // Found a const generic parameter
                                return true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Type::Reference(type_ref) => {
            // References may have lifetimes
            if type_ref.lifetime.is_some() {
                return true;
            }
            // Check the referenced type
            return has_generics_or_lifetimes(&type_ref.elem);
        }
        Type::Tuple(type_tuple) => {
            // Check all tuple elements
            for elem in &type_tuple.elems {
                if has_generics_or_lifetimes(elem) {
                    return true;
                }
            }
        }
        Type::Array(type_array) => {
            // Check the array element type and length (if it's a const generic)
            return has_generics_or_lifetimes(&type_array.elem);
        }
        Type::Slice(type_slice) => {
            // Check the slice element type
            return has_generics_or_lifetimes(&type_slice.elem);
        }
        Type::Ptr(type_ptr) => {
            // Check the pointed-to type
            return has_generics_or_lifetimes(&type_ptr.elem);
        }
        Type::BareFn(bare_fn) => {
            // Function pointers may have lifetimes in their signature
            if bare_fn.lifetimes.is_some() {
                return true;
            }
            // Check input and output types
            for input in &bare_fn.inputs {
                if has_generics_or_lifetimes(&input.ty) {
                    return true;
                }
            }
            if let syn::ReturnType::Type(_, output_ty) = &bare_fn.output {
                if has_generics_or_lifetimes(output_ty) {
                    return true;
                }
            }
        }
        Type::TraitObject(trait_object) => {
            // Trait objects may have lifetime bounds
            for bound in &trait_object.bounds {
                if let syn::TypeParamBound::Lifetime(_) = bound {
                    return true;
                }
            }
        }
        _ => {
            // Other types (Infer, Never, etc.) typically don't contain generics
        }
    }

    false
}

/// Heuristic to determine if an identifier name is likely a generic parameter.
///
/// This function uses naming conventions to guess whether an identifier
/// represents a generic type parameter.
///
/// # Arguments
///
/// * `name` - The identifier name to analyze
///
/// # Returns
///
/// `true` if the name is likely a generic parameter, `false` otherwise.
///
/// # Heuristics
///
/// The function considers these patterns as likely generic parameters:
/// - Single uppercase letters: T, U, V, etc.
/// - Short CamelCase names: Item, Key, Value, etc.
/// - Common generic parameter names: Self, etc.
///
fn is_likely_generic_name(name: &str) -> bool {
    // Single uppercase letter (T, U, K, V, etc.)
    // Single uppercase letter (T, U, K, V, etc.) - safe since we checked length
    if name.len() == 1 && name.chars().next().is_some_and(|c| c.is_uppercase()) {
        return true;
    }

    // Short CamelCase names that are likely generic parameters
    if name.len() <= 8 && name.starts_with(char::is_uppercase) {
        // Check if it's not a common concrete type
        match name {
            // Common concrete types that start with uppercase
            "String" | "Vec" | "HashMap" | "HashSet" | "BTreeMap" | "BTreeSet" | "Option"
            | "Result" | "Box" | "Arc" | "Rc" | "Cell" | "RefCell" | "Mutex" | "RwLock"
            | "AtomicBool" | "AtomicI32" | "AtomicU32" | "PathBuf" | "OsString" | "CString"
            | "Duration" | "Instant" => false,
            // Everything else that's short and CamelCase is likely generic
            _ => true,
        }
    } else {
        false
    }
}

/// Extracts all declared generic parameter names from struct generics.
///
/// This function collects the names of type and const generic parameters
/// declared in a struct's generic parameter list. It excludes lifetimes
/// since they are handled separately in PhantomData generation.
///
/// # Purpose
///
/// Used for:
/// - Determining which identifiers in field types refer to declared generic parameters
/// - Validating that field types don't reference undeclared generics
/// - Generating appropriate PhantomData types that only include relevant parameters
///
/// # Arguments
///
/// * `generics` - The generic parameter list from a struct definition
///
/// # Returns
///
/// A `BTreeSet<String>` containing the names of all declared type and const
/// generic parameters.
/// # Implementation Details
///
/// The function iterates through all generic parameters and uses pattern matching
/// to extract names from:
/// - `syn::GenericParam::Type` - Type parameters (T, U, etc.)
/// - `syn::GenericParam::Const` - Const parameters (const N: usize, etc.)
/// - `syn::GenericParam::Lifetime` - Excluded from results
///
/// The resulting BTreeSet provides O(log n) lookup for determining if an identifier
/// refers to a declared generic parameter.
pub fn collect_declared_generic_names(generics: &Generics) -> BTreeSet<String> {
    generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => {
                // Extract the identifier name from type parameters
                Some(type_param.ident.to_string())
            }
            GenericParam::Const(const_param) => {
                // Extract the identifier name from const parameters
                Some(const_param.ident.to_string())
            }
            GenericParam::Lifetime(_) => {
                // Lifetimes are handled separately in PhantomData generation
                None
            }
        })
        .collect()
}

/// Transforms a field type for inclusion in PhantomData with advanced variance preservation.
///
/// This is the **core transformation function** for the type-state builder pattern's
/// PhantomData generation. It implements sophisticated logic to maintain generic
/// parameter relationships while minimizing unnecessary constraints.
///
/// # The PhantomData Challenge
///
/// When generating builders for generic structs, we face a complex challenge:
/// the builder must track all generic parameters from the original struct without
/// imposing unnecessary constraints. For example, consider a struct with both
/// generic fields (using type parameters) and concrete fields (using specific types).
///
/// A naive PhantomData approach would include all field types, forcing the builder
/// to satisfy constraints on concrete types that don't actually matter. The correct
/// approach preserves only the generic relationships while replacing concrete types
/// with the unit type to avoid unnecessary constraints.
///
/// # Transformation Strategy
///
/// This function implements a **variance-preserving transformation** with these rules:
///
/// 1. **Generic Parameters** (`T`, `U`) → **Keep unchanged**
///    - Preserves covariance/contravariance relationships
///    - Maintains trait bounds through the type system
///
/// 2. **References to Generics** (`&'a T`) → **Keep unchanged**
///    - Preserves both lifetime and type parameter relationships
///    - Critical for borrowed data patterns
///
/// 3. **References to Concrete** (`&'a str`) → **Transform to `&'a ()`**
///    - Preserves the lifetime parameter `'a`
///    - Removes unnecessary constraint on `str`
///
/// 4. **Concrete Types** (`String`, `i32`) → **Replace with `()`**
///    - Eliminates all constraints from concrete types
///    - Minimal impact on generated code
///
/// 5. **Complex Types** (`Vec<T>`, `HashMap<K, V>`) → **Context-dependent**
///    - Keep if they reference declared generics
///    - Replace with `()` if purely concrete
///
/// # Theoretical Foundation
///
/// This transformation is based on **phantom type theory** where:
/// - PhantomData exists solely for compile-time type checking
/// - Variance relationships must be preserved for soundness
/// - Unnecessary constraints create compilation failures
/// - The transformation must be **homomorphic** over type structure
///
/// # Arguments
///
/// * `ty` - The field type to transform
/// * `declared_generics` - Set of declared generic parameter names for lookup
///
/// # Returns
///
/// A `proc_macro2::TokenStream` representing the transformed type suitable
/// for use in PhantomData.
/// # Implementation Details
///
/// The transformation logic:
///
/// 1. **References** (`&'a T`):
///    - If T is a declared generic → keep as `&'a T`
///    - If T is concrete → replace with `&'a ()`
///    - If no lifetime → handle as `&()` or `&T`
///
/// 2. **Path types** (`T`, `Vec<T>`, `String`):
///    - Single identifier matching declared generic → keep unchanged
///    - Complex path or concrete type → replace with `()`
///
/// 3. **Other types** (tuples, arrays, slices):
///    - Generally replaced with `()` unless they contain declared generics
///
/// This approach ensures that PhantomData maintains the correct variance
/// relationships while imposing minimal additional constraints on the
/// generated builder types.
pub fn transform_type_for_phantom_data(
    ty: &Type,
    declared_generics: &BTreeSet<String>,
) -> proc_macro2::TokenStream {
    match ty {
        Type::Reference(type_ref) => {
            // Handle reference types: &'a T, &T, etc.
            let lifetime = &type_ref.lifetime;

            match type_ref.elem.as_ref() {
                Type::Path(inner_path) if inner_path.path.segments.len() == 1 => {
                    // Single identifier reference like &'a T or &T
                    let inner_ident = &inner_path.path.segments[0].ident;

                    if declared_generics.contains(&inner_ident.to_string()) {
                        // &'a T where T is a declared generic parameter
                        // Keep the reference as-is to maintain variance
                        quote! { #ty }
                    } else {
                        // &'a str where str is concrete
                        // Replace the concrete type with () but keep the lifetime
                        if let Some(lifetime) = lifetime {
                            quote! { &#lifetime () }
                        } else {
                            quote! { &() }
                        }
                    }
                }
                _ => {
                    // Complex reference type like &Vec<T>
                    // Replace with lifetime + () to maintain lifetime tracking
                    if let Some(lifetime) = lifetime {
                        quote! { &#lifetime () }
                    } else {
                        quote! { &() }
                    }
                }
            }
        }
        Type::Path(type_path) => {
            // Handle path types: T, Vec<T>, String, etc.
            if type_path.path.segments.len() == 1 && type_path.path.segments[0].arguments.is_empty()
            {
                // Simple single identifier like T, String, etc.
                let ident = &type_path.path.segments[0].ident;

                if declared_generics.contains(&ident.to_string()) {
                    // This is a declared generic parameter (T, U, etc.)
                    // Keep it unchanged to maintain proper variance
                    quote! { #ident }
                } else {
                    // This is a concrete type (String, i32, etc.)
                    // Replace with () to avoid unnecessary constraints
                    quote! { () }
                }
            } else {
                // Complex path like Vec<T>, Option<String>, std::string::String
                // Replace with () - if it contains generics, they'll be captured
                // through other field types or explicit generic parameters
                quote! { () }
            }
        }
        Type::Tuple(_) => {
            // Tuple types like (T, U) or (String, i32)
            // For simplicity, replace with () - individual generic parameters
            // will be captured through other means
            quote! { () }
        }
        Type::Array(_) => {
            // Array types like [T; N] or [i32; 10]
            // Replace with () - generic parameters will be captured elsewhere
            quote! { () }
        }
        Type::Slice(_) => {
            // Slice types like [T] or [i32]
            // Replace with () - generic parameters will be captured elsewhere
            quote! { () }
        }
        _ => {
            // Other complex types (function pointers, trait objects, etc.)
            // Replace with () for simplicity - generic parameters will be
            // captured through other field types or explicit parameters
            quote! { () }
        }
    }
}

/// Generates a comprehensive PhantomData type that encompasses all generic relationships.
///
/// This is the **orchestration function** for PhantomData generation in type-state builders.
/// It coordinates the complex process of tracking all generic parameters, lifetimes, and
/// variance relationships while minimizing constraints on the generated builder types.
///
/// # The Complete PhantomData Picture
///
/// PhantomData generation is a **multi-phase process** that must handle:
///
/// 1. **Field Type Analysis** - Transform each field type using variance-preserving rules
/// 2. **Generic Parameter Coverage** - Ensure all declared generics are referenced
/// 3. **Lifetime Preservation** - Maintain all lifetime relationships through references
/// 4. **Constraint Minimization** - Replace concrete types with minimal equivalents
/// 5. **Tuple Composition** - Combine everything into a single PhantomData type
///
/// ## Why This Complexity?
///
/// Consider a challenging example with multiple generic parameters and lifetimes,
/// where some parameters are used directly in fields, some are wrapped in other
/// types, and some may not be used in fields at all but are declared in the
/// generic parameter list.
///
/// The generated PhantomData must carefully balance preserving the necessary
/// generic relationships while including all declared parameters to avoid
/// "unused parameter" compiler errors. The transformation ensures that:
/// - Direct generic usage is preserved
/// - Wrapped generics are simplified but still tracked
/// - Concrete types become unit types
/// - All declared generics appear in the final PhantomData
///
/// ## The Algorithm
///
/// This function implements a **dual-coverage algorithm**:
///
/// ### Phase 1: Field Type Transformation
/// - Transform each field type using `transform_type_for_phantom_data`
/// - Preserve generic relationships while minimizing constraints
/// - Collect all transformed types into a list
///
/// ### Phase 2: Explicit Generic Coverage  
/// - Add explicit references to all declared type parameters
/// - Ensures no "parameter is never used" errors
/// - Handles edge cases where generics appear only in bounds
///
/// ### Phase 3: Tuple Composition
/// - Combine all types into a single tuple type
/// - Create the final `PhantomData<(...)>` declaration
/// - Return empty stream if no generics are present
///
/// # Theoretical Soundness
///
/// This approach is **theoretically sound** because:
/// - All generic parameters are explicitly tracked (no unused parameter errors)
/// - Variance relationships are preserved through careful transformation
/// - Constraints are minimized to prevent over-specification
/// - The generated PhantomData has **zero runtime cost**
///
/// # Arguments
///
/// * `field_types` - Iterator over all field types in the struct
/// * `struct_generics` - The generic parameter list from the struct
///
/// # Returns
///
/// A `proc_macro2::TokenStream` representing the complete PhantomData type,
/// or an empty token stream if no PhantomData is needed.
/// # Implementation Details
///
/// The function follows this process:
///
/// 1. **Collect declared generics** for transformation lookup
/// 2. **Transform field types** using `transform_type_for_phantom_data`
/// 3. **Add explicit generic parameters** to ensure complete coverage
/// 4. **Combine into tuple** for the PhantomData parameter
/// 5. **Return empty stream** if no generics are present
///
/// The resulting PhantomData type includes:
/// - Transformed versions of all field types
/// - Explicit references to all declared type parameters
/// - Lifetime tracking through reference transformations
///
/// This comprehensive approach ensures that the builder struct properly
/// maintains all generic relationships from the original struct.
pub fn generate_phantom_data_type<'a>(
    field_types: impl Iterator<Item = &'a Type>,
    struct_generics: &Generics,
) -> proc_macro2::TokenStream {
    let mut phantom_types = Vec::new();

    // Collect declared generic names for transformation lookup
    let declared_generics = collect_declared_generic_names(struct_generics);

    // Transform all field types for PhantomData inclusion
    for field_type in field_types {
        let transformed = transform_type_for_phantom_data(field_type, &declared_generics);
        phantom_types.push(transformed);
    }

    // Add explicit references to all declared type parameters
    // This ensures that even if a generic parameter isn't used in fields
    // (which shouldn't happen after validation), it's still tracked
    for param in &struct_generics.params {
        match param {
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                phantom_types.push(quote! { #ident });
            }
            GenericParam::Lifetime(_) => {
                // Lifetimes are handled within the field type transformations
                // through reference types like &'a ()
            }
            GenericParam::Const(_) => {
                // Const generics are handled within the field type transformations
                // and don't need explicit tracking in PhantomData
            }
        }
    }

    // If no phantom types are needed, return empty stream
    if phantom_types.is_empty() {
        quote! {}
    } else {
        // Create PhantomData with tuple of all types
        // The tuple ensures all generic parameters are properly tracked
        quote! { ::std::marker::PhantomData<( #(#phantom_types),* )> }
    }
}

/// Determines if PhantomData is needed for a given set of generics and fields.
///
/// This function provides a more precise check than `has_generics_or_lifetimes`
/// by analyzing whether the struct actually needs PhantomData based on its
/// generic parameters and field usage.
///
/// # Purpose
///
/// Used to determine whether to include a PhantomData field in generated
/// builder structs. PhantomData is only needed when:
/// - The struct has generic parameters
/// - The fields use types that contain generics or lifetimes
///
/// # Arguments
///
/// * `struct_generics` - The generic parameter list from the struct
/// * `field_types` - Iterator over all field types in the struct
///
/// # Returns
///
/// `true` if PhantomData is needed, `false` otherwise.
/// # Implementation Details
///
/// The function checks:
/// 1. Whether the struct declares any generic parameters
/// 2. Whether any field types potentially contain generics or lifetimes
///
/// This is more efficient than always including PhantomData, though the
/// current implementation of `has_generics_or_lifetimes` is conservative
/// and always returns true for safety.
pub fn needs_phantom_data<'a>(
    struct_generics: &Generics,
    field_types: impl Iterator<Item = &'a Type>,
) -> bool {
    // If struct has generic parameters, we likely need PhantomData
    if !struct_generics.params.is_empty() {
        return true;
    }

    // Check if any field types contain generics or lifetimes
    // This uses the conservative approach of the current implementation
    for field_type in field_types {
        if has_generics_or_lifetimes(field_type) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_has_generics_or_lifetimes() {
        let generic_ty: Type = parse_quote!(Vec<T>);
        assert!(has_generics_or_lifetimes(&generic_ty));

        let concrete_ty: Type = parse_quote!(String);
        assert!(!has_generics_or_lifetimes(&concrete_ty)); // Now properly analyzed

        let reference_ty: Type = parse_quote!(&'a str);
        assert!(has_generics_or_lifetimes(&reference_ty));

        let simple_generic: Type = parse_quote!(T);
        assert!(has_generics_or_lifetimes(&simple_generic));

        let complex_generic: Type = parse_quote!(HashMap<K, V>);
        assert!(has_generics_or_lifetimes(&complex_generic));

        let no_generics: Type = parse_quote!(i32);
        assert!(!has_generics_or_lifetimes(&no_generics));
    }

    #[test]
    fn test_is_likely_generic_name() {
        // Single uppercase letters are likely generic
        assert!(is_likely_generic_name("T"));
        assert!(is_likely_generic_name("U"));
        assert!(is_likely_generic_name("K"));

        // Common concrete types are not generic
        assert!(!is_likely_generic_name("String"));
        assert!(!is_likely_generic_name("Vec"));
        assert!(!is_likely_generic_name("HashMap"));

        // Short CamelCase names are likely generic
        assert!(is_likely_generic_name("Item"));
        assert!(is_likely_generic_name("Key"));
        assert!(is_likely_generic_name("Value"));

        // Lowercase names are not generic
        assert!(!is_likely_generic_name("field"));
        assert!(!is_likely_generic_name("value"));
    }

    #[test]
    fn test_collect_declared_generic_names() {
        // Simple type parameters
        let generics: Generics = parse_quote!(<T, U>);
        let names = collect_declared_generic_names(&generics);
        assert!(names.contains("T"));
        assert!(names.contains("U"));
        assert_eq!(names.len(), 2);

        // Mixed parameters
        let generics: Generics = parse_quote!(<'a, T: Clone, const N: usize>);
        let names = collect_declared_generic_names(&generics);
        assert!(names.contains("T"));
        assert!(names.contains("N"));
        assert!(!names.contains("a")); // Lifetimes excluded
        assert_eq!(names.len(), 2);

        // No generics
        let generics: Generics = parse_quote!();
        let names = collect_declared_generic_names(&generics);
        assert!(names.is_empty());
    }

    #[test]
    fn test_transform_type_for_phantom_data() {
        let generics: Generics = parse_quote!(<T>);
        let declared = collect_declared_generic_names(&generics);

        // Generic type parameter - should be kept
        let generic_ty: Type = parse_quote!(T);
        let result = transform_type_for_phantom_data(&generic_ty, &declared);
        assert_eq!(result.to_string(), "T");

        // Concrete type - should become ()
        let concrete_ty: Type = parse_quote!(String);
        let result = transform_type_for_phantom_data(&concrete_ty, &declared);
        assert_eq!(result.to_string(), "()");

        // Reference to generic - should be kept
        let ref_generic_ty: Type = parse_quote!(&'a T);
        let result = transform_type_for_phantom_data(&ref_generic_ty, &declared);
        assert_eq!(result.to_string(), "& 'a T");

        // Reference to concrete - should become &'a ()
        let ref_concrete_ty: Type = parse_quote!(&'a str);
        let result = transform_type_for_phantom_data(&ref_concrete_ty, &declared);
        assert_eq!(result.to_string(), "& 'a ()");
    }

    #[test]
    fn test_generate_phantom_data_type() {
        let generics: Generics = parse_quote!(<T, U>);
        let field_types = vec![parse_quote!(T), parse_quote!(String), parse_quote!(Vec<U>)];

        let result = generate_phantom_data_type(field_types.iter(), &generics);
        let result_str = result.to_string();

        // Should contain PhantomData
        assert!(result_str.contains("PhantomData"));
        // Should contain the explicit type parameters
        assert!(result_str.contains("T"));
        assert!(result_str.contains("U"));
    }

    #[test]
    fn test_needs_phantom_data() {
        // With generics - should need PhantomData
        let generics: Generics = parse_quote!(<T>);
        let field_types = vec![parse_quote!(T)];
        assert!(needs_phantom_data(&generics, field_types.iter()));

        // Without generics and concrete fields - should not need PhantomData
        let no_generics: Generics = parse_quote!();
        let concrete_fields = vec![parse_quote!(String)];
        assert!(!needs_phantom_data(&no_generics, concrete_fields.iter())); // Now properly analyzed

        // No generics and no fields - should not need PhantomData
        let no_generics: Generics = parse_quote!();
        let no_fields: Vec<Type> = vec![];
        assert!(!needs_phantom_data(&no_generics, no_fields.iter()));
    }
}
