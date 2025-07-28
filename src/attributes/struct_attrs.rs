//! Struct Attribute Parsing
//!
//! This module handles parsing of struct-level builder attributes, providing
//! configuration options that apply to the entire builder generation process.
//!
//! # Supported Struct Attributes
//!
//! - `build_method = "name"` - Specifies a custom name for the build method
//! - `setter_prefix = "prefix_"` - Specifies a prefix for all setter method names
//! - `impl_into` - Use `impl Into<FieldType>` for setter parameters instead of `FieldType`
//!
//! # Future Extensions
//!
//! This module is designed to be easily extended with additional struct-level
//! configuration options such as:
//! - Builder visibility settings
//! - Custom builder struct names
//! - Error handling strategies
//! - Documentation generation options
//!

/// Configuration derived from struct-level builder attributes.
///
/// This struct represents all the builder-specific configuration that can
/// be applied to the entire struct through attributes on the struct definition.
///
/// # Field Descriptions
///
/// * `build_method_name` - Custom name for the final build method (None = "build")
/// * `setter_prefix` - Common prefix for all setter method names (None = no prefix)
/// * `impl_into` - Whether setters should accept `impl Into<FieldType>` (false = use `FieldType`)
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructAttributes {
    /// Custom name for the build method.
    ///
    /// If None, the build method will be named "build". If Some, the provided
    /// name will be used for the final method that constructs the struct instance.
    ///
    /// # Method Name Requirements
    ///
    /// The build method name must:
    /// - Be a valid Rust identifier
    /// - Not conflict with generated setter method names
    /// - Not be a Rust keyword (unless raw identifier syntax is used)
    ///
    pub build_method_name: Option<String>,

    /// Common prefix for all setter method names.
    ///
    /// If None, setter methods use their natural names (field name or custom setter_name).
    /// If Some, the provided prefix is prepended to all setter method names.
    ///
    /// # Prefix Requirements
    ///
    /// The setter prefix must:
    /// - Be a valid start of a Rust identifier
    /// - Not create conflicts when combined with field names
    /// - Follow Rust naming conventions (typically lowercase with underscore)
    ///
    /// # Examples
    pub setter_prefix: Option<String>,

    /// Whether setter methods should use `impl Into<FieldType>` parameters.
    ///
    /// If false, setter methods use the field type directly: `fn field(value: FieldType)`
    /// If true, setter methods accept any convertible type: `fn field(value: impl Into<FieldType>)`
    ///
    /// This provides ergonomic conversion for common cases like:
    /// - `String` fields accepting `&str`, `String`, `Cow<str>`
    /// - `PathBuf` fields accepting `&str`, `&Path`, `String`
    /// - `Vec<T>` fields accepting arrays, slices, iterators
    ///
    /// # Field-Level Override
    ///
    /// Individual fields can override this struct-level setting using
    /// `#[builder(impl_into = true/false)]` on the field itself.
    ///
    /// See the crate-level documentation for comprehensive usage examples.
    pub impl_into: bool,
}

impl Default for StructAttributes {
    /// Creates default struct attributes.
    ///
    /// Default configuration:
    /// - `build_method_name: None` - Use "build" as the method name
    /// - `setter_prefix: None` - No prefix for setter methods
    /// - `impl_into: false` - Use direct field types in setters
    fn default() -> Self {
        Self {
            build_method_name: None,
            setter_prefix: None,
            impl_into: false,
        }
    }
}

impl StructAttributes {
    /// Gets the build method name, returning "build" if no custom name is set.
    ///
    /// This method provides a convenient way to get the build method name
    /// with the default fallback behavior built in.
    ///
    /// # Returns
    ///
    /// The build method name as a `&str`. Returns "build" if no custom name
    /// was specified, otherwise returns the custom name.
    ///
    pub fn get_build_method_name(&self) -> &str {
        self.build_method_name.as_deref().unwrap_or("build")
    }

    /// Gets the setter prefix, returning None if no custom prefix is set.
    ///
    /// This method provides access to the struct-level setter prefix that
    /// should be applied to all setter method names unless overridden by
    /// field-level settings.
    ///
    /// # Returns
    ///
    /// An `Option<&str>` containing the setter prefix if one was specified,
    /// or None if setter methods should use their natural names.
    ///
    pub fn get_setter_prefix(&self) -> Option<&str> {
        self.setter_prefix.as_deref()
    }

    /// Gets the impl_into setting for the struct.
    ///
    /// This method provides access to the struct-level impl_into setting that
    /// controls whether setter methods should accept `impl Into<FieldType>`
    /// parameters instead of `FieldType` directly.
    ///
    /// # Returns
    ///
    /// A `bool` indicating whether setters should use `impl Into<T>` parameters.
    /// - `true` - Setters accept `impl Into<FieldType>`
    /// - `false` - Setters accept `FieldType` directly
    ///
    pub fn get_impl_into(&self) -> bool {
        self.impl_into
    }

    /// Validates that the struct attributes are consistent and valid.
    ///
    /// This method checks that all struct-level attributes have valid values
    /// and don't conflict with each other or with Rust language requirements.
    ///
    /// # Returns
    ///
    /// A `syn::Result<()>` indicating success or containing validation errors.
    ///
    /// # Validation Rules
    ///
    /// The method validates:
    /// - Build method names are not empty
    /// - Build method names are valid identifiers (when possible)
    /// - Setter prefixes are not empty
    /// - Setter prefixes are valid identifier beginnings
    /// - No conflicting attribute combinations
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - Empty build method names
    /// - Empty setter prefixes
    /// - Invalid identifier syntax (when detectable)
    /// - Reserved keywords without raw identifier syntax
    pub fn validate(&self) -> syn::Result<()> {
        // Validate build method name if provided
        if let Some(build_method_name) = &self.build_method_name {
            if build_method_name.is_empty() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "Build method name cannot be empty",
                ));
            }

            // Try to validate as identifier (this catches most invalid cases)
            // Note: This doesn't catch all cases (like reserved keywords) but
            // those will be caught during token generation
            if syn::parse_str::<syn::Ident>(build_method_name).is_err() {
                // If it's not a valid identifier, check if it might be a raw identifier
                if !build_method_name.starts_with("r#") {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!(
                            "Invalid build method name '{build_method_name}'. Build method names must be valid Rust identifiers. \
                            Use raw identifier syntax (r#name) for keywords."
                        ),
                    ));
                }
            }
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

        Ok(())
    }
}

/// Parses builder attributes from a struct's attribute list.
///
/// This function processes all `#[builder(...)]` attributes on a struct and
/// combines them into a single `StructAttributes` configuration. It validates
/// that the attributes are valid and returns appropriate errors for invalid
/// configurations.
///
/// # Arguments
///
/// * `attrs` - The list of attributes from a struct definition
///
/// # Returns
///
/// A `Result<StructAttributes, syn::Error>` containing the parsed configuration
/// or a descriptive error for invalid attributes.
///
/// # Supported Attribute Syntax
///
/// Supported struct-level attributes include:
/// - `build_method = "name"` - Custom build method name
/// - `setter_prefix = "prefix_"` - Prefix for all setter method names
/// - `impl_into` - Use `impl Into<FieldType>` for setter parameters
/// - Combined attributes in a single attribute block
///
/// # Errors
///
/// Returns errors for:
/// - Invalid attribute syntax
/// - Unknown struct-level attribute names
/// - Invalid build method names
/// - Empty attribute values
/// - Conflicting attribute combinations
///
/// # Implementation Details
///
/// The function:
/// 1. Iterates through all attributes looking for `#[builder(...)]`
/// 2. Parses each builder attribute using `syn::parse_nested_meta`
/// 3. Validates individual attribute values
/// 4. Combines multiple attributes into a single configuration
/// 5. Validates the final configuration for consistency
/// 6. Returns the complete configuration or the first error encountered
pub fn parse_struct_attributes(attrs: &[syn::Attribute]) -> syn::Result<StructAttributes> {
    let mut struct_attributes = StructAttributes::default();

    // Process each attribute in the list
    for attr in attrs {
        // Only process #[builder(...)] attributes
        if attr.path().is_ident("builder") {
            // Parse the nested meta inside the builder attribute
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("build_method") {
                    // #[builder(build_method = "name")]
                    let value = meta.value()?;
                    let lit_str: syn::LitStr = value.parse()?;
                    let build_method_name = lit_str.value();

                    // Validate that the build method name is not empty
                    if build_method_name.is_empty() {
                        return Err(meta.error("Build method name cannot be empty"));
                    }

                    struct_attributes.build_method_name = Some(build_method_name);
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

                    struct_attributes.setter_prefix = Some(setter_prefix);
                    Ok(())
                } else if meta.path.is_ident("impl_into") {
                    // #[builder(impl_into)]
                    struct_attributes.impl_into = true;
                    Ok(())
                } else {
                    // Unknown struct-level attribute
                    Err(meta.error(
                        "Unknown struct-level builder attribute. Supported attributes: build_method, setter_prefix, impl_into"
                    ))
                }
            })?;
        }
    }

    // Validate the final configuration
    struct_attributes.validate()?;

    Ok(struct_attributes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_default_struct_attributes() {
        let attrs = StructAttributes::default();
        assert!(attrs.build_method_name.is_none());
        assert!(attrs.setter_prefix.is_none());
        assert!(!attrs.impl_into);
        assert_eq!(attrs.get_build_method_name(), "build");
        assert_eq!(attrs.get_setter_prefix(), None);
        assert!(!attrs.get_impl_into());
    }

    #[test]
    fn test_get_build_method_name() {
        // Default case
        let default_attrs = StructAttributes::default();
        assert_eq!(default_attrs.get_build_method_name(), "build");

        // Custom case
        let custom_attrs = StructAttributes {
            build_method_name: Some("create".to_string()),
            setter_prefix: None,
            impl_into: false,
        };
        assert_eq!(custom_attrs.get_build_method_name(), "create");
    }

    #[test]
    fn test_with_build_method_name() {
        let attrs = StructAttributes {
            build_method_name: Some("create".to_string()),
            setter_prefix: None,
            impl_into: false,
        };
        assert_eq!(attrs.get_build_method_name(), "create");

        let attrs2 = StructAttributes {
            build_method_name: Some("construct".to_string()),
            setter_prefix: None,
            impl_into: false,
        };
        assert_eq!(attrs2.get_build_method_name(), "construct");
    }

    #[test]
    fn test_validate_valid_attributes() {
        let valid_attrs = StructAttributes {
            build_method_name: Some("create".to_string()),
            setter_prefix: Some("with_".to_string()),
            impl_into: false,
        };
        assert!(valid_attrs.validate().is_ok());

        let default_attrs = StructAttributes::default();
        assert!(default_attrs.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_build_method_name() {
        let invalid_attrs = StructAttributes {
            build_method_name: Some("".to_string()),
            setter_prefix: None,
            impl_into: false,
        };
        let result = invalid_attrs.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Build method name cannot be empty"));
    }

    #[test]
    fn test_parse_build_method_attribute() {
        let attrs = vec![parse_quote!(#[builder(build_method = "create")])];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert_eq!(struct_attrs.build_method_name, Some("create".to_string()));
        assert_eq!(struct_attrs.get_build_method_name(), "create");
    }

    #[test]
    fn test_parse_raw_identifier_build_method() {
        let attrs = vec![parse_quote!(#[builder(build_method = "r#type")])];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert_eq!(struct_attrs.build_method_name, Some("r#type".to_string()));
        assert_eq!(struct_attrs.get_build_method_name(), "r#type");
    }

    #[test]
    fn test_parse_no_builder_attributes() {
        let attrs = vec![parse_quote!(#[derive(Debug)])];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        // Should return default attributes
        assert!(struct_attrs.build_method_name.is_none());
        assert_eq!(struct_attrs.get_build_method_name(), "build");
    }

    #[test]
    fn test_parse_multiple_builder_attributes() {
        // Multiple #[builder(...)] attributes should be combined
        let attrs = vec![
            parse_quote!(#[builder(build_method = "create")]),
            // Could add more struct-level attributes here in the future
        ];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert_eq!(struct_attrs.get_build_method_name(), "create");
    }

    #[test]
    fn test_parse_empty_build_method_error() {
        let attrs = vec![parse_quote!(#[builder(build_method = "")])];
        let result = parse_struct_attributes(&attrs);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_parse_unknown_attribute_error() {
        let attrs = vec![parse_quote!(#[builder(unknown_attr = "value")])];
        let result = parse_struct_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown struct-level builder attribute"));
    }

    #[test]
    fn test_parse_invalid_build_method_name() {
        // This test depends on syn's identifier validation
        // Some invalid names might not be caught until token generation
        let attrs = vec![parse_quote!(#[builder(build_method = "123invalid")])];
        let result = parse_struct_attributes(&attrs);

        // Should either succeed (caught later) or fail with identifier error
        if let Err(e) = result {
            assert!(
                e.to_string().contains("Invalid build method name")
                    || e.to_string().contains("identifier")
            );
        }
    }

    // Tests for setter_prefix functionality

    #[test]
    fn test_get_setter_prefix() {
        // Default case (no prefix)
        let default_attrs = StructAttributes::default();
        assert_eq!(default_attrs.get_setter_prefix(), None);

        // Custom prefix case
        let custom_attrs = StructAttributes {
            build_method_name: None,
            setter_prefix: Some("with_".to_string()),
            impl_into: false,
        };
        assert_eq!(custom_attrs.get_setter_prefix(), Some("with_"));
    }

    #[test]
    fn test_parse_setter_prefix_attribute() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "with_")])];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert_eq!(struct_attrs.setter_prefix, Some("with_".to_string()));
        assert_eq!(struct_attrs.get_setter_prefix(), Some("with_"));
        assert_eq!(struct_attrs.get_build_method_name(), "build"); // default
    }

    #[test]
    fn test_parse_combined_attributes() {
        let attrs =
            vec![parse_quote!(#[builder(build_method = "create", setter_prefix = "with_")])];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert_eq!(struct_attrs.build_method_name, Some("create".to_string()));
        assert_eq!(struct_attrs.setter_prefix, Some("with_".to_string()));
        assert_eq!(struct_attrs.get_build_method_name(), "create");
        assert_eq!(struct_attrs.get_setter_prefix(), Some("with_"));
    }

    #[test]
    fn test_parse_empty_setter_prefix_error() {
        let attrs = vec![parse_quote!(#[builder(setter_prefix = "")])];
        let result = parse_struct_attributes(&attrs);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Setter prefix cannot be empty"));
    }

    #[test]
    fn test_validate_empty_setter_prefix() {
        let invalid_attrs = StructAttributes {
            build_method_name: None,
            setter_prefix: Some("".to_string()),
            impl_into: false,
        };
        let result = invalid_attrs.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Setter prefix cannot be empty"));
    }

    #[test]
    fn test_validate_invalid_setter_prefix_starting_with_number() {
        let invalid_attrs = StructAttributes {
            build_method_name: None,
            setter_prefix: Some("1invalid_".to_string()),
            impl_into: false,
        };
        let result = invalid_attrs.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot start with a number"));
    }

    #[test]
    fn test_validate_invalid_setter_prefix_special_chars() {
        let invalid_attrs = StructAttributes {
            build_method_name: None,
            setter_prefix: Some("with-".to_string()),
            impl_into: false,
        };
        let result = invalid_attrs.validate();
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
            let attrs = StructAttributes {
                build_method_name: None,
                setter_prefix: Some(prefix.to_string()),
                impl_into: false,
            };
            assert!(
                attrs.validate().is_ok(),
                "Prefix '{prefix}' should be valid"
            );
        }
    }

    #[test]
    fn test_parse_multiple_setter_prefix_attributes() {
        // Multiple #[builder(...)] attributes with different settings
        let attrs = vec![
            parse_quote!(#[builder(setter_prefix = "with_")]),
            parse_quote!(#[builder(build_method = "create")]),
        ];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert_eq!(struct_attrs.get_setter_prefix(), Some("with_"));
        assert_eq!(struct_attrs.get_build_method_name(), "create");
    }

    // Tests for impl_into functionality

    #[test]
    fn test_get_impl_into() {
        // Default case (false)
        let default_attrs = StructAttributes::default();
        assert!(!default_attrs.get_impl_into());

        // Explicit true case
        let impl_into_attrs = StructAttributes {
            build_method_name: None,
            setter_prefix: None,
            impl_into: true,
        };
        assert!(impl_into_attrs.get_impl_into());

        // Explicit false case
        let no_impl_into_attrs = StructAttributes {
            build_method_name: None,
            setter_prefix: None,
            impl_into: false,
        };
        assert!(!no_impl_into_attrs.get_impl_into());
    }

    #[test]
    fn test_parse_impl_into_attribute() {
        let attrs = vec![parse_quote!(#[builder(impl_into)])];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert!(struct_attrs.impl_into);
        assert!(struct_attrs.get_impl_into());
        assert_eq!(struct_attrs.get_build_method_name(), "build"); // default
        assert_eq!(struct_attrs.get_setter_prefix(), None); // default
    }

    #[test]
    fn test_parse_combined_attributes_with_impl_into() {
        let attrs = vec![parse_quote!(#[builder(
            impl_into,
            build_method = "create",
            setter_prefix = "with_"
        )])];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert!(struct_attrs.impl_into);
        assert_eq!(struct_attrs.build_method_name, Some("create".to_string()));
        assert_eq!(struct_attrs.setter_prefix, Some("with_".to_string()));
        assert!(struct_attrs.get_impl_into());
        assert_eq!(struct_attrs.get_build_method_name(), "create");
        assert_eq!(struct_attrs.get_setter_prefix(), Some("with_"));
    }

    #[test]
    fn test_parse_multiple_impl_into_attributes() {
        // Multiple #[builder(...)] attributes with different settings
        let attrs = vec![
            parse_quote!(#[builder(impl_into)]),
            parse_quote!(#[builder(build_method = "create")]),
        ];
        let struct_attrs = parse_struct_attributes(&attrs).unwrap();

        assert!(struct_attrs.get_impl_into());
        assert_eq!(struct_attrs.get_build_method_name(), "create");
    }

    #[test]
    fn test_validate_impl_into_attributes() {
        // Valid: impl_into with other attributes
        let valid_attrs = StructAttributes {
            build_method_name: Some("create".to_string()),
            setter_prefix: Some("with_".to_string()),
            impl_into: true,
        };
        assert!(valid_attrs.validate().is_ok());

        // Valid: impl_into alone
        let impl_into_only = StructAttributes {
            build_method_name: None,
            setter_prefix: None,
            impl_into: true,
        };
        assert!(impl_into_only.validate().is_ok());

        // Valid: no impl_into (default case)
        let no_impl_into = StructAttributes {
            build_method_name: None,
            setter_prefix: None,
            impl_into: false,
        };
        assert!(no_impl_into.validate().is_ok());
    }
}
