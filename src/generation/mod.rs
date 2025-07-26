//! Code Generation Module
//!
//! This module contains all the code generation logic for creating type-safe
//! builders from struct definitions. It's organized into specialized generators
//! that handle different aspects of the builder pattern.
//!
//! # Generation Strategy
//!
//! The generation process follows this strategy:
//! 1. **Analysis Phase** - Complete struct analysis and validation
//! 2. **Pattern Selection** - Choose between type-state and regular builders
//! 3. **Token Generation** - Create the appropriate Rust code tokens
//! 4. **Assembly** - Combine all generated components into final output
//!
//! # Module Organization
//!
//! - [`tokens`]: Core token generation utilities and the `TokenGenerator`
//! - [`templates`]: Template-based code generation system for maintainable patterns
//! - [`type_state_builder`]: Type-state builder pattern implementation
//! - [`regular_builder`]: Simple builder pattern for optional-only structs
//!
//! # Builder Pattern Selection
//!
//! The module automatically selects the appropriate builder pattern:
//!
//! - **Type-State Builder**: Used when the struct has required fields
//!   - Provides compile-time validation that all required fields are set
//!   - Uses different types for different builder states
//!   - More complex but prevents runtime errors
//!
//! - **Regular Builder**: Used when all fields are optional
//!   - Simple builder with immediate `build()` availability
//!   - Single builder type with straightforward implementation
//!   - More user-friendly for simple cases
//!

pub mod regular_builder;
pub mod tokens;
pub mod type_state_builder;

// Re-export main types and functions for convenience
pub use self::type_state_builder::generate_type_state_builder;
pub use regular_builder::generate_regular_builder;
pub use tokens::TokenGenerator;

use crate::analysis::StructAnalysis;

/// Generates a complete builder implementation for a struct.
///
/// This is the main entry point for builder generation. It analyzes the struct
/// configuration and automatically selects the appropriate builder pattern
/// (type-state or regular) based on the field requirements.
///
/// # Arguments
///
/// * `analysis` - Complete struct analysis containing all necessary information
///
/// # Returns
///
/// A `syn::Result<proc_macro2::TokenStream>` containing the complete builder
/// implementation or an error if generation fails.
///
/// # Builder Selection Logic
///
/// The function uses this logic to select the builder pattern:
/// - **Type-State Builder**: If the struct has any required fields
/// - **Regular Builder**: If all fields are optional
///
///
///
/// # Generated Code Structure
///
/// The generated code typically includes:
/// - Builder struct(s) with appropriate fields
/// - Constructor methods on the original struct
/// - Setter methods for each field (unless skipped)
/// - Build method(s) to create the final instance
/// - Appropriate generic parameter handling
/// - Documentation for all generated items
///
/// # Errors
///
/// Returns errors for:
/// - Invalid struct configurations that passed analysis
/// - Token generation failures
/// - Generic parameter handling issues
/// - Internal consistency errors during generation
pub fn generate_builder(analysis: &StructAnalysis) -> syn::Result<proc_macro2::TokenStream> {
    // Validate the analysis before generation
    analysis.validate_for_generation()?;

    // Select the appropriate builder pattern based on field requirements
    if analysis.has_only_optional_fields() {
        // All fields are optional - use the simpler regular builder pattern
        generate_regular_builder(analysis)
    } else {
        // Has required fields - use the type-state builder pattern for compile-time safety
        generate_type_state_builder(analysis)
    }
}

/// Configuration for builder generation behavior.
///
/// This struct allows customization of the generation process beyond what's
/// available through attributes. It's primarily used internally but could
/// be exposed for advanced use cases.
///
/// # Design Philosophy
///
/// The configuration follows these principles:
/// - **Sensible defaults** - Works well out of the box
/// - **Minimal configuration** - Most behavior derived from struct analysis
/// - **Clear overrides** - Explicit settings for when defaults aren't suitable
/// - **Future extensibility** - Easy to add new options without breaking changes
#[derive(Debug, Clone)]
pub struct GenerationConfig {
    /// Whether to include detailed documentation in generated code
    pub include_documentation: bool,

    /// Whether to include helpful error messages for type mismatches
    pub include_error_guidance: bool,

    /// Whether to generate Debug implementations for builder types
    pub generate_debug_impls: bool,

    /// Whether to use fully qualified paths in generated code
    pub use_qualified_paths: bool,
}

impl Default for GenerationConfig {
    /// Creates the default generation configuration.
    ///
    /// The default configuration:
    /// - Includes comprehensive documentation
    /// - Includes helpful error messages
    /// - Generates Debug implementations for easier debugging
    /// - Uses fully qualified paths for reliability
    fn default() -> Self {
        Self {
            include_documentation: true,
            include_error_guidance: true,
            generate_debug_impls: true,
            use_qualified_paths: true,
        }
    }
}

impl GenerationConfig {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::analyze_struct;
    use syn::parse_quote;

    #[test]
    fn test_generate_builder_with_required_fields() {
        let input = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
                age: Option<u32>,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_builder(&analysis);

        assert!(result.is_ok());
        // Should generate type-state builder since there are required fields
    }

    #[test]
    fn test_generate_builder_with_optional_only() {
        let input = parse_quote! {
            struct Example {
                name: Option<String>,
                age: u32,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_builder(&analysis);

        assert!(result.is_ok());
        // Should generate regular builder since all fields are optional
    }

    #[test]
    fn test_generation_config_default() {
        let config = GenerationConfig::default();

        assert!(config.include_documentation);
        assert!(config.include_error_guidance);
        assert!(config.generate_debug_impls);
        assert!(config.use_qualified_paths);
    }

    #[test]
    fn test_generate_builder_with_custom_config() {
        let input = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let _config = GenerationConfig::default();
        let result = generate_builder(&analysis);

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_selection_logic() {
        // Test type-state builder selection
        let input_with_required = parse_quote! {
            struct WithRequired {
                #[builder(required)]
                name: String,
                age: Option<u32>,
            }
        };
        let analysis = analyze_struct(&input_with_required).unwrap();
        assert!(!analysis.has_only_optional_fields());

        // Test regular builder selection
        let input_optional_only = parse_quote! {
            struct OptionalOnly {
                name: Option<String>,
                age: u32,
            }
        };
        let analysis = analyze_struct(&input_optional_only).unwrap();
        assert!(analysis.has_only_optional_fields());
    }
}
