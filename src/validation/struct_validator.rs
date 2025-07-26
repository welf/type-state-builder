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
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                ErrorMessages::empty_struct(&analysis.struct_name().to_string()),
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
                    let field_type = ErrorMessages::format_field_type(field.field_type());
                    return Err(syn::Error::new_spanned(
                        field.name(),
                        ErrorMessages::setter_name_conflict(
                            &setter_name,
                            &field_name,
                            existing_field,
                            &field_type,
                        ),
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
                    let field_type = ErrorMessages::format_field_type(field.field_type());
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        ErrorMessages::build_method_name_conflict(
                            build_method_name,
                            &field.clean_name(),
                            &analysis.struct_name().to_string(),
                            &field_type,
                        ),
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
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                ErrorMessages::empty_attribute_value("build_method"),
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
}
