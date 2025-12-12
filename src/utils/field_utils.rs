//! Field Processing Utilities
//!
//! This module provides utilities for processing and manipulating field information
//! during builder generation. It centralizes common field-related operations that
//! are used across different parts of the builder generation process.
//!
//! # Key Concepts
//!
//! ## Field Processing Workflow
//!
//! Field processing typically follows this pattern:
//! 1. **Parse attributes** - Extract builder-specific attributes from field
//! 2. **Generate documentation** - Create appropriate doc comments
//! 3. **Handle defaults** - Process default values and skip_setter logic
//! 4. **Create setters** - Generate setter method tokens
//! 5. **Handle initialization** - Generate field initialization code
//!
//! ## Setter Configuration
//!
//! The `SetterConfig` type encapsulates all the information needed to generate
//! a setter method, including custom names, documentation, and special handling.
//!

use std::borrow::Cow;

/// Configuration for generating setter methods.
///
/// This struct encapsulates all the information needed to generate a setter
/// method for a field, including naming, documentation, and special handling.
///
/// # Fields
///
/// * `setter_name` - The name to use for the setter method
/// * `field_name` - The original field name (may be raw identifier)
/// * `clean_name` - The field name with raw identifier prefix stripped
/// * `skip_setter` - Whether to skip generating a setter for this field
/// * `doc_comment` - Documentation comment for the setter method
#[derive(Debug, Clone)]
pub struct SetterConfig {
    /// The identifier to use for the setter method
    pub setter_name: Cow<'static, str>,
    /// The original field identifier (may include r# prefix)
    pub _field_name: Cow<'static, str>,
    /// The field name with raw identifier prefix removed
    pub _clean_name: Cow<'static, str>,
    /// Whether to skip generating a setter method
    pub skip_setter: bool,
    /// Documentation comment for the setter method
    pub doc_comment: String,
}

/// Configuration for field default value handling.
///
/// This struct contains information about how to initialize a field with
/// its default value, including custom default expressions and fallback
/// behavior.
///
/// # Fields
///
/// * `has_custom_default` - Whether the field has a custom default value
/// * `default_expression` - The default value expression, if any
/// * `requires_default_trait` - Whether the field type needs Default::default()
#[derive(Debug, Clone)]
pub struct DefaultConfig {
    /// Whether the field has a custom default value specified
    pub _has_custom_default: bool,
    /// The custom default expression, if provided
    pub default_expression: Option<syn::Expr>,
    /// Whether to use Default::default() for initialization
    pub _requires_default_trait: bool,
}

/// Resolves the effective `impl_into` setting for a field.
///
/// This function determines whether a field's setter should use `impl Into<FieldType>`
/// parameters by considering both struct-level and field-level `impl_into` settings.
/// Field-level settings take precedence over struct-level settings.
///
/// # Arguments
///
/// * `field_impl_into` - The field-level `impl_into` setting (None = inherit from struct)
/// * `struct_impl_into` - The struct-level `impl_into` setting
///
/// # Returns
///
/// A `bool` indicating whether the field's setter should use `impl Into<T>` parameters:
/// - `true` - Setter accepts `impl Into<FieldType>`
/// - `false` - Setter accepts `FieldType` directly
///
/// # Precedence Rules
///
/// 1. If field has explicit `impl_into = true/false`, use that value
/// 2. Otherwise, inherit the struct-level `impl_into` setting
///
/// See the crate-level documentation for usage examples.
pub fn resolve_effective_impl_into(field_impl_into: Option<bool>, struct_impl_into: bool) -> bool {
    field_impl_into.unwrap_or(struct_impl_into)
}

/// Configuration for setter parameter type and field assignment generation.
///
/// This struct contains the information needed to generate setter method
/// parameters and field assignments, supporting both regular setters,
/// impl_into setters, and custom setter functions.
#[derive(Debug, Clone)]
pub struct SetterParameterConfig {
    /// The parameter type for the setter method
    pub param_type: proc_macro2::TokenStream,
    /// The expression to assign to the field (e.g., `value` or `custom_fn(value)`)
    pub field_assignment_expr: proc_macro2::TokenStream,
}

/// Determines setter parameter configuration based on field attributes.
///
/// This function centralizes the logic for determining how to generate setter
/// method parameters and field assignments. It handles three cases:
/// 1. Custom setter function - calls the function with the parameter
/// 2. impl_into enabled - uses `impl Into<FieldType>` parameter with `.into()`
/// 3. Regular setter - uses direct field type parameter
///
/// # Arguments
///
/// * `field_type` - The type of the field being set
/// * `converter` - Optional custom converter closure expression
/// * `use_impl_into` - Whether to use `impl Into<T>` parameters
///
/// # Returns
///
/// A `SetterParameterConfig` containing the parameter type and assignment expression.
pub fn resolve_setter_parameter_config(
    field_type: &syn::Type,
    converter: Option<&syn::Expr>,
    use_impl_into: bool,
) -> SetterParameterConfig {
    if let Some(converter_expr) = converter {
        // Custom converter case - extract parameter type from closure
        let param_type = extract_closure_parameter_type(converter_expr).unwrap_or_else(
            || quote::quote! { /* Error: Unable to parse closure parameter type */ },
        );

        SetterParameterConfig {
            param_type,
            field_assignment_expr: quote::quote! { (#converter_expr)(value) },
        }
    } else if use_impl_into {
        // impl_into case
        SetterParameterConfig {
            param_type: quote::quote! { impl Into<#field_type> },
            field_assignment_expr: quote::quote! { value.into() },
        }
    } else {
        // Regular setter case
        SetterParameterConfig {
            param_type: quote::quote! { #field_type },
            field_assignment_expr: quote::quote! { value },
        }
    }
}

/// Extracts the parameter type from a closure expression.
///
/// This function parses a closure expression like `|value: Vec<&str>| ...`
/// and extracts the parameter type `Vec<&str>`.
///
/// # Arguments
///
/// * `expr` - The closure expression to parse
///
/// # Returns
///
/// An `Option<proc_macro2::TokenStream>` containing the parameter type,
/// or `None` if the expression is not a valid closure or the parameter type cannot be extracted.
fn extract_closure_parameter_type(expr: &syn::Expr) -> Option<proc_macro2::TokenStream> {
    match expr {
        syn::Expr::Closure(closure) => {
            // Get the first parameter (we expect exactly one parameter)
            if let Some(first_param) = closure.inputs.first() {
                match first_param {
                    syn::Pat::Type(pat_type) => {
                        // Extract the type from |value: Type| pattern
                        let param_type = &pat_type.ty;
                        Some(quote::quote! { #param_type })
                    }
                    _ => {
                        // Parameter doesn't have an explicit type annotation
                        None
                    }
                }
            } else {
                // No parameters in closure
                None
            }
        }
        _ => {
            // Not a closure expression
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_effective_impl_into_field_inherits_struct() {
        // Field inherits struct setting when field is None
        assert!(resolve_effective_impl_into(None, true));
        assert!(!resolve_effective_impl_into(None, false));
    }

    #[test]
    fn test_resolve_effective_impl_into_field_overrides_struct() {
        // Field setting overrides struct setting when field is Some
        assert!(resolve_effective_impl_into(Some(true), false));
        assert!(!resolve_effective_impl_into(Some(false), true));
        assert!(resolve_effective_impl_into(Some(true), true));
        assert!(!resolve_effective_impl_into(Some(false), false));
    }

    #[test]
    fn test_resolve_effective_impl_into_precedence() {
        // Test all combinations to ensure field-level always takes precedence
        let test_cases = [
            // (field_impl_into, struct_impl_into, expected)
            (None, true, true),          // inherit struct true
            (None, false, false),        // inherit struct false
            (Some(true), true, true),    // field true, struct true
            (Some(true), false, true),   // field true overrides struct false
            (Some(false), true, false),  // field false overrides struct true
            (Some(false), false, false), // field false, struct false
        ];

        for (field_setting, struct_setting, expected) in test_cases {
            let result = resolve_effective_impl_into(field_setting, struct_setting);
            assert_eq!(
                result, expected,
                "Failed for field_impl_into={field_setting:?}, struct_impl_into={struct_setting}, expected={expected}"
            );
        }
    }

    // Tests for resolve_setter_parameter_config

    #[test]
    fn test_resolve_setter_parameter_config_converter_closure() {
        let field_type: syn::Type = syn::parse_quote!(Vec<String>);
        let converter: syn::Expr = syn::parse_quote!(|values: Vec<&str>| values
            .into_iter()
            .map(|s| s.to_string())
            .collect());

        let config = resolve_setter_parameter_config(
            &field_type,
            Some(&converter),
            false, // use_impl_into should be ignored when converter is provided
        );

        // Should use the extracted parameter type from the closure
        assert_eq!(config.param_type.to_string(), "Vec < & str >");
        // Should call the converter closure
        assert_eq!(config.field_assignment_expr.to_string(), "(| values : Vec < & str > | values . into_iter () . map (| s | s . to_string ()) . collect ()) (value)");
    }

    #[test]
    fn test_resolve_setter_parameter_config_converter_ignores_impl_into() {
        let field_type: syn::Type = syn::parse_quote!(String);
        let converter: syn::Expr = syn::parse_quote!(|input: &str| input.to_uppercase());

        // Even with use_impl_into = true, should prioritize converter
        let config = resolve_setter_parameter_config(&field_type, Some(&converter), true);

        assert_eq!(config.param_type.to_string(), "& str");
        assert_eq!(
            config.field_assignment_expr.to_string(),
            "(| input : & str | input . to_uppercase ()) (value)"
        );
    }

    #[test]
    fn test_resolve_setter_parameter_config_impl_into() {
        let field_type: syn::Type = syn::parse_quote!(String);

        let config = resolve_setter_parameter_config(&field_type, None, true);

        assert_eq!(config.param_type.to_string(), "impl Into < String >");
        assert_eq!(config.field_assignment_expr.to_string(), "value . into ()");
    }

    #[test]
    fn test_resolve_setter_parameter_config_regular_setter() {
        let field_type: syn::Type = syn::parse_quote!(i32);

        let config = resolve_setter_parameter_config(&field_type, None, false);

        assert_eq!(config.param_type.to_string(), "i32");
        assert_eq!(config.field_assignment_expr.to_string(), "value");
    }

    #[test]
    fn test_resolve_setter_parameter_config_complex_types() {
        // Test with complex generic types
        let field_type: syn::Type = syn::parse_quote!(HashMap<String, Vec<i32>>);

        // impl_into case
        let config = resolve_setter_parameter_config(&field_type, None, true);
        assert_eq!(
            config.param_type.to_string(),
            "impl Into < HashMap < String , Vec < i32 > > >"
        );

        // Regular case
        let config = resolve_setter_parameter_config(&field_type, None, false);
        assert_eq!(
            config.param_type.to_string(),
            "HashMap < String , Vec < i32 > >"
        );

        // Converter case
        let converter: syn::Expr =
            syn::parse_quote!(|data: Vec<(String, Vec<i32>)>| data.into_iter().collect());
        let config = resolve_setter_parameter_config(&field_type, Some(&converter), false);
        assert_eq!(
            config.param_type.to_string(),
            "Vec < (String , Vec < i32 >) >"
        );
        assert_eq!(
            config.field_assignment_expr.to_string(),
            "(| data : Vec < (String , Vec < i32 >) > | data . into_iter () . collect ()) (value)"
        );
    }

    #[test]
    fn test_resolve_setter_parameter_config_complex_converters() {
        let field_type: syn::Type = syn::parse_quote!(PathBuf);

        // Converter with method chaining
        let converter: syn::Expr =
            syn::parse_quote!(|path_str: &str| PathBuf::from(path_str.trim()));
        let config = resolve_setter_parameter_config(&field_type, Some(&converter), false);

        assert_eq!(config.param_type.to_string(), "& str");
        assert_eq!(
            config.field_assignment_expr.to_string(),
            "(| path_str : & str | PathBuf :: from (path_str . trim ())) (value)"
        );

        // Converter with more complex logic
        let converter: syn::Expr = syn::parse_quote!(|items: Vec<String>| items
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("/")
            .into());
        let config = resolve_setter_parameter_config(&field_type, Some(&converter), false);

        assert_eq!(config.param_type.to_string(), "Vec < String >");
        assert_eq!(config.field_assignment_expr.to_string(), "(| items : Vec < String > | items . into_iter () . filter (| s | ! s . is_empty ()) . collect :: < Vec < _ > > () . join (\"/\") . into ()) (value)");
    }

    #[test]
    fn test_resolve_setter_parameter_config_precedence() {
        let field_type: syn::Type = syn::parse_quote!(String);
        let converter: syn::Expr = syn::parse_quote!(|input: &str| input.to_string());

        // Converter should always take precedence over impl_into
        let configs = [
            resolve_setter_parameter_config(&field_type, Some(&converter), true),
            resolve_setter_parameter_config(&field_type, Some(&converter), false),
        ];

        for config in configs {
            assert_eq!(config.param_type.to_string(), "& str");
            assert_eq!(
                config.field_assignment_expr.to_string(),
                "(| input : & str | input . to_string ()) (value)"
            );
        }
    }
}
