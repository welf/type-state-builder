//! Centralized Error Message Management
//!
//! This module provides consistent, helpful error messages across the entire
//! codebase. All error message formatting is centralized here to ensure
//! consistency and maintainability.

/// Centralized error message generator for consistent formatting.
///
/// This struct provides methods for generating standardized error messages
/// with consistent formatting, helpful examples, and clear guidance for fixes.
pub struct ErrorMessages;

impl ErrorMessages {
    /// Creates an error message for required fields with default values.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The clean field name (without raw identifier prefix)
    /// * `field_type` - String representation of the field type
    ///
    /// # Returns
    ///
    /// A formatted error message with fix suggestions.
    pub fn required_field_with_default(field_name: &str, field_type: &str) -> String {
        format!(
            "Required field '{field_name}' cannot have a default value. \
            \n\nRequired fields must be explicitly set before build() can be called, \
            so default values are not meaningful. \
            \n\nTo fix this issue, either:\
            \n1. Remove #[builder(default = \"...\")] to keep the field required\
            \n2. Remove #[builder(required)] to make the field optional with a default\
            \n\nExample fixes:\
            \n  // Option 1: Keep as required field\
            \n  #[builder(required)]\
            \n  {field_name}: {field_type},\
            \n\n  // Option 2: Make optional with default\
            \n  #[builder(default = \"...\")]\
            \n  {field_name}: {field_type},"
        )
    }

    /// Creates an error message for required fields that skip setters.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The clean field name (without raw identifier prefix)
    /// * `field_type` - String representation of the field type
    ///
    /// # Returns
    ///
    /// A formatted error message with fix suggestions.
    pub fn required_field_skip_setter(field_name: &str, field_type: &str) -> String {
        format!(
            "Required field '{field_name}' cannot skip setter generation. \
            \n\nRequired fields need setter methods to be set before build() can be called. \
            \n\nTo fix this issue, either:\
            \n1. Remove #[builder(skip_setter)] to generate a setter for this required field\
            \n2. Remove #[builder(required)] and add a default value to make the field optional\
            \n\nExample fixes:\
            \n  // Option 1: Required field with setter\
            \n  #[builder(required)]\
            \n  {field_name}: {field_type},\
            \n\n  // Option 2: Optional field without setter\
            \n  #[builder(default = \"...\", skip_setter)]\
            \n  {field_name}: {field_type},"
        )
    }

    /// Creates an error message for fields that skip setters without defaults.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The clean field name (without raw identifier prefix)
    /// * `field_type` - String representation of the field type
    ///
    /// # Returns
    ///
    /// A formatted error message with fix suggestions.
    pub fn skip_setter_without_default(field_name: &str, field_type: &str) -> String {
        format!(
            "Field '{field_name}' skips setter generation but has no default value. \
            \n\nFields with #[builder(skip_setter)] must have a way to be initialized \
            since no setter method will be generated. \
            \n\nTo fix this issue, either:\
            \n1. Add a custom default value: #[builder(default = \"...\", skip_setter)]\
            \n2. Remove skip_setter to generate a normal setter method\
            \n3. Ensure the field type implements Default (the macro will use Default::default())\
            \n\nExample fixes:\
            \n  // Option 1: Custom default with no setter\
            \n  #[builder(default = \"Uuid::new_v4()\", skip_setter)]\
            \n  {field_name}: {field_type},\
            \n\n  // Option 2: Normal optional field with setter\
            \n  {field_name}: {field_type},\
            \n\n  // Option 3: Use Default::default() (if type implements Default)\
            \n  #[builder(skip_setter)]\
            \n  {field_name}: {field_type},"
        )
    }

    /// Creates an error message for setter name conflicts.
    ///
    /// # Arguments
    ///
    /// * `setter_name` - The conflicting setter name
    /// * `field1_name` - Name of the first field using this setter name
    /// * `field2_name` - Name of the second field using this setter name
    /// * `field_type` - String representation of the field type for examples
    ///
    /// # Returns
    ///
    /// A formatted error message with fix suggestions.
    pub fn setter_name_conflict(
        setter_name: &str,
        field1_name: &str,
        field2_name: &str,
        field_type: &str,
    ) -> String {
        format!(
            "Setter name conflict: '{setter_name}' is used by both field '{field1_name}' and field '{field2_name}'. \
            \n\nEach setter method must have a unique name. \
            \n\nTo fix this issue:\
            \n1. Use #[builder(setter_name = \"unique_name\")] on one of the fields\
            \n2. Rename one of the fields to avoid the conflict\
            \n\nExample fix:\
            \n  #[builder(setter_name = \"set_{field1_name}\")]\
            \n  {field1_name}: {field_type},"
        )
    }

    /// Creates an error message for build method name conflicts.
    ///
    /// # Arguments
    ///
    /// * `build_method_name` - The conflicting build method name
    /// * `field_name` - Name of the field that has a setter with the same name
    /// * `struct_name` - Name of the struct for examples
    /// * `field_type` - String representation of the field type for examples
    ///
    /// # Returns
    ///
    /// A formatted error message with fix suggestions.
    pub fn build_method_name_conflict(
        build_method_name: &str,
        field_name: &str,
        struct_name: &str,
        field_type: &str,
    ) -> String {
        format!(
            "Build method name '{build_method_name}' conflicts with setter method for field '{field_name}'. \
            \n\nThe build method and setter methods must have unique names. \
            \n\nTo fix this issue, either:\
            \n1. Change the build method name: #[builder(build_method = \"create\")]\
            \n2. Change the setter name: #[builder(setter_name = \"set_{field_name}\")]\
            \n\nExample fixes:\
            \n  // Option 1: Custom build method name\
            \n  #[builder(build_method = \"create\")]\
            \n  struct {struct_name} {{ ... }}\
            \n\n  // Option 2: Custom setter name\
            \n  #[builder(setter_name = \"set_{field_name}\")]\
            \n  {field_name}: {field_type},"
        )
    }

    /// Creates an error message for invalid setter names.
    ///
    /// # Arguments
    ///
    /// * `setter_name` - The invalid setter name
    ///
    /// # Returns
    ///
    /// A formatted error message with guidance.
    pub fn invalid_setter_name(setter_name: &str) -> String {
        format!(
            "Invalid setter name '{setter_name}'. Setter names must be valid Rust identifiers. \
            \n\nValid setter names:\
            \n  - Must start with a letter or underscore\
            \n  - Can contain letters, numbers, and underscores\
            \n  - Can use raw identifier syntax for keywords (r#type)\
            \n\nExamples:\
            \n  #[builder(setter_name = \"set_value\")]  // Valid\
            \n  #[builder(setter_name = \"r#type\")]     // Valid (raw identifier)\
            \n  #[builder(setter_name = \"field_1\")]    // Valid"
        )
    }

    /// Creates an error message for empty attribute values.
    ///
    /// # Arguments
    ///
    /// * `attribute_name` - The name of the attribute with empty value
    ///
    /// # Returns
    ///
    /// A formatted error message.
    pub fn empty_attribute_value(attribute_name: &str) -> String {
        format!(
            "{attribute_name} cannot be empty. \
            \n\nProvide a non-empty value for the {attribute_name} attribute.\
            \n\nExample:\
            \n  #[builder({attribute_name} = \"valid_value\")]"
        )
    }

    /// Creates an error message for structs with no fields.
    ///
    /// # Arguments
    ///
    /// * `struct_name` - The name of the empty struct
    ///
    /// # Returns
    ///
    /// A formatted error message with suggestions.
    pub fn empty_struct(struct_name: &str) -> String {
        format!(
            "Struct '{struct_name}' has no fields. \
            \n\nBuilder generation requires at least one field to be meaningful. \
            \n\nAdd some fields to your struct:\
            \n  struct {struct_name} {{\
            \n    // Add fields here\
            \n    field_name: FieldType,\
            \n  }}"
        )
    }

    /// Creates an error message for invalid default value expressions.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The field name with invalid default
    /// * `default_value` - The invalid default value expression
    /// * `parse_error` - The parse error details
    ///
    /// # Returns
    ///
    /// A formatted error message with examples.
    pub fn invalid_default_expression(
        field_name: &str,
        default_value: &str,
        parse_error: &dyn std::fmt::Display,
    ) -> String {
        format!(
            "Invalid default value expression for field '{field_name}': '{default_value}'. \
            \n\nParse error: {parse_error} \
            \n\nDefault values must be valid Rust expressions. \
            \n\nExample valid expressions:\
            \n  #[builder(default = \"42\")]           // Literal\
            \n  #[builder(default = \"String::new()\")] // Function call\
            \n  #[builder(default = \"vec![1, 2, 3]\")] // Macro call"
        )
    }

    /// Creates an error message for fields that skip setters but have setter names.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The clean field name (without raw identifier prefix)
    /// * `setter_name` - The custom setter name that conflicts with skip_setter
    /// * `field_type` - String representation of the field type
    ///
    /// # Returns
    ///
    /// A formatted error message with fix suggestions.
    pub fn skip_setter_with_setter_name(
        field_name: &str,
        setter_name: &str,
        field_type: &str,
    ) -> String {
        format!(
            "Field '{field_name}' has conflicting attributes: skip_setter and setter_name. \
            \n\nThe skip_setter attribute prevents setter method generation, \
            but the presence of setter_name indicates you still want to have a setter. \
            We can't resolve this ambiguity automatically, so you need to fix the issue. \
            \n\nEither:\
            \n1. Remove setter_name = \"{setter_name}\" to keep skip_setter\
            \n2. Remove skip_setter to generate a setter with the custom name\
            \n\nExample fixes:\
            \n  // Option 1: Field with no setter (auto-generated value)\
            \n  #[builder(default = \"...\", skip_setter)]\
            \n  {field_name}: {field_type},\
            \n\n  // Option 2: Field with custom setter name\
            \n  #[builder(setter_name = \"{setter_name}\")]\
            \n  {field_name}: {field_type},"
        )
    }

    /// Creates an error message for fields that skip setters but have setter prefixes.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The clean field name (without raw identifier prefix)
    /// * `setter_prefix` - The custom setter prefix that conflicts with skip_setter
    /// * `field_type` - String representation of the field type
    ///
    /// # Returns
    ///
    /// A formatted error message with fix suggestions.
    pub fn skip_setter_with_setter_prefix(
        field_name: &str,
        setter_prefix: &str,
        field_type: &str,
    ) -> String {
        format!(
            "Field '{field_name}' has conflicting attributes: skip_setter and setter_prefix. \
            \n\nThe skip_setter attribute prevents setter method generation, \
            but the presence of setter_prefix indicates you still want to have a setter. \
            We can't resolve this ambiguity automatically, so you need to fix the issue. \
            \n\nEither:\
            \n1. Remove setter_prefix = \"{setter_prefix}\" to keep skip_setter\
            \n2. Remove skip_setter to generate a setter with the custom prefix\
            \n\nExample fixes:\
            \n  // Option 1: Field with no setter (auto-generated value)\
            \n  #[builder(default = \"...\", skip_setter)]\
            \n  {field_name}: {field_type},\
            \n\n  // Option 2: Field with custom setter prefix\
            \n  #[builder(setter_prefix = \"{setter_prefix}\")]\
            \n  {field_name}: {field_type},"
        )
    }

    /// Formats a field type for display in error messages.
    ///
    /// # Arguments
    ///
    /// * `field_type` - The syn::Type to format
    ///
    /// # Returns
    ///
    /// A string representation of the type suitable for error messages.
    pub fn format_field_type(field_type: &syn::Type) -> String {
        quote::quote!(#field_type).to_string()
    }
}
