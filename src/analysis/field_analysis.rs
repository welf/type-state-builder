//! Field Analysis Implementation
//!
//! This module contains the `FieldInfo` type and its methods, which represent
//! complete information about individual struct fields for builder generation.
//! It includes field metadata, attribute parsing, and utility methods for
//! field processing.
//!
//! # Field Information Structure
//!
//! Each field is analyzed to extract:
//! - **Basic Information** - Name, type, and position
//! - **Attributes** - Builder-specific configuration
//! - **Classification** - Required vs optional status
//! - **Generation Context** - Information needed for code generation
//!

use crate::attributes::{parse_field_attributes, FieldAttributes};
use crate::utils::field_utils::{
    resolve_effective_impl_into, resolve_setter_parameter_config, DefaultConfig, SetterConfig,
};
use crate::utils::identifiers::strip_raw_identifier_prefix;
use crate::validation::error_messages::ErrorMessages;
use quote::quote;
use std::borrow::Cow;
use syn::{Ident, Type};

/// Complete information about a struct field for builder generation.
///
/// This struct encapsulates all the information needed to generate appropriate
/// builder code for a single field, including its metadata, attributes, and
/// processing configuration.
///
/// # Design Principles
///
/// The `FieldInfo` follows these principles:
/// - **Immutable after creation** - Field information is fixed once analyzed
/// - **Complete context** - Contains all information needed for generation
/// - **Validated attributes** - Attribute combinations are validated during creation
/// - **Processing methods** - Provides methods for common field operations
///
/// # Field Classification
///
/// Fields are classified as either:
/// - **Required** - Must be set before build() can be called
/// - **Optional** - Have default values and don't need to be explicitly set
///
/// This classification drives the type-state builder generation and determines
/// which builder pattern to use.
#[derive(Debug, Clone)]
pub struct FieldInfo {
    /// The field's identifier name
    name: Ident,

    /// The field's type
    ty: Type,

    /// Parsed builder attributes for this field
    attributes: FieldAttributes,
}

impl FieldInfo {
    /// Creates a new `FieldInfo` from syn field components.
    ///
    /// This is the primary constructor for `FieldInfo`, parsing attributes
    /// and validating field configuration during creation.
    ///
    /// # Arguments
    ///
    /// * `name` - The field's identifier
    /// * `ty` - The field's type
    /// * `attrs` - The field's attribute list for parsing
    ///
    /// # Returns
    ///
    /// A `Result<FieldInfo, syn::Error>` containing the complete field
    /// information or an error for invalid configurations.
    ///
    ///
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - Invalid attribute syntax or combinations
    /// - Conflicting field configurations
    /// - Missing required attribute values
    pub fn from_syn_field(name: Ident, ty: Type, attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let attributes = parse_field_attributes(attrs)?;

        let field_info = Self {
            name,
            ty,
            attributes,
        };

        // Validate the field configuration
        field_info.validate_configuration()?;

        Ok(field_info)
    }

    // Basic accessors

    /// Returns the field's name identifier.
    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// Returns the field's type.
    pub fn field_type(&self) -> &Type {
        &self.ty
    }

    /// Returns the field's parsed builder attributes.
    pub fn attributes(&self) -> &FieldAttributes {
        &self.attributes
    }

    // Classification methods

    /// Returns `true` if this field is required.
    pub fn is_required(&self) -> bool {
        self.attributes.required
    }

    /// Creates a new `FieldInfo` for testing purposes.
    ///
    /// This constructor is only available for testing and creates a FieldInfo
    /// with minimal configuration for use in unit tests.
    ///
    /// # Arguments
    ///
    /// * `name` - The field name
    /// * `ty` - The field type
    /// * `attributes` - The field attributes
    ///
    /// # Returns
    ///
    /// A new `FieldInfo` instance for testing.
    #[cfg(test)]
    pub fn new_for_test(
        name: syn::Ident,
        ty: syn::Type,
        attributes: crate::attributes::FieldAttributes,
    ) -> Self {
        Self {
            name,
            ty,
            attributes,
        }
    }

    /// Checks if this field is optional.
    ///
    /// Optional fields have default values and don't need to be explicitly
    /// set before calling build().
    ///
    /// # Returns
    ///
    /// `true` if the field is optional, `false` if it's required.
    ///
    pub fn is_optional(&self) -> bool {
        !self.is_required()
    }

    /// Checks if a setter method should be generated for this field.
    ///
    /// Fields with the `skip_setter` attribute don't get setter methods
    /// generated and are only initialized with their default values.
    ///
    /// # Returns
    ///
    /// `true` if a setter should be generated, `false` if it should be skipped.
    ///
    pub fn should_generate_setter(&self) -> bool {
        !self.attributes.skip_setter
    }

    /// Returns `true` if this field has a custom default value.
    pub fn has_custom_default(&self) -> bool {
        self.attributes.default_value.is_some()
    }

    // Name processing methods

    /// Returns the field name with raw identifier prefix removed (e.g., "type" instead of "r#type").
    pub fn clean_name(&self) -> String {
        strip_raw_identifier_prefix(&self.name.to_string()).into_owned()
    }

    /// Gets the name to use for the setter method without any prefix applied.
    ///
    /// This respects custom setter names specified in attributes, falling
    /// back to the field name (including raw identifiers) if no custom
    /// name is provided.
    ///
    /// # Returns
    ///
    /// The base setter method name as a string (without prefix).
    ///
    pub fn setter_name(&self) -> String {
        self.attributes
            .setter_name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| self.name.to_string())
    }

    /// Gets the final setter method name with prefixes applied.
    ///
    /// This method applies the setter prefix precedence rules:
    /// 1. Field-level setter_prefix takes highest priority
    /// 2. Struct-level setter_prefix takes second priority  
    /// 3. No prefix (original name) is the fallback
    ///
    /// # Arguments
    ///
    /// * `struct_setter_prefix` - Optional struct-level setter prefix from struct attributes
    ///
    /// # Returns
    ///
    /// The final setter method name as a string (with prefix applied if applicable).
    ///
    pub fn final_setter_name(&self, struct_setter_prefix: Option<&str>) -> String {
        // Get base setter name (field name or custom setter_name)
        let base_name = self.setter_name();

        // Apply setter prefix with proper priority
        if let Some(field_prefix) = &self.attributes().setter_prefix {
            // Field-level setter_prefix wins over everything
            format!("{field_prefix}{base_name}")
        } else if let Some(struct_prefix) = struct_setter_prefix {
            // Struct-level setter_prefix applies if no field-level prefix
            format!("{struct_prefix}{base_name}")
        } else {
            // No prefix - use the base name
            base_name
        }
    }

    // Processing and generation methods

    /// Creates a setter configuration for this field.
    ///
    /// This method uses the field processor to create a complete setter
    /// configuration that includes naming, documentation, and behavioral
    /// settings.
    ///
    /// # Arguments
    ///
    /// * `struct_setter_prefix` - Optional struct-level setter prefix from struct attributes
    ///
    /// # Returns
    ///
    /// A `SetterConfig` containing all information needed to generate
    /// a setter method.
    ///
    pub fn create_setter_config(&self, struct_setter_prefix: Option<&str>) -> SetterConfig {
        let field_name_str = self.name().to_string();
        let clean_name = strip_raw_identifier_prefix(&field_name_str);

        // Get the final setter name with prefixes applied
        let setter_name = Cow::Owned(self.final_setter_name(struct_setter_prefix));

        // Generate documentation comment
        let doc_comment = if self.attributes().required {
            format!("Sets the required field `{clean_name}`.")
        } else {
            format!("Sets the optional field `{clean_name}`.")
        };

        SetterConfig {
            setter_name,
            _field_name: Cow::Owned(field_name_str.clone()),
            _clean_name: clean_name.into_owned().into(),
            skip_setter: self.attributes().skip_setter,
            doc_comment,
        }
    }

    /// Creates a default value configuration for this field.
    ///
    /// This method analyzes the field's default value attributes and creates
    /// a configuration describing how the field should be initialized.
    ///
    /// # Returns
    ///
    /// A `Result<DefaultConfig, syn::Error>` containing default initialization
    /// information or an error for invalid default values.
    ///
    ///
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - Invalid default value expressions
    /// - Malformed default value syntax
    pub fn create_default_config(&self) -> DefaultConfig {
        let has_custom_default = self.attributes().default_value.is_some();

        // default_value is already a syn::Expr, just clone it
        let default_expression = self.attributes().default_value.clone();

        // If no custom default, we'll need to use Default::default()
        let requires_default_trait = !has_custom_default;

        DefaultConfig {
            _has_custom_default: has_custom_default,
            default_expression,
            _requires_default_trait: requires_default_trait,
        }
    }

    /// Generates field initialization code for builder constructors.
    ///
    /// Creates token streams for initializing this field in various contexts,
    /// such as builder constructors or state transitions.
    ///
    /// # Arguments
    ///
    /// * `is_required_unset` - Whether this is a required field in unset state
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the field
    /// initialization code or an error for invalid configurations.
    ///
    ///
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - Invalid default value expressions
    /// - Inconsistent field configurations
    pub fn generate_initialization(
        &self,
        is_required_unset: bool,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let field_name = self.name();

        if is_required_unset {
            // Required field in unset state - initialize as None
            Ok(quote! {
                #field_name: ::core::option::Option::None,
            })
        } else {
            // Optional field or required field in set state
            let default_config = self.create_default_config();

            if let Some(default_expr) = default_config.default_expression {
                // Use custom default value
                Ok(quote! {
                    #field_name: #default_expr,
                })
            } else {
                // Use Default::default() with helpful error message
                let field_name_str = self.name().to_string();
                let clean_name = strip_raw_identifier_prefix(&field_name_str);
                let field_type = self.field_type();

                let helpful_message = format!(
                    "Field `{}` does not have a custom default and its type may not implement `Default`. \
                    Solutions: \
                    1. Add `#[builder(default = \"...\")]` with a custom default value, \
                    2. Ensure the field type implements `Default`, or \
                    3. Add `#[derive(Default)]` to your custom types.",
                    &*clean_name
                );

                Ok(quote! {
                    #field_name: {
                        // This will generate a helpful error if Default is not implemented
                        #[allow(unused)]
                        const _HELP: &str = #helpful_message;
                        <#field_type as ::core::default::Default>::default()
                    },
                })
            }
        }
    }

    /// Generates a complete setter method for this field.
    ///
    /// Creates the full implementation of a setter method, including
    /// documentation, method signature, and body.
    ///
    /// # Arguments
    ///
    /// * `return_type` - The type that the setter method should return
    /// * `struct_setter_prefix` - Optional struct-level setter prefix from struct attributes
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the complete
    /// setter method or an error for invalid configurations.
    ///
    ///
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - Invalid setter names
    /// - Inconsistent field configurations
    /// - Generation failures
    pub fn generate_setter_method(
        &self,
        return_type: &Type,
        struct_setter_prefix: Option<&str>,
        struct_impl_into: bool,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let config = self.create_setter_config(struct_setter_prefix);

        if config.skip_setter {
            // No setter should be generated
            return Ok(quote! {});
        }

        let field_name = self.name();
        let field_type = self.field_type();

        // Determine parameter type and field assignment logic
        let field_impl_into = self.attributes().impl_into;
        let converter = self.attributes().converter.as_ref();
        let use_impl_into = resolve_effective_impl_into(field_impl_into, struct_impl_into);

        // Use the shared utility to determine parameter configuration
        let param_config = resolve_setter_parameter_config(field_type, converter, use_impl_into);

        // Handle setter method name - for raw identifiers, we use the raw identifier
        let setter_ident = if config.setter_name.starts_with("r#") {
            // Keep raw identifier for setter method
            syn::parse_str::<Ident>(&config.setter_name)?
        } else {
            syn::parse_str::<Ident>(&config.setter_name)?
        };

        let doc_comment = &config.doc_comment;
        let param_type = param_config.param_type;
        let field_assignment_expr = param_config.field_assignment_expr;

        // Generate assignment statement
        let assignment = quote! { self.#field_name = #field_assignment_expr; };

        // Generate method signature
        let method_signature = quote! {
            pub fn #setter_ident(mut self, value: #param_type) -> #return_type
        };

        Ok(quote! {
            #[doc = #doc_comment]
            #method_signature {
                #assignment
                self
            }
        })
    }

    // Validation methods

    /// Validates the field's configuration for consistency.
    ///
    /// This method performs comprehensive validation to ensure that the
    /// field's attributes are consistent and valid for builder generation.
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    ///
    ///
    ///
    /// # Validation Rules
    ///
    /// The method validates:
    /// - Required fields don't have default values
    /// - Required fields don't skip setters
    /// - Fields that skip setters have default values
    /// - Custom setter names are valid identifiers
    ///
    /// # Errors
    ///
    /// Returns specific errors for:
    /// - Conflicting attribute combinations
    /// - Invalid setter names
    /// - Missing required configurations
    pub fn validate_configuration(&self) -> syn::Result<()> {
        // Required fields cannot have default values
        if self.is_required() && self.attributes().default_value.is_some() {
            return Err(ErrorMessages::structured_error(
                self.name(),
                "Required fields cannot have default values",
                Some("#[builder(default)] and #[builder(required)] are incompatible"),
                Some("remove one of incompatible attributes"),
            ));
        }

        // Required fields cannot skip setters
        if self.is_required() && self.attributes().skip_setter {
            return Err(ErrorMessages::structured_error(
                self.name(),
                "Required fields cannot skip setters",
                Some("#[builder(skip_setter)] and #[builder(required)] are incompatible"),
                Some("remove one of incompatible attributes"),
            ));
        }

        // Fields that skip setters must have default values
        if self.attributes().skip_setter && self.attributes().default_value.is_none() {
            return Err(ErrorMessages::structured_error(
                self.name(),
                "Fields with #[builder(skip_setter)] must have a default value",
                Some("#[builder(skip_setter)] requires a way to initialize the field"),
                Some("add #[builder(default = \"...\")] or remove skip_setter"),
            ));
        }

        // Validate custom setter name if provided
        if let Some(setter_name) = &self.attributes().setter_name {
            // Try to parse as identifier to ensure it's valid
            syn::parse_str::<Ident>(setter_name).map_err(|_| {
                syn::Error::new_spanned(
                    self.name(),
                    format!("Invalid setter name '{setter_name}'. Setter names must be valid Rust identifiers.")
                )
            })?;
        }

        Ok(())
    }

    // Utility methods for analysis
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_from_syn_field_required() {
        let attrs = vec![parse_quote!(#[builder(required)])];
        let field =
            FieldInfo::from_syn_field(parse_quote!(name), parse_quote!(String), &attrs).unwrap();

        assert!(field.is_required());
        assert!(!field.is_optional());
        assert_eq!(field.name().to_string(), "name");
    }

    #[test]
    fn test_from_syn_field_optional() {
        let field =
            FieldInfo::from_syn_field(parse_quote!(age), parse_quote!(Option<u32>), &[]).unwrap();

        assert!(!field.is_required());
        assert!(field.is_optional());
    }

    #[test]
    fn test_from_syn_field_with_custom_default() {
        let attrs = vec![parse_quote!(#[builder(default = "42")])];
        let field =
            FieldInfo::from_syn_field(parse_quote!(count), parse_quote!(i32), &attrs).unwrap();

        assert!(field.has_custom_default());
        // default_value is now a syn::Expr, verify it exists
        assert!(field.attributes().default_value.is_some());
    }

    #[test]
    fn test_from_syn_field_skip_setter() {
        let attrs = vec![parse_quote!(#[builder(default = "Uuid::new_v4()", skip_setter)])];
        let field =
            FieldInfo::from_syn_field(parse_quote!(id), parse_quote!(Uuid), &attrs).unwrap();

        assert!(!field.should_generate_setter());
        assert!(field.has_custom_default());
    }

    #[test]
    fn test_clean_name() {
        // Regular field name
        let regular_field =
            FieldInfo::from_syn_field(parse_quote!(user_name), parse_quote!(String), &[]).unwrap();
        assert_eq!(regular_field.clean_name(), "user_name");

        // Raw identifier field name
        let raw_field =
            FieldInfo::from_syn_field(parse_quote!(r#type), parse_quote!(String), &[]).unwrap();
        assert_eq!(raw_field.clean_name(), "type");
    }

    #[test]
    fn test_setter_name() {
        // Default setter name
        let default_field =
            FieldInfo::from_syn_field(parse_quote!(user_name), parse_quote!(String), &[]).unwrap();
        assert_eq!(default_field.setter_name(), "user_name");

        // Custom setter name
        let custom_attrs = vec![parse_quote!(#[builder(setter_name = "set_name")])];
        let custom_field = FieldInfo::from_syn_field(
            parse_quote!(internal_name),
            parse_quote!(String),
            &custom_attrs,
        )
        .unwrap();
        assert_eq!(custom_field.setter_name(), "set_name");
    }

    #[test]
    fn test_create_setter_config() {
        let field = FieldInfo::from_syn_field(
            parse_quote!(name),
            parse_quote!(String),
            &[parse_quote!(#[builder(required)])],
        )
        .unwrap();

        let config = field.create_setter_config(None);
        assert_eq!(config.setter_name, "name");
        assert!(!config.skip_setter);
        assert!(config.doc_comment.contains("required"));
    }

    #[test]
    fn test_create_default_config() {
        let custom_attrs = vec![parse_quote!(#[builder(default = "42")])];
        let field =
            FieldInfo::from_syn_field(parse_quote!(count), parse_quote!(i32), &custom_attrs)
                .unwrap();

        let config = field.create_default_config();
        assert!(config._has_custom_default);
        assert!(config.default_expression.is_some());
    }

    #[test]
    fn test_validation_errors() {
        // Required field with default should fail validation
        let invalid_attrs = vec![
            parse_quote!(#[builder(required)]),
            parse_quote!(#[builder(default = "test")]),
        ];
        let result =
            FieldInfo::from_syn_field(parse_quote!(name), parse_quote!(String), &invalid_attrs);
        assert!(result.is_err());
    }

    #[test]
    fn test_final_setter_name_with_no_prefix() {
        let field =
            FieldInfo::from_syn_field(parse_quote!(user_name), parse_quote!(String), &[]).unwrap();

        assert_eq!(field.final_setter_name(None), "user_name");
        assert_eq!(field.final_setter_name(Some("with_")), "with_user_name");
    }

    #[test]
    fn test_final_setter_name_with_custom_setter_name() {
        let attrs = vec![parse_quote!(#[builder(setter_name = "set_name")])];
        let field =
            FieldInfo::from_syn_field(parse_quote!(internal_name), parse_quote!(String), &attrs)
                .unwrap();

        assert_eq!(field.final_setter_name(None), "set_name");
        assert_eq!(field.final_setter_name(Some("with_")), "with_set_name");
    }

    #[test]
    fn test_final_setter_name_with_field_level_prefix() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "set_")])];
        let field =
            FieldInfo::from_syn_field(parse_quote!(name), parse_quote!(String), &attrs).unwrap();

        // Field-level prefix wins over struct-level prefix
        assert_eq!(field.final_setter_name(None), "set_name");
        assert_eq!(field.final_setter_name(Some("with_")), "set_name");
    }

    #[test]
    fn test_final_setter_name_with_field_prefix_and_custom_name() {
        let attrs = vec![
            parse_quote!(#[builder(setter_name = "data")]),
            parse_quote!(#[builder(setter_prefix = "set_")]),
        ];
        let field =
            FieldInfo::from_syn_field(parse_quote!(internal_value), parse_quote!(String), &attrs)
                .unwrap();

        // Field-level prefix applies to custom setter name
        assert_eq!(field.final_setter_name(None), "set_data");
        assert_eq!(field.final_setter_name(Some("with_")), "set_data");
    }

    #[test]
    fn test_create_setter_config_with_prefixes() {
        // Test struct-level prefix
        let field =
            FieldInfo::from_syn_field(parse_quote!(name), parse_quote!(String), &[]).unwrap();

        let config = field.create_setter_config(Some("with_"));
        assert_eq!(config.setter_name, "with_name");

        // Test field-level prefix overrides struct-level
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "set_")])];
        let field_with_prefix =
            FieldInfo::from_syn_field(parse_quote!(name), parse_quote!(String), &attrs).unwrap();

        let config = field_with_prefix.create_setter_config(Some("with_"));
        assert_eq!(config.setter_name, "set_name");
    }
}
