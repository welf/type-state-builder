//! Field Validation Logic
//!
//! This module provides centralized validation for field configurations,
//! eliminating duplication and ensuring consistent validation behavior.

use crate::analysis::FieldInfo;
use crate::utils::identifiers::strip_raw_identifier_prefix;
use crate::validation::{ErrorMessages, ValidationContext};
use syn::Ident;

/// Validator for field-level configurations and attributes.
///
/// This struct provides centralized validation logic for field attributes,
/// eliminating the duplication that previously existed between validation.rs
/// and field_utils.rs.
pub struct FieldValidator<'a> {
    /// Context for validation operations
    _context: &'a ValidationContext,
}

impl<'a> FieldValidator<'a> {
    /// Creates a new field validator with the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - Validation context for shared state and options
    ///
    /// # Returns
    ///
    /// A new `FieldValidator` ready for validation operations.
    pub fn new(context: &'a ValidationContext) -> Self {
        Self { _context: context }
    }

    /// Validates all aspects of a field's configuration.
    ///
    /// This is the main entry point for field validation that performs
    /// all necessary checks for a field's attributes and configuration.
    ///
    /// # Arguments
    ///
    /// * `field` - The field information to validate
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    ///
    /// # Validation Rules
    ///
    /// This method validates:
    /// 1. Required field attribute combinations
    /// 2. Skip setter attribute requirements
    /// 3. Custom setter name validity
    /// 4. Default value expression syntax
    pub fn validate_field_configuration(&self, field: &FieldInfo) -> syn::Result<()> {
        // Validate required field constraints
        if field.is_required() {
            self.validate_required_field_constraints(field)?;
        }

        // Validate skip setter constraints
        if !field.should_generate_setter() {
            self.validate_skip_setter_constraints(field)?;
        }

        // Validate custom setter name if provided
        if let Some(setter_name) = &field.attributes().setter_name {
            self.validate_setter_name(setter_name, field)?;
        }

        // Validate default value expression if provided
        if let Some(default_value) = &field.attributes().default_value {
            self.validate_default_expression(default_value, field)?;
        }

        Ok(())
    }

    /// Validates constraints specific to required fields.
    ///
    /// # Arguments
    ///
    /// * `field` - The required field to validate
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_required_field_constraints(&self, field: &FieldInfo) -> syn::Result<()> {
        let field_name_str = field.name().to_string();
        let clean_name = strip_raw_identifier_prefix(&field_name_str);
        let field_type = ErrorMessages::format_field_type(field.field_type());

        // Required fields cannot have default values
        if field.has_custom_default() {
            return Err(syn::Error::new_spanned(
                field.name(),
                ErrorMessages::required_field_with_default(&clean_name, &field_type),
            ));
        }

        // Required fields cannot skip setters
        if !field.should_generate_setter() {
            return Err(syn::Error::new_spanned(
                field.name(),
                ErrorMessages::required_field_skip_setter(&clean_name, &field_type),
            ));
        }

        Ok(())
    }

    /// Validates constraints for fields that skip setter generation.
    ///
    /// # Arguments
    ///
    /// * `field` - The field that skips setter generation
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_skip_setter_constraints(&self, field: &FieldInfo) -> syn::Result<()> {
        let field_name_str = field.name().to_string();
        let clean_name = strip_raw_identifier_prefix(&field_name_str);
        let field_type = ErrorMessages::format_field_type(field.field_type());

        // Fields that skip setters cannot have custom setter names
        if let Some(setter_name) = &field.attributes().setter_name {
            return Err(syn::Error::new_spanned(
                field.name(),
                ErrorMessages::skip_setter_with_setter_name(&clean_name, setter_name, &field_type),
            ));
        }

        // Fields that skip setters cannot have setter prefixes
        if let Some(setter_prefix) = &field.attributes().setter_prefix {
            return Err(syn::Error::new_spanned(
                field.name(),
                ErrorMessages::skip_setter_with_setter_prefix(
                    &clean_name,
                    setter_prefix,
                    &field_type,
                ),
            ));
        }

        // Fields that skip setters must have default values (custom or Default::default())
        if !field.has_custom_default() && field.is_optional() {
            return Err(syn::Error::new_spanned(
                field.name(),
                ErrorMessages::skip_setter_without_default(&clean_name, &field_type),
            ));
        }

        Ok(())
    }

    /// Validates that a custom setter name is a valid identifier.
    ///
    /// # Arguments
    ///
    /// * `setter_name` - The custom setter name to validate
    /// * `field` - The field that uses this setter name (for error context)
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_setter_name(&self, setter_name: &str, field: &FieldInfo) -> syn::Result<()> {
        // Try to parse as identifier to ensure it's valid
        syn::parse_str::<Ident>(setter_name).map_err(|_| {
            syn::Error::new_spanned(
                field.name(),
                ErrorMessages::invalid_setter_name(setter_name),
            )
        })?;

        Ok(())
    }

    /// Validates that a default value expression is syntactically correct.
    ///
    /// # Arguments
    ///
    /// * `default_value` - The default value expression to validate
    /// * `field` - The field that uses this default value (for error context)
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_default_expression(
        &self,
        default_value: &str,
        field: &FieldInfo,
    ) -> syn::Result<()> {
        // Try to parse the default value as an expression
        syn::parse_str::<syn::Expr>(default_value).map_err(|parse_error| {
            let field_name_str = field.name().to_string();
            let clean_name = strip_raw_identifier_prefix(&field_name_str);
            syn::Error::new_spanned(
                field.name(),
                ErrorMessages::invalid_default_expression(&clean_name, default_value, &parse_error),
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::FieldAttributes;
    use syn::parse_quote;

    fn create_test_field(name: &str, attributes: FieldAttributes) -> FieldInfo {
        FieldInfo::new_for_test(
            syn::parse_str(name).unwrap(),
            parse_quote!(String),
            attributes,
        )
    }

    #[test]
    fn test_validate_required_field_with_default_fails() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: true,
                default_value: Some("test".to_string()),
                setter_name: None,
                setter_prefix: None,
                skip_setter: false,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Required field"));
    }

    #[test]
    fn test_validate_required_field_skip_setter_fails() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: true,
                default_value: None,
                setter_name: None,
                setter_prefix: None,
                skip_setter: true,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot skip setter"));
    }

    #[test]
    fn test_validate_valid_required_field_passes() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: true,
                default_value: None,
                setter_name: None,
                setter_prefix: None,
                skip_setter: false,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_skip_setter_without_default_fails() {
        let context = ValidationContext::new();
        let _validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: false,
                default_value: None,
                setter_name: None,
                setter_prefix: None,
                skip_setter: true,
                impl_into: None,
            },
        );

        let result = field.validate_configuration();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have a default value"));
    }

    #[test]
    fn test_validate_invalid_setter_name_fails() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: false,
                default_value: None,
                setter_name: Some("123invalid".to_string()),
                setter_prefix: None,
                skip_setter: false,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid setter name"));
    }

    #[test]
    fn test_validate_skip_setter_with_setter_name_fails() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "count",
            FieldAttributes {
                required: false,
                default_value: Some("42".to_string()),
                setter_name: Some("custom_name".to_string()),
                setter_prefix: None,
                skip_setter: true,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("conflicting attributes: skip_setter and setter_name"));
    }

    #[test]
    fn test_validate_skip_setter_with_setter_prefix_fails() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "count",
            FieldAttributes {
                required: false,
                default_value: Some("42".to_string()),
                setter_name: None,
                setter_prefix: Some("with_".to_string()),
                skip_setter: true,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("conflicting attributes: skip_setter and setter_prefix"));
    }

    #[test]
    fn test_validate_skip_setter_with_default_passes() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "count",
            FieldAttributes {
                required: false,
                default_value: Some("42".to_string()),
                setter_name: None,
                setter_prefix: None,
                skip_setter: true,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_optional_field_with_custom_setter_name_passes() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: false,
                default_value: None,
                setter_name: Some("set_name".to_string()),
                setter_prefix: None,
                skip_setter: false,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_required_field_with_custom_setter_name_passes() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: true,
                default_value: None,
                setter_name: Some("set_name".to_string()),
                setter_prefix: None,
                skip_setter: false,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_field_with_setter_prefix_and_setter_name_passes() {
        let context = ValidationContext::new();
        let validator = FieldValidator::new(&context);

        let field = create_test_field(
            "name",
            FieldAttributes {
                required: false,
                default_value: None,
                setter_name: Some("custom_name".to_string()),
                setter_prefix: Some("with_".to_string()),
                skip_setter: false,
                impl_into: None,
            },
        );

        let result = validator.validate_field_configuration(&field);
        assert!(result.is_ok());
    }
}
