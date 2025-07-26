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
