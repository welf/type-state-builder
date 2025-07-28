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

/// Utility struct for processing field-related operations.
///
/// This struct provides methods for common field processing operations
/// that are needed during builder generation. It encapsulates the logic
/// for handling field attributes, generating documentation, and creating
/// configuration objects.
///
/// # Design Philosophy
///
/// The `FieldProcessor` follows these principles:
/// - **Centralized logic** - All field processing in one place
/// - **Immutable operations** - Methods don't modify state
/// - **Comprehensive error handling** - Proper error propagation
/// - **Flexible configuration** - Support for various field configurations
///
#[derive(Debug, Clone)]
pub struct FieldProcessor;

impl FieldProcessor {
    /// Creates a new `FieldProcessor` instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for FieldProcessor {
    fn default() -> Self {
        Self::new()
    }
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
}
