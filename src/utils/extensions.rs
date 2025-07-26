//! Trait Extensions for Common Operations
//!
//! This module provides trait extensions that add commonly used functionality
//! to syn types and other core types, reducing the need for standalone helper
//! functions and improving code organization.
//!
//! # Design Philosophy
//!
//! - **Logical grouping** - Extensions grouped by the type they extend
//! - **Consistent naming** - Method names follow Rust conventions
//! - **Non-intrusive** - Extensions don't change existing API
//! - **Comprehensive coverage** - Cover all common use cases

use std::collections::BTreeSet;
use syn::{GenericParam, Generics, Ident, Type};

/// Extension trait for `syn::Ident` with builder-specific functionality.
///
/// This trait provides methods commonly needed when working with identifiers
/// in the builder pattern context, such as handling raw identifiers and
/// name transformations.
#[allow(dead_code)]
pub trait IdentExt {}

impl IdentExt for Ident {}

/// Extension trait for `syn::Generics` with builder-specific functionality.
///
/// This trait provides methods for analyzing and manipulating generic
/// parameters in the context of builder generation.
#[allow(dead_code)]
pub trait GenericsExt {
    /// Collects all declared generic parameter names.
    ///
    /// This method extracts the names of type and const generic parameters
    /// declared in the generic parameter list, excluding lifetimes.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<String>` containing the names of all declared type and const
    /// generic parameters.
    ///
    fn collect_generic_names(&self) -> BTreeSet<String>;

    /// Collects all declared lifetime parameter names.
    ///
    /// This method extracts the names of lifetime parameters declared
    /// in the generic parameter list.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<String>` containing the names of all declared lifetime
    /// parameters (without the `'` prefix).
    ///
    fn collect_lifetime_names(&self) -> BTreeSet<String>;

    /// Checks if the generics list is empty.
    ///
    /// # Returns
    ///
    /// `true` if there are no generic parameters, `false` otherwise.
    fn is_empty(&self) -> bool;

    /// Checks if the generics contain any type parameters.
    ///
    /// # Returns
    ///
    /// `true` if there are type parameters, `false` otherwise.
    fn has_type_params(&self) -> bool;

    /// Checks if the generics contain any lifetime parameters.
    ///
    /// # Returns
    ///
    /// `true` if there are lifetime parameters, `false` otherwise.
    fn has_lifetime_params(&self) -> bool;

    /// Checks if the generics contain any const parameters.
    ///
    /// # Returns
    ///
    /// `true` if there are const parameters, `false` otherwise.
    fn has_const_params(&self) -> bool;
}

impl GenericsExt for Generics {
    fn collect_generic_names(&self) -> BTreeSet<String> {
        crate::utils::generics::collect_declared_generic_names(self)
    }

    fn collect_lifetime_names(&self) -> BTreeSet<String> {
        self.params
            .iter()
            .filter_map(|param| match param {
                GenericParam::Lifetime(lifetime_param) => {
                    Some(lifetime_param.lifetime.ident.to_string())
                }
                _ => None,
            })
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    fn has_type_params(&self) -> bool {
        self.params
            .iter()
            .any(|param| matches!(param, GenericParam::Type(_)))
    }

    fn has_lifetime_params(&self) -> bool {
        self.params
            .iter()
            .any(|param| matches!(param, GenericParam::Lifetime(_)))
    }

    fn has_const_params(&self) -> bool {
        self.params
            .iter()
            .any(|param| matches!(param, GenericParam::Const(_)))
    }
}

/// Extension trait for `syn::Type` with builder-specific functionality.
///
/// This trait provides methods for analyzing types in the context of
/// builder generation, particularly for generic parameter detection
/// and PhantomData requirements.
#[allow(dead_code)]
pub trait TypeExt {
    /// Checks if this type references any declared generic parameters.
    ///
    /// # Arguments
    ///
    /// * `declared_generics` - Set of declared generic parameter names
    ///
    /// # Returns
    ///
    /// `true` if the type references any of the declared generics.
    fn references_generics(&self, declared_generics: &BTreeSet<String>) -> bool;

    /// Checks if this type is a simple path (single identifier).
    ///
    /// # Returns
    ///
    /// `true` if the type is a simple path like `T` or `String`.
    fn is_simple_path(&self) -> bool;
}

impl TypeExt for Type {
    fn references_generics(&self, declared_generics: &BTreeSet<String>) -> bool {
        match self {
            Type::Path(type_path) => {
                // Check if single identifier is a declared generic
                if type_path.path.segments.len() == 1 {
                    let segment = &type_path.path.segments[0];
                    if segment.arguments.is_empty() {
                        let ident_str = segment.ident.to_string();
                        if declared_generics.contains(&ident_str) {
                            return true;
                        }
                    }
                }

                // Check generic arguments in path segments
                for segment in &type_path.path.segments {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                if inner_ty.references_generics(declared_generics) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            Type::Reference(type_ref) => {
                return type_ref.elem.references_generics(declared_generics);
            }
            Type::Tuple(type_tuple) => {
                for elem in &type_tuple.elems {
                    if elem.references_generics(declared_generics) {
                        return true;
                    }
                }
            }
            Type::Array(type_array) => {
                return type_array.elem.references_generics(declared_generics);
            }
            Type::Slice(type_slice) => {
                return type_slice.elem.references_generics(declared_generics);
            }
            Type::Ptr(type_ptr) => {
                return type_ptr.elem.references_generics(declared_generics);
            }
            _ => {
                // Other types are less common in struct fields
            }
        }

        false
    }

    fn is_simple_path(&self) -> bool {
        match self {
            Type::Path(type_path) => {
                type_path.path.segments.len() == 1
                    && type_path.path.segments[0].arguments.is_empty()
            }
            _ => false,
        }
    }
}

// Helper trait for recursive collection (not public)
trait _UnusedTypeAnalysis {
    fn collect_generic_names_recursive(&self, names: &mut BTreeSet<String>);
    fn collect_lifetime_names_recursive(&self, names: &mut BTreeSet<String>);
}

impl _UnusedTypeAnalysis for Type {
    fn collect_generic_names_recursive(&self, names: &mut BTreeSet<String>) {
        match self {
            Type::Path(type_path) => {
                // Check if single identifier could be a generic
                if type_path.path.segments.len() == 1
                    && type_path.path.segments[0].arguments.is_empty()
                {
                    let ident_str = type_path.path.segments[0].ident.to_string();
                    // Simple heuristic: single uppercase letter is likely a generic
                    // Check if single character is uppercase - safe since we checked length
                    if ident_str.len() == 1
                        && ident_str.chars().next().is_some_and(|c| c.is_uppercase())
                    {
                        names.insert(ident_str);
                    }
                }

                // Recursively check generic arguments
                for segment in &type_path.path.segments {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                inner_ty.collect_generic_names_recursive(names);
                            }
                        }
                    }
                }
            }
            Type::Reference(type_ref) => {
                type_ref.elem.collect_generic_names_recursive(names);
            }
            Type::Tuple(type_tuple) => {
                for elem in &type_tuple.elems {
                    elem.collect_generic_names_recursive(names);
                }
            }
            Type::Array(type_array) => {
                type_array.elem.collect_generic_names_recursive(names);
            }
            Type::Slice(type_slice) => {
                type_slice.elem.collect_generic_names_recursive(names);
            }
            Type::Ptr(type_ptr) => {
                type_ptr.elem.collect_generic_names_recursive(names);
            }
            _ => {}
        }
    }

    fn collect_lifetime_names_recursive(&self, names: &mut BTreeSet<String>) {
        match self {
            Type::Reference(type_ref) => {
                if let Some(lifetime) = &type_ref.lifetime {
                    names.insert(lifetime.ident.to_string());
                }
                type_ref.elem.collect_lifetime_names_recursive(names);
            }
            Type::Path(type_path) => {
                for segment in &type_path.path.segments {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        for arg in &args.args {
                            match arg {
                                syn::GenericArgument::Type(inner_ty) => {
                                    inner_ty.collect_lifetime_names_recursive(names);
                                }
                                syn::GenericArgument::Lifetime(lifetime) => {
                                    names.insert(lifetime.ident.to_string());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Type::Tuple(type_tuple) => {
                for elem in &type_tuple.elems {
                    elem.collect_lifetime_names_recursive(names);
                }
            }
            Type::Array(type_array) => {
                type_array.elem.collect_lifetime_names_recursive(names);
            }
            Type::Slice(type_slice) => {
                type_slice.elem.collect_lifetime_names_recursive(names);
            }
            Type::Ptr(type_ptr) => {
                type_ptr.elem.collect_lifetime_names_recursive(names);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    // #[test]
    // fn test_ident_ext_strip_raw_prefix() {
    //     let raw_ident: Ident = parse_quote!(r#type);
    //     assert_eq!(raw_ident.strip_raw_prefix(), "type");
    //
    //     let normal_ident: Ident = parse_quote!(field_name);
    //     assert_eq!(normal_ident.strip_raw_prefix(), "field_name");
    // }

    // #[test]
    // fn test_ident_ext_capitalize_first() {
    //     let ident: Ident = parse_quote!(field_name);
    //     assert_eq!(ident.capitalize_first(), "Field_name");
    // }

    // #[test]
    // fn test_ident_ext_is_raw_identifier() {
    //     let raw_ident: Ident = parse_quote!(r#type);
    //     assert!(raw_ident.is_raw_identifier());
    //
    //     let normal_ident: Ident = parse_quote!(field_name);
    //     assert!(!normal_ident.is_raw_identifier());
    // }

    #[test]
    fn test_generics_ext_collect_names() {
        let generics: Generics = parse_quote!(<'a, T: Clone, U, const N: usize>);

        let generic_names = generics.collect_generic_names();
        assert!(generic_names.contains("T"));
        assert!(generic_names.contains("U"));
        assert!(generic_names.contains("N"));
        assert_eq!(generic_names.len(), 3);

        let lifetime_names = generics.collect_lifetime_names();
        assert!(lifetime_names.contains("a"));
        assert_eq!(lifetime_names.len(), 1);
    }

    #[test]
    fn test_generics_ext_predicates() {
        let generics: Generics = parse_quote!(<'a, T: Clone, const N: usize>);

        assert!(!generics.is_empty());
        assert!(generics.has_type_params());
        assert!(generics.has_lifetime_params());
        assert!(generics.has_const_params());

        let empty_generics: Generics = parse_quote!();
        assert!(empty_generics.is_empty());
        assert!(!empty_generics.has_type_params());
    }

    #[test]
    fn test_type_ext_references_generics() {
        let mut declared_generics = BTreeSet::new();
        declared_generics.insert("T".to_string());

        let generic_type: Type = parse_quote!(T);
        assert!(generic_type.references_generics(&declared_generics));

        let concrete_type: Type = parse_quote!(String);
        assert!(!concrete_type.references_generics(&declared_generics));

        let complex_type: Type = parse_quote!(Vec<T>);
        assert!(complex_type.references_generics(&declared_generics));
    }

    #[test]
    fn test_type_ext_is_simple_path() {
        let simple_type: Type = parse_quote!(T);
        assert!(simple_type.is_simple_path());

        let complex_type: Type = parse_quote!(Vec<T>);
        assert!(!complex_type.is_simple_path());

        let reference_type: Type = parse_quote!(&T);
        assert!(!reference_type.is_simple_path());
    }
}
