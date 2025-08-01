//! Field Attribute Parsing
//!
//! This module handles parsing of field-level builder attributes, providing
//! a structured representation of attribute information and validation of
//! attribute combinations.
//!
//! # Supported Field Attributes
//!
//! - `required` - Marks a field as required in the builder pattern
//! - `setter_name = "name"` - Specifies a custom name for the setter method
//! - `setter_prefix = "prefix_"` - Specifies a custom prefix for the setter method
//! - `default = "expression"` - Provides a custom default value expression
//! - `skip_setter` - Prevents generation of a setter method for this field
//! - `impl_into` - Uses `impl Into<FieldType>` parameters for ergonomic setters
//! - `converter = |value: InputType| expression` - Custom conversion logic using closures
//!
//! # Attribute Validation
//!
//! The module validates attribute combinations:
//! - Required fields cannot have default values
//! - Required fields cannot skip setter generation
//! - Fields that skip setters must have default values
//! - `converter` is incompatible with `skip_setter` and `impl_into`
//! - `impl_into` is incompatible with `skip_setter`
//! - Setter prefixes are incompatible with `skip_setter`
//!
//! # Converter Attribute
//!
//! The `converter` attribute allows custom transformation logic for field values:
//!
//! ```rust,ignore
//! #[derive(TypeStateBuilder)]
//! struct MyStruct {
//!     #[builder(converter = |values: Vec<&str>| values.into_iter().map(|s| s.to_string()).collect())]
//!     tags: Vec<String>,
//! }
//! ```
//!
//! This generates a setter that accepts `Vec<&str>` and converts it to `Vec<String>`
//! using the provided closure expression.

use crate::validation::error_messages::ErrorMessages;

/// Configuration derived from field-level builder attributes.
///
/// This struct represents all the builder-specific configuration that can
/// be applied to individual struct fields through attributes.
///
/// # Field Descriptions
///
/// * `required` - Whether this field must be set before building
/// * `setter_name` - Custom name for the setter method (None = use field name)
/// * `default_value` - Custom default value expression as a string
/// * `skip_setter` - Whether to skip generating a setter method
///
/// # Validation Rules
///
/// The following combinations are invalid and will cause compilation errors:
/// - `required = true` and `default_value.is_some()` - Required fields can't have defaults
/// - `required = true` and `skip_setter = true` - Required fields need setters
/// - `skip_setter = true` and `default_value.is_none()` - Skipped setters need defaults
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAttributes {
    /// Whether this field is required to be set before calling build().
    ///
    /// Required fields:
    /// - Must have setter methods generated
    /// - Cannot have default values
    /// - Are tracked through the type state system
    /// - Prevent build() until set
    pub required: bool,

    /// Custom name for the setter method.
    ///
    /// If None, the setter method will use the field name (including raw identifiers).
    /// If Some, the provided name will be used for the setter method.
    ///
    pub setter_name: Option<String>,

    /// Custom prefix for the setter method name.
    ///
    /// If None, the setter method uses its natural name (field name or custom setter_name).
    /// If Some, the provided prefix is prepended to the setter method name.
    /// This field-level setting takes precedence over struct-level setter_prefix.
    ///
    /// # Interaction with skip_setter
    ///
    /// This attribute is mutually exclusive with `skip_setter`. Fields that skip
    /// setter generation cannot have setter prefixes.
    ///
    /// # Prefix Requirements
    ///
    /// The setter prefix must:
    /// - Be a valid start of a Rust identifier
    /// - Not create conflicts when combined with field names
    /// - Follow Rust naming conventions (typically lowercase with underscore)
    ///
    pub setter_prefix: Option<String>,

    /// Custom default value expression as a string.
    ///
    /// This string will be parsed as a Rust expression and used to initialize
    /// the field in the builder constructor. If None, Default::default() will
    /// be used for optional fields.
    ///
    /// # Expression Requirements
    ///
    /// The expression must:
    /// - Be valid Rust syntax that evaluates to the field's type
    /// - Be available in the scope where the builder is used
    /// - Not reference `self` or other instance variables
    ///
    pub default_value: Option<String>,

    /// Whether to skip generating a setter method for this field.
    ///
    /// When true:
    /// - No setter method is generated
    /// - The field must have a default value (either custom or Default::default())
    /// - The field is initialized in the constructor and cannot be changed
    ///
    /// This is useful for:
    /// - Auto-generated fields (IDs, timestamps)
    /// - Fields that should always use their default value
    /// - Internal fields not meant to be set by users
    ///
    pub skip_setter: bool,

    /// Whether the setter method should use `impl Into<FieldType>` parameters.
    ///
    /// This field-level setting controls setter parameter types and takes precedence
    /// over struct-level `impl_into` settings.
    ///
    /// # Values
    ///
    /// - `None` - Inherit from struct-level `impl_into` setting
    /// - `Some(true)` - Use `impl Into<FieldType>` parameters  
    /// - `Some(false)` - Use `FieldType` parameters directly
    ///
    /// # Interaction with skip_setter
    ///
    /// This attribute is mutually exclusive with `skip_setter`. Fields that skip
    /// setter generation cannot specify `impl_into` behavior.
    ///
    /// See the crate-level documentation for comprehensive usage examples.
    pub impl_into: Option<bool>,

    /// Custom converter closure expression for parameter transformation.
    ///
    /// When specified, the setter method will use this closure to transform
    /// the input parameter before assigning to the field. The closure must:
    /// - Have explicit parameter types (e.g., `|value: Vec<&str>| ...`)
    /// - Return a value that matches the field's type exactly
    /// - Be valid Rust syntax that compiles in the generated code
    ///
    /// # Interaction with other attributes
    ///
    /// This attribute is mutually exclusive with `impl_into` and `skip_setter`.
    /// It is compatible with `required`, `setter_name`, `setter_prefix`, and `default`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// #[derive(TypeStateBuilder)]
    /// struct MyStruct {
    ///     #[builder(converter = |value: Vec<&str>| value.into_iter().map(|s| s.to_string()).collect())]
    ///     tags: Vec<String>,
    /// }
    /// ```
    pub converter: Option<syn::Expr>,
}

impl Default for FieldAttributes {
    /// Creates default field attributes (optional field with standard behavior).
    fn default() -> Self {
        Self {
            required: false,
            setter_name: None,
            setter_prefix: None,
            default_value: None,
            skip_setter: false,
            impl_into: None,
            converter: None,
        }
    }
}

impl FieldAttributes {
    /// Validates that the field attributes are consistent and valid.
    ///
    /// This method checks that all field-level attributes have valid values
    /// and don't conflict with each other or with Rust language requirements.
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    ///
    /// # Validation Rules
    ///
    /// The method validates:
    /// - Setter prefix and skip_setter are mutually exclusive
    /// - Setter prefixes are not empty
    /// - Setter prefixes are valid identifier beginnings
    /// - Setter names are valid identifiers when provided
    /// - Setter function and skip_setter are mutually exclusive
    /// - Setter function and impl_into are mutually exclusive
    /// - No duplicate setter functions
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - setter_prefix combined with skip_setter
    /// - Empty setter prefixes
    /// - Invalid setter prefix format
    /// - Invalid setter names
    /// - setter function combined with skip_setter
    /// - setter function combined with impl_into
    /// - duplicate setter attributes
    pub fn validate(&self) -> syn::Result<()> {
        // Validate that setter_prefix and skip_setter are mutually exclusive
        if self.setter_prefix.is_some() && self.skip_setter {
            return Err(ErrorMessages::structured_error_span(
                proc_macro2::Span::call_site(),
                "Field-level setter_prefix is incompatible with skip_setter",
                Some("#[builder(setter_prefix)] and #[builder(skip_setter)] are incompatible"),
                Some("remove one of these attributes"),
            ));
        }

        // Validate that impl_into and skip_setter are mutually exclusive
        if self.impl_into.is_some() && self.skip_setter {
            return Err(ErrorMessages::structured_error_span(
                proc_macro2::Span::call_site(),
                "Field-level impl_into is incompatible with skip_setter",
                Some("#[builder(impl_into)] and #[builder(skip_setter)] are incompatible"),
                Some("remove one of these attributes"),
            ));
        }

        // Validate that converter and skip_setter are mutually exclusive
        if self.converter.is_some() && self.skip_setter {
            return Err(ErrorMessages::structured_error_span(
                proc_macro2::Span::call_site(),
                "Field-level converter is incompatible with skip_setter",
                Some("#[builder(converter)] and #[builder(skip_setter)] are incompatible"),
                Some("remove one of these attributes"),
            ));
        }

        // Validate that converter and impl_into are mutually exclusive
        if self.converter.is_some() && self.impl_into.is_some() {
            return Err(ErrorMessages::structured_error_span(
                proc_macro2::Span::call_site(),
                "Field-level converter is incompatible with impl_into",
                Some("#[builder(converter)] and #[builder(impl_into)] are incompatible"),
                Some("use either custom converter or impl_into, not both"),
            ));
        }

        // Validate setter prefix if provided
        if let Some(setter_prefix) = &self.setter_prefix {
            if setter_prefix.is_empty() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "Setter prefix cannot be empty",
                ));
            }

            // Check if the prefix would create valid identifiers when combined with field names
            // We'll validate this by checking if it starts correctly
            if setter_prefix.chars().next().is_some_and(|c| c.is_numeric()) {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Invalid setter prefix '{setter_prefix}'. Setter prefixes cannot start with a number. \
                        Use a valid identifier prefix like 'with_' or 'set_'."
                    ),
                ));
            }

            // Check for basic identifier validity of prefix
            // This is a heuristic check - we'll do full validation when combining with field names
            if !setter_prefix
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_')
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Invalid setter prefix '{setter_prefix}'. Setter prefixes must contain only alphanumeric characters and underscores."
                    ),
                ));
            }
        }

        // Validate setter name if provided
        if let Some(setter_name) = &self.setter_name {
            if setter_name.is_empty() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "Setter name cannot be empty",
                ));
            }

            // Try to validate as identifier to ensure it's valid
            if syn::parse_str::<syn::Ident>(setter_name).is_err() {
                // If it's not a valid identifier, check if it might be a raw identifier
                if !setter_name.starts_with("r#") {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!(
                            "Invalid setter name '{setter_name}'. Setter names must be valid Rust identifiers. \
                            Use raw identifier syntax (r#name) for keywords."
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Parses builder attributes from a field's attribute list.
///
/// This function processes all `#[builder(...)]` attributes on a field and
/// combines them into a single `FieldAttributes` configuration. It validates
/// that the attribute combinations are valid and returns appropriate errors
/// for invalid configurations.
///
/// # Arguments
///
/// * `attrs` - The list of attributes from a struct field
///
/// # Returns
///
/// A `Result<FieldAttributes, syn::Error>` containing the parsed configuration
/// or a descriptive error for invalid attributes.
///
///
/// # Errors
///
/// Returns errors for:
/// - Invalid attribute syntax
/// - Unknown attribute names
/// - Invalid attribute value types
/// - Missing values for attributes that require them
/// - Empty attribute values
///
/// # Implementation Details
///
/// The function:
/// 1. Iterates through all attributes looking for `#[builder(...)]`
/// 2. Parses each builder attribute using `syn::parse_nested_meta`
/// 3. Validates individual attribute values
/// 4. Combines multiple attributes into a single configuration
/// 5. Returns the complete configuration or the first error encountered
pub fn parse_field_attributes(attrs: &[syn::Attribute]) -> syn::Result<FieldAttributes> {
    let mut field_attributes = FieldAttributes::default();

    // Process each attribute in the list
    for attr in attrs {
        // Only process #[builder(...)] attributes
        if attr.path().is_ident("builder") {
            // Parse the nested meta inside the builder attribute
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("required") {
                    // #[builder(required)]
                    // Check for duplicate required attributes
                    if field_attributes.required {
                        return Err(meta.error("Duplicate required attribute. Only one required is allowed per field"));
                    }
                    field_attributes.required = true;
                    Ok(())
                } else if meta.path.is_ident("skip_setter") {
                    // #[builder(skip_setter)]
                    // Check for duplicate skip_setter attributes
                    if field_attributes.skip_setter {
                        return Err(meta.error("Duplicate skip_setter attribute. Only one skip_setter is allowed per field"));
                    }
                    field_attributes.skip_setter = true;
                    Ok(())
                } else if meta.path.is_ident("setter_name") {
                    // #[builder(setter_name = "name")]
                    let value = meta.value()?;
                    let lit_str: syn::LitStr = value.parse()?;
                    let setter_name = lit_str.value();

                    // Validate that the setter name is not empty
                    if setter_name.is_empty() {
                        return Err(meta.error("Setter name cannot be empty"));
                    }

                    // Check for duplicate setter_name attributes
                    if field_attributes.setter_name.is_some() {
                        return Err(meta.error("Duplicate setter_name attribute. Only one setter_name is allowed per field"));
                    }

                    field_attributes.setter_name = Some(setter_name);
                    Ok(())
                } else if meta.path.is_ident("setter_prefix") {
                    // #[builder(setter_prefix = "prefix_")]
                    let value = meta.value()?;
                    let lit_str: syn::LitStr = value.parse()?;
                    let setter_prefix = lit_str.value();

                    // Validate that the setter prefix is not empty
                    if setter_prefix.is_empty() {
                        return Err(meta.error("Setter prefix cannot be empty"));
                    }

                    // Check for duplicate setter_prefix attributes
                    if field_attributes.setter_prefix.is_some() {
                        return Err(meta.error("Duplicate setter_prefix attribute. Only one setter_prefix is allowed per field"));
                    }

                    field_attributes.setter_prefix = Some(setter_prefix);
                    Ok(())
                } else if meta.path.is_ident("default") {
                    // #[builder(default = "expression")]
                    let value = meta.value()?;
                    let lit_str: syn::LitStr = value.parse()?;
                    let default_value = lit_str.value();

                    // Validate that the default value is not empty
                    if default_value.is_empty() {
                        return Err(meta.error("Default value cannot be empty"));
                    }

                    // Check for duplicate default attributes
                    if field_attributes.default_value.is_some() {
                        return Err(meta.error("Duplicate default attribute. Only one default is allowed per field"));
                    }

                    field_attributes.default_value = Some(default_value);
                    Ok(())
                } else if meta.path.is_ident("impl_into") {
                    // #[builder(impl_into)] or #[builder(impl_into = true/false)]
                    // Check for duplicate impl_into attributes
                    if field_attributes.impl_into.is_some() {
                        return Err(meta.error("Duplicate impl_into attribute. Only one impl_into is allowed per field"));
                    }

                    // Check if there's a value (impl_into = true/false) or just the flag (impl_into)
                    if meta.input.peek(syn::Token![=]) {
                        // #[builder(impl_into = true/false)]
                        let value = meta.value()?;
                        let lit_bool: syn::LitBool = value.parse()?;
                        field_attributes.impl_into = Some(lit_bool.value);
                    } else {
                        // #[builder(impl_into)] - defaults to true
                        field_attributes.impl_into = Some(true);
                    }
                    Ok(())
                } else if meta.path.is_ident("converter") {
                    // #[builder(converter = |value: Type| expression)]
                    let value = meta.value()?;
                    let expr: syn::Expr = value.parse()?;

                    // Check for duplicate converter attributes
                    if field_attributes.converter.is_some() {
                        return Err(meta.error("Duplicate converter attribute. Only one converter is allowed per field"));
                    }

                    field_attributes.converter = Some(expr);
                    Ok(())
                } else {
                    // Unknown attribute
                    Err(meta.error(
                        "Unknown builder attribute. Supported attributes: required, setter_name, setter_prefix, default, skip_setter, impl_into, converter"
                    ))
                }
            })?;
        }
    }

    // Validate field attribute combinations
    field_attributes.validate()?;

    Ok(field_attributes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_default_field_attributes() {
        let attrs = FieldAttributes::default();
        assert!(!attrs.required);
        assert!(attrs.setter_name.is_none());
        assert!(attrs.setter_prefix.is_none());
        assert!(attrs.default_value.is_none());
        assert!(!attrs.skip_setter);
        assert!(attrs.impl_into.is_none());
    }

    #[test]
    fn test_parse_required_attribute() {
        let attrs = vec![parse_quote!(#[builder(required)])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_setter_name_attribute() {
        let attrs = vec![parse_quote!(#[builder(setter_name = "set_field")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("set_field".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_default_attribute() {
        let attrs = vec![parse_quote!(#[builder(default = "42")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert_eq!(field_attrs.default_value, Some("42".to_string()));
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_skip_setter_attribute() {
        let attrs = vec![parse_quote!(#[builder(skip_setter)])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert!(field_attrs.default_value.is_none());
        assert!(field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_combined_attributes() {
        let attrs = vec![parse_quote!(#[builder(required, setter_name = "set_name")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("set_name".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_multiple_separate_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(setter_name = "custom_name")]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom_name".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_complex_default_value() {
        let attrs = vec![parse_quote!(#[builder(default = "std::collections::HashMap::new()")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert_eq!(
            field_attrs.default_value,
            Some("std::collections::HashMap::new()".to_string())
        );
    }

    #[test]
    fn test_parse_no_builder_attributes() {
        let attrs = vec![parse_quote!(#[derive(Debug)])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        // Should return default attributes
        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_empty_setter_name_error() {
        let attrs = vec![parse_quote!(#[builder(setter_name = "")])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_parse_empty_default_value_error() {
        let attrs = vec![parse_quote!(#[builder(default = "")])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_parse_unknown_attribute_error() {
        let attrs = vec![parse_quote!(#[builder(unknown_attr)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown builder attribute"));
    }

    // Tests for setter_prefix functionality

    #[test]
    fn test_parse_setter_prefix_attribute() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "with_")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert_eq!(field_attrs.setter_prefix, Some("with_".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_combined_setter_prefix_attributes() {
        let attrs = vec![parse_quote!(#[builder(required, setter_prefix = "set_")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert_eq!(field_attrs.setter_prefix, Some("set_".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_setter_prefix_with_setter_name() {
        let attrs = vec![parse_quote!(#[builder(setter_name = "custom", setter_prefix = "with_")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom".to_string()));
        assert_eq!(field_attrs.setter_prefix, Some("with_".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_empty_setter_prefix_error() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "")])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Setter prefix cannot be empty"));
    }

    #[test]
    fn test_validate_setter_prefix_with_skip_setter_error() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "with_", skip_setter)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("setter_prefix is incompatible with skip_setter"));
    }

    #[test]
    fn test_validate_invalid_setter_prefix_starting_with_number() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "1invalid_")])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot start with a number"));
    }

    #[test]
    fn test_validate_invalid_setter_prefix_special_chars() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "with-")])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("alphanumeric characters and underscores"));
    }

    #[test]
    fn test_validate_valid_setter_prefixes() {
        let valid_prefixes = [
            "with_", "set_", "use_", "add_", "remove_", "update_", "get_",
        ];

        for prefix in valid_prefixes {
            let attrs = vec![parse_quote!(#[builder(setter_prefix = #prefix)])];
            let result = parse_field_attributes(&attrs);
            assert!(result.is_ok(), "Prefix '{prefix}' should be valid");
        }
    }

    #[test]
    fn test_parse_multiple_setter_prefix_attributes() {
        // Multiple #[builder(...)] attributes with different settings
        let attrs = vec![
            parse_quote!(#[builder(setter_prefix = "with_")]),
            parse_quote!(#[builder(required)]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert_eq!(field_attrs.setter_prefix, Some("with_".to_string()));
        assert!(field_attrs.required);
    }

    #[test]
    fn test_field_attributes_validate_method() {
        // Test valid field attributes
        let valid_attrs = FieldAttributes {
            required: true,
            setter_name: Some("custom_name".to_string()),
            setter_prefix: Some("with_".to_string()),
            default_value: None,
            skip_setter: false,
            impl_into: None,
            converter: None,
        };
        assert!(valid_attrs.validate().is_ok());

        // Test invalid combination: setter_prefix with skip_setter
        let invalid_attrs = FieldAttributes {
            required: false,
            setter_name: None,
            setter_prefix: Some("with_".to_string()),
            default_value: Some("42".to_string()),
            skip_setter: true,
            impl_into: None,
            converter: None,
        };
        let result = invalid_attrs.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("incompatible with skip_setter"));
    }

    #[test]
    fn test_field_attributes_default_includes_setter_prefix() {
        let default_attrs = FieldAttributes::default();
        assert!(!default_attrs.required);
        assert!(default_attrs.setter_name.is_none());
        assert!(default_attrs.setter_prefix.is_none());
        assert!(default_attrs.default_value.is_none());
        assert!(!default_attrs.skip_setter);
    }

    #[test]
    fn test_parse_duplicate_setter_name_error() {
        let attrs = vec![
            parse_quote!(#[builder(setter_name = "first_name")]),
            parse_quote!(#[builder(setter_name = "full_name")]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate setter_name attribute"));
    }

    #[test]
    fn test_parse_duplicate_required_error() {
        let attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(required)]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate required attribute"));
    }

    #[test]
    fn test_parse_duplicate_skip_setter_error() {
        let attrs = vec![
            parse_quote!(#[builder(skip_setter)]),
            parse_quote!(#[builder(skip_setter)]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate skip_setter attribute"));
    }

    #[test]
    fn test_parse_duplicate_default_error() {
        let attrs = vec![
            parse_quote!(#[builder(default = "first")]),
            parse_quote!(#[builder(default = "second")]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate default attribute"));
    }

    #[test]
    fn test_parse_duplicate_setter_prefix_error() {
        let attrs = vec![
            parse_quote!(#[builder(setter_prefix = "with_")]),
            parse_quote!(#[builder(setter_prefix = "set_")]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate setter_prefix attribute"));
    }

    #[test]
    fn test_parse_mixed_attributes_in_single_builder_attribute() {
        let attrs =
            vec![parse_quote!(#[builder(required, setter_name = "custom", default = "42")])];
        let result = parse_field_attributes(&attrs);

        // Parsing should succeed - validation happens later in FieldValidator
        assert!(result.is_ok());
        let field_attrs = result.unwrap();
        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom".to_string()));
        assert_eq!(field_attrs.default_value, Some("42".to_string()));
    }

    #[test]
    fn test_parse_mixed_attributes_across_multiple_builder_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(setter_name = "custom")]),
            parse_quote!(#[builder(default = "42")]),
        ];
        let result = parse_field_attributes(&attrs);

        // Parsing should succeed - validation happens later in FieldValidator
        assert!(result.is_ok());
        let field_attrs = result.unwrap();
        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom".to_string()));
        assert_eq!(field_attrs.default_value, Some("42".to_string()));
    }

    #[test]
    fn test_parse_skip_setter_with_setter_name_only() {
        let attrs = vec![
            parse_quote!(#[builder(skip_setter)]),
            parse_quote!(#[builder(setter_name = "custom")]),
            parse_quote!(#[builder(default = "42")]),
        ];
        let result = parse_field_attributes(&attrs);

        // Parsing should succeed - validation happens later in FieldValidator
        assert!(result.is_ok());
        let field_attrs = result.unwrap();
        assert!(field_attrs.skip_setter);
        assert_eq!(field_attrs.setter_name, Some("custom".to_string()));
        assert_eq!(field_attrs.default_value, Some("42".to_string()));
        assert!(field_attrs.setter_prefix.is_none());
    }

    #[test]
    fn test_parse_complex_valid_combination() {
        let attrs = vec![
            parse_quote!(#[builder(setter_name = "custom_name")]),
            parse_quote!(#[builder(setter_prefix = "with_")]),
            parse_quote!(#[builder(default = "42")]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_ok());
        let field_attrs = result.unwrap();
        assert!(!field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom_name".to_string()));
        assert_eq!(field_attrs.setter_prefix, Some("with_".to_string()));
        assert_eq!(field_attrs.default_value, Some("42".to_string()));
        assert!(!field_attrs.skip_setter);
    }

    #[test]
    fn test_parse_duplicate_mixed_with_valid_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(setter_name = "first")]),
            parse_quote!(#[builder(setter_name = "second")]), // Duplicate
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate setter_name attribute"));
    }

    // Tests for impl_into functionality

    #[test]
    fn test_parse_impl_into_flag_attribute() {
        let attrs = vec![parse_quote!(#[builder(impl_into)])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
        assert_eq!(field_attrs.impl_into, Some(true));
    }

    #[test]
    fn test_parse_impl_into_true_attribute() {
        let attrs = vec![parse_quote!(#[builder(impl_into = true)])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
        assert_eq!(field_attrs.impl_into, Some(true));
    }

    #[test]
    fn test_parse_impl_into_false_attribute() {
        let attrs = vec![parse_quote!(#[builder(impl_into = false)])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
        assert_eq!(field_attrs.impl_into, Some(false));
    }

    #[test]
    fn test_parse_combined_attributes_with_impl_into() {
        let attrs = vec![parse_quote!(#[builder(required, impl_into, setter_name = "set_field")])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("set_field".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
        assert_eq!(field_attrs.impl_into, Some(true));
    }

    #[test]
    fn test_parse_multiple_separate_attributes_with_impl_into() {
        let attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(impl_into = false)]),
            parse_quote!(#[builder(setter_name = "custom_name")]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom_name".to_string()));
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
        assert_eq!(field_attrs.impl_into, Some(false));
    }

    #[test]
    fn test_parse_duplicate_impl_into_error() {
        let attrs = vec![
            parse_quote!(#[builder(impl_into)]),
            parse_quote!(#[builder(impl_into = false)]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate impl_into attribute"));
    }

    #[test]
    fn test_validate_impl_into_with_skip_setter_error() {
        let attrs = vec![parse_quote!(#[builder(impl_into, skip_setter)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("impl_into is incompatible with skip_setter"));
    }

    #[test]
    fn test_validate_impl_into_false_with_skip_setter_error() {
        let attrs = vec![parse_quote!(#[builder(impl_into = false, skip_setter)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("impl_into is incompatible with skip_setter"));
    }

    #[test]
    fn test_validate_valid_impl_into_combinations() {
        // impl_into with required
        let attrs = vec![parse_quote!(#[builder(required, impl_into)])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // impl_into with setter_name
        let attrs = vec![parse_quote!(#[builder(impl_into, setter_name = "custom")])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // impl_into with setter_prefix
        let attrs = vec![parse_quote!(#[builder(impl_into, setter_prefix = "with_")])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // impl_into with default
        let attrs = vec![parse_quote!(#[builder(impl_into, default = "42")])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // impl_into = false is also valid
        let attrs = vec![parse_quote!(#[builder(impl_into = false)])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_field_attributes_default_includes_impl_into() {
        let default_attrs = FieldAttributes::default();
        assert!(!default_attrs.required);
        assert!(default_attrs.setter_name.is_none());
        assert!(default_attrs.setter_prefix.is_none());
        assert!(default_attrs.default_value.is_none());
        assert!(!default_attrs.skip_setter);
        assert!(default_attrs.impl_into.is_none());
        assert!(default_attrs.converter.is_none());
    }

    // Comprehensive tests for setter function functionality

    #[test]
    fn test_parse_converter_simple() {
        let attrs =
            vec![parse_quote!(#[builder(converter = |value: String| value.to_uppercase())])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(!field_attrs.required);
        assert!(field_attrs.setter_name.is_none());
        assert!(field_attrs.setter_prefix.is_none());
        assert!(field_attrs.default_value.is_none());
        assert!(!field_attrs.skip_setter);
        assert!(field_attrs.impl_into.is_none());

        assert!(field_attrs.converter.is_some());
    }

    #[test]
    fn test_parse_converter_basic_closure() {
        let attrs =
            vec![parse_quote!(#[builder(converter = |value: String| value.to_uppercase())])];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.converter.is_some());
    }

    #[test]
    fn test_parse_converter_complex_closure() {
        let attrs = vec![
            parse_quote!(#[builder(converter = |values: Vec<String>| values.into_iter().collect())]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.converter.is_some());
    }

    #[test]
    fn test_parse_converter_with_other_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(required, converter = |value: String| value, setter_name = "custom")]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom".to_string()));
        assert!(field_attrs.converter.is_some());
    }

    #[test]
    fn test_parse_converter_with_setter_prefix() {
        let attrs = vec![
            parse_quote!(#[builder(converter = |value: String| value, setter_prefix = "with_")]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert_eq!(field_attrs.setter_prefix, Some("with_".to_string()));
        assert!(field_attrs.converter.is_some());
    }

    #[test]
    fn test_parse_converter_with_default() {
        let attrs = vec![
            parse_quote!(#[builder(converter = |value: String| value, default = "Vec::new()")]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert_eq!(field_attrs.default_value, Some("Vec::new()".to_string()));
        assert!(field_attrs.converter.is_some());
    }

    #[test]
    fn test_parse_converter_across_multiple_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(converter = |value: String| value)]),
            parse_quote!(#[builder(setter_name = "custom")]),
        ];
        let field_attrs = parse_field_attributes(&attrs).unwrap();

        assert!(field_attrs.required);
        assert_eq!(field_attrs.setter_name, Some("custom".to_string()));
        assert!(field_attrs.converter.is_some());
    }

    // Duplicate converter tests

    #[test]
    fn test_parse_duplicate_converter_same_attribute() {
        let attrs = vec![parse_quote!(#[builder(converter = |x: i32| x, converter = |y: i32| y)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate converter attribute"));
    }

    #[test]
    fn test_parse_duplicate_converter_different_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(converter = |x: i32| x)]),
            parse_quote!(#[builder(converter = |y: i32| y)]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate converter attribute"));
    }

    #[test]
    fn test_parse_duplicate_converter_mixed_with_valid() {
        let attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(converter = |x: i32| x)]),
            parse_quote!(#[builder(converter = |y: i32| y)]), // Duplicate
            parse_quote!(#[builder(setter_name = "custom")]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate converter attribute"));
    }

    // Validation tests for incompatible combinations

    #[test]
    fn test_validate_converter_with_skip_setter_error() {
        let attrs = vec![parse_quote!(#[builder(converter = |x: i32| x, skip_setter)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with skip_setter"));
    }

    #[test]
    fn test_validate_converter_with_impl_into_flag_error() {
        let attrs = vec![parse_quote!(#[builder(converter = |x: i32| x, impl_into)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with impl_into"));
    }

    #[test]
    fn test_validate_converter_with_impl_into_true_error() {
        let attrs = vec![parse_quote!(#[builder(converter = |x: i32| x, impl_into = true)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with impl_into"));
    }

    #[test]
    fn test_validate_converter_with_impl_into_false_error() {
        let attrs = vec![parse_quote!(#[builder(converter = |x: i32| x, impl_into = false)])];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with impl_into"));
    }

    #[test]
    fn test_validate_converter_with_skip_setter_across_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(converter = |x: i32| x)]),
            parse_quote!(#[builder(skip_setter)]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with skip_setter"));
    }

    #[test]
    fn test_validate_converter_with_impl_into_across_attributes() {
        let attrs = vec![
            parse_quote!(#[builder(converter = |x: i32| x)]),
            parse_quote!(#[builder(impl_into)]),
        ];
        let result = parse_field_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with impl_into"));
    }

    // Valid combinations tests

    #[test]
    fn test_validate_converter_with_valid_combinations() {
        // converter with required
        let attrs = vec![parse_quote!(#[builder(required, converter = |x: i32| x)])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // converter with setter_name
        let attrs = vec![parse_quote!(#[builder(converter = |x: i32| x, setter_name = "custom")])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // converter with setter_prefix
        let attrs = vec![parse_quote!(#[builder(converter = |x: i32| x, setter_prefix = "with_")])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // converter with default
        let attrs =
            vec![parse_quote!(#[builder(converter = |x: Vec<i32>| x, default = "Vec::new()")])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // converter with multiple valid attributes
        let attrs = vec![
            parse_quote!(#[builder(required, converter = |x: i32| x, setter_name = "custom", setter_prefix = "with_", default = "42")]),
        ];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok()); // Note: parsing should succeed, validation of required+default happens elsewhere
    }

    // Edge cases and complex path tests

    #[test]
    fn test_parse_converter_complex_closures() {
        // Complex closure with multiple parameters
        let attrs = vec![
            parse_quote!(#[builder(converter = |values: Vec<String>| values.into_iter().map(|s| s.to_uppercase()).collect())]),
        ];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        let field_attrs = result.unwrap();
        assert!(field_attrs.converter.is_some());

        // Closure with method calls and chaining
        let attrs =
            vec![parse_quote!(#[builder(converter = |input: String| input.trim().to_string())])];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());

        // Closure with complex logic
        let attrs = vec![
            parse_quote!(#[builder(converter = |data: Vec<i32>| data.into_iter().filter(|&x| x > 0).sum::<i32>())]),
        ];
        let result = parse_field_attributes(&attrs);
        assert!(result.is_ok());
    }

    // Manual validation tests using FieldAttributes directly

    #[test]
    fn test_field_attributes_validate_setter_function_combinations() {
        // Valid: setter function alone
        let valid_attrs = FieldAttributes {
            required: false,
            setter_name: None,
            setter_prefix: None,
            default_value: None,
            skip_setter: false,
            impl_into: None,
            converter: Some(syn::parse_str("|value: String| value").unwrap()),
        };
        assert!(valid_attrs.validate().is_ok());

        // Valid: setter function with other compatible attributes
        let valid_attrs = FieldAttributes {
            required: true,
            setter_name: Some("custom".to_string()),
            setter_prefix: Some("with_".to_string()),
            default_value: Some("Vec::new()".to_string()),
            skip_setter: false,
            impl_into: None,
            converter: Some(syn::parse_str("|value: String| value").unwrap()),
        };
        assert!(valid_attrs.validate().is_ok());

        // Invalid: converter with skip_setter
        let invalid_attrs = FieldAttributes {
            required: false,
            setter_name: None,
            setter_prefix: None,
            default_value: None,
            skip_setter: true,
            impl_into: None,
            converter: Some(syn::parse_str("|value: String| value").unwrap()),
        };
        let result = invalid_attrs.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with skip_setter"));

        // Invalid: converter with impl_into
        let invalid_attrs = FieldAttributes {
            required: false,
            setter_name: None,
            setter_prefix: None,
            default_value: None,
            skip_setter: false,
            impl_into: Some(true),
            converter: Some(syn::parse_str("|value: String| value").unwrap()),
        };
        let result = invalid_attrs.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Field-level converter is incompatible with impl_into"));
    }

    #[test]
    fn test_field_attributes_default_includes_setter_function() {
        let default_attrs = FieldAttributes::default();
        assert!(!default_attrs.required);
        assert!(default_attrs.setter_name.is_none());
        assert!(default_attrs.setter_prefix.is_none());
        assert!(default_attrs.default_value.is_none());
        assert!(!default_attrs.skip_setter);
        assert!(default_attrs.impl_into.is_none());
        assert!(default_attrs.converter.is_none());
    }
}
