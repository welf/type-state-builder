//! Struct Validation Logic
//!
//! This module provides centralized validation for struct-level configurations
//! and cross-field relationships, ensuring consistent validation behavior.

use crate::analysis::StructAnalysis;
use crate::validation::{ErrorMessages, FieldValidator, ValidationContext};
use std::collections::HashMap;

/// Validator for struct-level configurations and cross-field relationships.
///
/// This struct provides centralized validation logic for struct attributes
/// and relationships between fields that must be consistent.
pub struct StructValidator<'a> {
    /// Context for validation operations
    context: &'a mut ValidationContext,
}

impl<'a> StructValidator<'a> {
    /// Creates a new struct validator with the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - Mutable validation context for shared state and options
    ///
    /// # Returns
    ///
    /// A new `StructValidator` ready for validation operations.
    pub fn new(context: &'a mut ValidationContext) -> Self {
        Self { context }
    }

    /// Validates complete struct configuration for builder generation.
    ///
    /// This is the main entry point for struct validation that performs
    /// all necessary checks for struct-level and cross-field validation.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The complete struct analysis to validate
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    pub fn validate_struct_for_generation(&mut self, analysis: &StructAnalysis) -> syn::Result<()> {
        // Clear any previous setter name records
        self.context.clear_setter_names();

        // Validate that the struct has at least one field
        self.validate_struct_has_fields(analysis)?;

        // Validate all individual field configurations
        self.validate_all_field_configurations(analysis)?;

        // Validate cross-field relationships
        self.validate_field_relationships(analysis)?;

        // Validate struct-level attributes
        self.validate_struct_attributes(analysis)?;

        // Validate const builder requirements
        self.validate_const_builder_requirements(analysis)?;

        // Validate builder_method requirements
        self.validate_builder_method_requirements(analysis)?;

        Ok(())
    }

    /// Validates that the struct has at least one field.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis to check
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_struct_has_fields(&self, analysis: &StructAnalysis) -> syn::Result<()> {
        let total_fields = analysis.required_fields().len() + analysis.optional_fields().len();
        if total_fields == 0 {
            return Err(ErrorMessages::structured_error_span(
                proc_macro2::Span::call_site(),
                &format!("Struct '{}' has no fields", analysis.struct_name()),
                Some("builder generation requires at least one field to be meaningful"),
                Some("add some fields to your struct"),
            ));
        }
        Ok(())
    }

    /// Validates all individual field configurations.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis containing all fields
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_all_field_configurations(&self, analysis: &StructAnalysis) -> syn::Result<()> {
        // Validate required fields
        for field in analysis.required_fields() {
            FieldValidator::new(self.context).validate_field_configuration(field)?;
        }

        // Validate optional fields
        for field in analysis.optional_fields() {
            FieldValidator::new(self.context).validate_field_configuration(field)?;
        }

        Ok(())
    }

    /// Validates relationships and dependencies between fields.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis to validate
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_field_relationships(&mut self, analysis: &StructAnalysis) -> syn::Result<()> {
        // Validate setter name conflicts
        self.validate_setter_name_conflicts(analysis)?;

        // Validate build method name doesn't conflict with setters
        self.validate_build_method_name_conflict(analysis)?;

        Ok(())
    }

    /// Validates that setter method names don't conflict with each other.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis to check for conflicts
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_setter_name_conflicts(&mut self, analysis: &StructAnalysis) -> syn::Result<()> {
        let mut setter_names: HashMap<String, String> = HashMap::new();

        for field in analysis.all_fields() {
            if field.should_generate_setter() {
                let setter_name = field.setter_name();
                let field_name = field.clean_name();

                if let Some(existing_field) = setter_names.get(&setter_name) {
                    return Err(ErrorMessages::structured_error(
                        field.name(),
                        &format!("Setter name conflict: '{setter_name}' is used by both field '{field_name}' and field '{existing_field}'"),
                        Some("each setter method must have a unique name"),
                        Some("use #[builder(setter_name = \"unique_name\")] on one of the fields"),
                    ));
                }

                setter_names.insert(setter_name.clone(), field_name.to_string());
                // Also record in context for potential future use
                self.context
                    .record_setter_name(setter_name, field.name().to_string());
            }
        }

        Ok(())
    }

    /// Validates that the build method name doesn't conflict with setter names.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis to check for conflicts
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_build_method_name_conflict(&self, analysis: &StructAnalysis) -> syn::Result<()> {
        let build_method_name = analysis.struct_attributes().get_build_method_name();

        for field in analysis.all_fields() {
            if field.should_generate_setter() {
                let setter_name = field.setter_name();

                if setter_name == build_method_name {
                    return Err(ErrorMessages::structured_error_span(
                        proc_macro2::Span::call_site(),
                        &format!("Build method name '{build_method_name}' conflicts with setter method for field '{}'", field.clean_name()),
                        Some("the build method and setter methods must have unique names"),
                        Some("change the build method name with #[builder(build_method = \"create\")] or the setter name"),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Validates struct-level attributes and configuration.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis to validate
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_struct_attributes(&self, analysis: &StructAnalysis) -> syn::Result<()> {
        // The struct attributes are already validated during parsing,
        // but we can add additional semantic validation here if needed

        // Validate that build method name is reasonable
        let build_method_name = analysis.struct_attributes().get_build_method_name();
        if build_method_name.is_empty() {
            return Err(ErrorMessages::structured_error_span(
                proc_macro2::Span::call_site(),
                "build_method cannot be empty",
                Some("provide a non-empty value for the build_method attribute"),
                Some("example: #[builder(build_method = \"create\")]"),
            ));
        }

        Ok(())
    }

    /// Validates const builder requirements.
    ///
    /// When `#[builder(const)]` is enabled, this validates that:
    /// - All optional fields have explicit default values (Default::default() is not const)
    /// - No fields use `impl_into` (trait bounds not supported in const fn)
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis to validate
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    fn validate_const_builder_requirements(&self, analysis: &StructAnalysis) -> syn::Result<()> {
        if !analysis.struct_attributes().get_const_builder() {
            return Ok(());
        }

        // Check all fields for const-incompatible attributes
        for field in analysis.all_fields() {
            // Check for field-level impl_into
            if field.attributes().impl_into == Some(true) {
                let field_name = field.name();
                return Err(ErrorMessages::structured_error_span(
                    field_name.span(),
                    &format!(
                        "field `{}`: `impl_into` cannot be used with `#[builder(const)]`",
                        field_name
                    ),
                    Some("`impl Into<T>` requires trait bounds which are not supported in const fn"),
                    Some("remove the `impl_into` attribute from this field or remove `const` from the struct"),
                ));
            }
        }

        // Check that all optional fields have explicit defaults
        for field in analysis.optional_fields() {
            if !field.has_custom_default() {
                let field_name = field.name();
                return Err(ErrorMessages::structured_error_span(
                    field_name.span(),
                    &format!(
                        "const builder requires explicit default for field `{}`",
                        field_name
                    ),
                    Some("Default::default() cannot be used in const context"),
                    Some(&format!(
                        "add an explicit default: #[builder(default = <value>)] to field `{}`",
                        field_name
                    )),
                ));
            }
        }

        Ok(())
    }

    /// Validates builder_method attribute requirements.
    ///
    /// When `#[builder(builder_method)]` is used, this validates that:
    /// - Only one field has the attribute
    /// - The field is required (not optional)
    fn validate_builder_method_requirements(&self, analysis: &StructAnalysis) -> syn::Result<()> {
        let mut builder_method_fields: Vec<&crate::analysis::FieldInfo> = Vec::new();

        // Check required fields for builder_method
        for field in analysis.required_fields() {
            if field.attributes().builder_method {
                builder_method_fields.push(field);
            }
        }

        // Check optional fields - builder_method on optional field is an error
        for field in analysis.optional_fields() {
            if field.attributes().builder_method {
                let field_name = field.name();
                return Err(ErrorMessages::structured_error_span(
                    field_name.span(),
                    &format!(
                        "`builder_method` can only be used on required fields, but `{}` is optional",
                        field_name
                    ),
                    Some("optional fields cannot be builder entry points"),
                    Some("add `#[builder(required)]` to this field or remove `builder_method`"),
                ));
            }
        }

        // Check for multiple builder_method fields
        if builder_method_fields.len() > 1 {
            let field_names: Vec<_> = builder_method_fields
                .iter()
                .map(|f| f.name().to_string())
                .collect();
            return Err(ErrorMessages::structured_error_span(
                proc_macro2::Span::call_site(),
                &format!(
                    "only one field can have `builder_method`, but found on: {}",
                    field_names.join(", ")
                ),
                Some("the builder can only have one entry point"),
                Some("remove `builder_method` from all but one field"),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::analyze_struct;
    use syn::parse_quote;

    #[test]
    fn test_validate_simple_struct() {
        let input = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
                age: Option<u32>,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let mut context = ValidationContext::new();
        let mut validator = StructValidator::new(&mut context);
        assert!(validator.validate_struct_for_generation(&analysis).is_ok());
    }

    #[test]
    fn test_validate_empty_struct_fails() {
        let input = parse_quote! {
            struct Example {
                // No fields
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let mut context = ValidationContext::new();
        let mut validator = StructValidator::new(&mut context);
        let result = validator.validate_struct_for_generation(&analysis);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no fields"));
    }

    #[test]
    fn test_validate_setter_name_conflicts() {
        let input = parse_quote! {
            struct Example {
                #[builder(setter_name = "name")]
                first_name: String,
                #[builder(setter_name = "name")]
                last_name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let mut context = ValidationContext::new();
        let mut validator = StructValidator::new(&mut context);
        let result = validator.validate_struct_for_generation(&analysis);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("conflict"));
    }

    #[test]
    fn test_const_builder_requires_explicit_defaults() {
        // const builder without explicit default should fail
        let input = parse_quote! {
            #[builder(const)]
            struct Example {
                name: Option<String>,  // No explicit default
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let mut context = ValidationContext::new();
        let mut validator = StructValidator::new(&mut context);
        let result = validator.validate_struct_for_generation(&analysis);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("const builder requires explicit default"));
    }

    #[test]
    fn test_const_builder_with_explicit_defaults_passes() {
        // const builder with explicit defaults should pass
        let input = parse_quote! {
            #[builder(const)]
            struct Example {
                #[builder(default = None)]
                name: Option<String>,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let mut context = ValidationContext::new();
        let mut validator = StructValidator::new(&mut context);
        assert!(validator.validate_struct_for_generation(&analysis).is_ok());
    }

    #[test]
    fn test_const_builder_with_required_fields_passes() {
        // const builder with required fields only (no defaults needed) should pass
        let input = parse_quote! {
            #[builder(const)]
            struct Example {
                #[builder(required)]
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let mut context = ValidationContext::new();
        let mut validator = StructValidator::new(&mut context);
        assert!(validator.validate_struct_for_generation(&analysis).is_ok());
    }

    #[test]
    fn test_const_builder_with_field_impl_into_fails() {
        // const builder with field-level impl_into should fail
        let input = parse_quote! {
            #[builder(const)]
            struct Example {
                #[builder(required, impl_into)]
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let mut context = ValidationContext::new();
        let mut validator = StructValidator::new(&mut context);
        let result = validator.validate_struct_for_generation(&analysis);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("impl_into") && err.contains("const"));
    }
}
