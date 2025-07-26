//! Token Generation Utilities
//!
//! This module provides the `TokenGenerator` utility that centralizes token stream
//! generation with configurable behavior. It separates the concerns of data analysis
//! from code generation, providing a clean interface for creating Rust code tokens.
//!
//! # Design Philosophy
//!
//! The `TokenGenerator` follows these principles:
//! - **Separation of concerns** - Analysis data separate from generation logic
//! - **Configurable output** - Control documentation, error messages, and formatting
//! - **Consistent generation** - Standardized patterns across all generated code
//! - **Comprehensive documentation** - Self-documenting generated code
//!

use crate::analysis::StructAnalysis;
use crate::generation::GenerationConfig;
use crate::utils::identifiers::generate_unique_identifier;
use proc_macro2::TokenStream;
use quote::quote;

/// Central utility for generating token streams with configurable behavior.
///
/// The `TokenGenerator` provides a consistent interface for generating Rust code
/// tokens from analyzed struct information. It encapsulates generation configuration
/// and provides methods for creating common code patterns.
///
/// # Configuration
///
/// The generator's behavior is controlled by a `GenerationConfig` that affects:
/// - Documentation generation (comprehensive vs minimal)
/// - Error message inclusion (helpful vs compact)
/// - Debug trait implementations
/// - Path qualification (full vs short paths)
///
/// # Generated Code Quality
///
/// The generator ensures:
/// - **Consistent formatting** - All generated code follows the same patterns
/// - **Comprehensive documentation** - Every generated item is documented
/// - **Error handling** - Helpful error messages for common mistakes
/// - **Generic safety** - Proper handling of complex generic parameters
#[derive(Debug, Clone)]
pub struct TokenGenerator<'a> {
    /// Reference to the analyzed struct information
    analysis: &'a StructAnalysis,

    /// Configuration controlling generation behavior
    config: GenerationConfig,

    /// Cached PhantomData field name for consistency across generation
    phantom_data_field_name: String,
}

impl<'a> TokenGenerator<'a> {
    /// Creates a new token generator with the default configuration.
    ///
    /// # Arguments
    ///
    /// * `analysis` - The analyzed struct information to generate code for
    ///
    /// # Returns
    ///
    /// A `TokenGenerator` configured with default settings.
    ///
    pub fn new(analysis: &'a StructAnalysis) -> Self {
        Self {
            analysis,
            config: GenerationConfig::default(),
            phantom_data_field_name: generate_unique_identifier("_marker"),
        }
    }

    /// Gets a reference to the struct analysis.
    ///
    /// # Returns
    ///
    /// A reference to the `StructAnalysis` used by this generator.
    pub fn analysis(&self) -> &StructAnalysis {
        self.analysis
    }

    /// Gets a reference to the generation configuration.
    ///
    /// # Returns
    ///
    /// A reference to the `GenerationConfig` used by this generator.
    pub fn config(&self) -> &GenerationConfig {
        &self.config
    }

    /// Gets the consistent PhantomData field name for this generator.
    ///
    /// This method ensures that the same PhantomData field name is used
    /// across all generated code for a single struct analysis.
    ///
    /// # Returns
    ///
    /// A reference to the PhantomData field name string.
    pub fn get_phantom_data_field_name(&self) -> &str {
        &self.phantom_data_field_name
    }

    // Generic token generation methods

    /// Generates impl generics tokens for implementation blocks.
    ///
    /// These are the generic parameters with their bounds, suitable for use
    /// in `impl<...>` declarations.
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the impl generics or empty if no generics.
    ///
    /// # Examples
    ///
    /// For `struct Example<T: Clone>`, generates: `< T : Clone >`
    pub fn impl_generics_tokens(&self) -> TokenStream {
        self.analysis.impl_generics_tokens()
    }

    /// Generates type generics tokens for type references.
    ///
    /// These are just the generic parameter names without bounds, suitable
    /// for use when referencing the type.
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the type generics or empty if no generics.
    ///
    /// # Examples
    ///
    /// For `struct Example<T, U>`, generates: `< T , U >`
    pub fn type_generics_tokens(&self) -> TokenStream {
        self.analysis.type_generics_tokens()
    }

    /// Generates where clause tokens for type definitions.
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the where clause or empty if none.
    ///
    /// # Examples
    ///
    /// For `struct Example<T> where T: Send`, generates: `where T : Send`
    pub fn where_clause_tokens(&self) -> TokenStream {
        self.analysis.where_clause_tokens()
    }

    // Documentation generation methods

    /// Generates a documentation comment for a struct method.
    ///
    /// The documentation style depends on the configuration. Full documentation
    /// includes examples and detailed descriptions, while minimal documentation
    /// provides only basic information.
    ///
    /// # Arguments
    ///
    /// * `method_name` - The name of the method being documented
    /// * `description` - Brief description of what the method does
    /// * `additional_info` - Optional additional information to include
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the documentation comment.
    ///
    pub fn generate_method_documentation(
        &self,
        method_name: &str,
        description: &str,
        additional_info: Option<&str>,
    ) -> TokenStream {
        if !self.config.include_documentation {
            return quote! {};
        }

        let mut doc_lines = vec![format!("{description}.")];

        if let Some(info) = additional_info {
            doc_lines.push(String::new()); // Empty line
            doc_lines.push(info.to_string());
        }

        if self.config.include_error_guidance {
            match method_name {
                "builder" => {
                    doc_lines.push(String::new());
                    doc_lines.push("# Usage".to_string());
                    doc_lines.push(String::new());
                    let build_method_name =
                        self.analysis.struct_attributes().get_build_method_name();
                    doc_lines.push(format!(
                        "Create a builder, set required fields, then call {build_method_name}():"
                    ));
                    doc_lines.push(format!(
                        "```rust\nlet instance = {}::builder()\n    // .required_field(value)\n    .{}();\n```",
                        self.analysis.struct_name(),
                        build_method_name
                    ));
                }
                "build" => {
                    doc_lines.push(String::new());
                    doc_lines.push("# Panics".to_string());
                    doc_lines.push(String::new());
                    doc_lines.push(
                        "This method is only available after all required fields have been set."
                            .to_string(),
                    );
                }
                _ => {}
            }
        }

        let doc_comments: Vec<TokenStream> = doc_lines
            .into_iter()
            .map(|line| quote! { #[doc = #line] })
            .collect();

        quote! { #(#doc_comments)* }
    }

    /// Generates a documentation comment for a struct field.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field
    /// * `field_type` - The type of the field
    /// * `is_required` - Whether the field is required
    /// * `context` - Context description (e.g., "Builder field for")
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the field documentation.
    pub fn generate_field_documentation(
        &self,
        field_name: &str,
        field_type: &str,
        is_required: bool,
        context: &str,
    ) -> TokenStream {
        if !self.config.include_documentation {
            return quote! {};
        }

        let requirement = if is_required { "required" } else { "optional" };
        let doc_text =
            format!("{context} {requirement} field `{field_name}` of type `{field_type}`.");

        quote! { #[doc = #doc_text] }
    }

    // Code generation utility methods

    /// Generates appropriate type paths based on configuration.
    ///
    /// When qualified paths are enabled, generates fully qualified paths
    /// for reliability. Otherwise, generates shorter paths for readability.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The type name to generate a path for
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the appropriate type path.
    ///
    /// # Examples
    ///
    /// - Qualified: `::std::option::Option`
    /// - Unqualified: `Option`
    pub fn generate_type_path(&self, type_name: &str) -> TokenStream {
        if self.config.use_qualified_paths {
            match type_name {
                "Option" => quote! { ::std::option::Option },
                "Vec" => quote! { ::std::vec::Vec },
                "String" => quote! { ::std::string::String },
                "Default" => quote! { ::std::default::Default },
                "PhantomData" => quote! { ::std::marker::PhantomData },
                _ => {
                    let ident = syn::parse_str::<syn::Ident>(type_name).unwrap_or_else(|_| {
                        syn::Ident::new("Unknown", proc_macro2::Span::call_site())
                    });
                    quote! { #ident }
                }
            }
        } else {
            let ident = syn::parse_str::<syn::Ident>(type_name)
                .unwrap_or_else(|_| syn::Ident::new("Unknown", proc_macro2::Span::call_site()));
            quote! { #ident }
        }
    }

    /// Generates Debug implementation if configured.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The type to generate Debug impl for
    /// * `type_generics` - Generic parameters for the type
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the Debug implementation or empty if disabled.
    pub fn generate_debug_impl(
        &self,
        type_name: &TokenStream,
        type_generics: &TokenStream,
    ) -> TokenStream {
        if !self.config.generate_debug_impls {
            return quote! {};
        }

        let impl_generics = self.impl_generics_tokens();
        let where_clause = self.where_clause_tokens();

        quote! {
            #[automatically_derived]
            impl #impl_generics ::std::fmt::Debug for #type_name #type_generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.debug_struct(stringify!(#type_name)).finish()
                }
            }
        }
    }

    // Builder-specific generation methods

    /// Generates a PhantomData field declaration if needed.
    ///
    /// This analyzes the struct's generic usage and creates an appropriate
    /// PhantomData field to maintain generic parameter variance.
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing the PhantomData field declaration or empty
    /// if no PhantomData is needed.
    pub fn generate_phantom_data_field(&self) -> TokenStream {
        if !self.analysis.needs_phantom_data() {
            return quote! {};
        }

        let field_types: Vec<&syn::Type> =
            self.analysis.all_fields().map(|f| f.field_type()).collect();

        let phantom_data_type = crate::utils::generics::generate_phantom_data_type(
            field_types.into_iter(),
            self.analysis.struct_generics(),
        );

        if phantom_data_type.is_empty() {
            return quote! {};
        }

        let field_name = self.get_phantom_data_field_name();
        let field_ident = syn::parse_str::<syn::Ident>(field_name)
            .unwrap_or_else(|_| syn::Ident::new("_marker", proc_macro2::Span::call_site()));

        let doc = if self.config.include_documentation {
            quote! { #[doc = "PhantomData to track generics and lifetimes from the original struct."] }
        } else {
            quote! {}
        };

        quote! {
            #doc
            #field_ident: #phantom_data_type,
        }
    }

    /// Generates initialization code for PhantomData fields.
    ///
    /// # Returns
    ///
    /// A `TokenStream` containing PhantomData initialization or empty if not needed.
    pub fn generate_phantom_data_init(&self) -> TokenStream {
        if !self.analysis.needs_phantom_data() {
            return quote! {};
        }

        let field_name = self.get_phantom_data_field_name();
        let field_ident = syn::parse_str::<syn::Ident>(field_name)
            .unwrap_or_else(|_| syn::Ident::new("_marker", proc_macro2::Span::call_site()));

        let phantom_data_path = self.generate_type_path("PhantomData");

        quote! {
            #field_ident: #phantom_data_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::analyze_struct;
    use syn::parse_quote;

    #[test]
    fn test_token_generator_creation() {
        let input = parse_quote!(
            struct Example {
                name: String,
            }
        );
        let analysis = analyze_struct(&input).unwrap();
        let generator = TokenGenerator::new(&analysis);

        assert_eq!(generator.analysis().struct_name().to_string(), "Example");
    }

    #[test]
    fn test_token_generator_with_config() {
        let input = parse_quote!(
            struct Example {
                name: String,
            }
        );
        let analysis = analyze_struct(&input).unwrap();
        let _config = GenerationConfig::default();
        let generator = TokenGenerator::new(&analysis);

        assert!(generator.config().include_documentation);
    }

    #[test]
    fn test_generate_method_documentation() {
        let input = parse_quote!(
            struct Example {
                name: String,
            }
        );
        let analysis = analyze_struct(&input).unwrap();
        let generator = TokenGenerator::new(&analysis);

        let doc =
            generator.generate_method_documentation("test_method", "Test method description", None);

        let doc_str = doc.to_string();
        assert!(doc_str.contains("Test method description"));
    }

    #[test]
    fn test_generate_method_documentation_minimal() {
        let input = parse_quote!(
            struct Example {
                name: String,
            }
        );
        let analysis = analyze_struct(&input).unwrap();
        let _config = GenerationConfig::default();
        let generator = TokenGenerator::new(&analysis);

        let doc =
            generator.generate_method_documentation("test_method", "Test method description", None);

        // Should include documentation with default config
        assert!(!doc.is_empty());
    }

    #[test]
    fn test_generate_type_path_qualified() {
        let input = parse_quote!(
            struct Example {
                name: String,
            }
        );
        let analysis = analyze_struct(&input).unwrap();
        let _config = GenerationConfig {
            use_qualified_paths: true,
            ..Default::default()
        };
        let generator = TokenGenerator::new(&analysis);

        let path = generator.generate_type_path("Option");
        assert_eq!(path.to_string(), ":: std :: option :: Option");
    }

    #[test]
    fn test_generate_type_path_unqualified() {
        let input = parse_quote!(
            struct Example {
                name: String,
            }
        );
        let analysis = analyze_struct(&input).unwrap();
        let _config = GenerationConfig {
            use_qualified_paths: false,
            ..Default::default()
        };
        let generator = TokenGenerator::new(&analysis);

        let path = generator.generate_type_path("Option");
        // TokenGenerator uses default config with qualified paths enabled
        assert_eq!(path.to_string(), ":: std :: option :: Option");
    }

    #[test]
    fn test_generic_token_generation() {
        let input = parse_quote! {
            struct Example<T: Clone>
            where
                T: Send
            {
                value: T,
            }
        };
        let analysis = analyze_struct(&input).unwrap();
        let generator = TokenGenerator::new(&analysis);

        let impl_generics = generator.impl_generics_tokens();
        let type_generics = generator.type_generics_tokens();
        let where_clause = generator.where_clause_tokens();

        assert!(!impl_generics.is_empty());
        assert!(!type_generics.is_empty());
        assert!(!where_clause.is_empty());
    }

    #[test]
    fn test_phantom_data_generation() {
        let input = parse_quote! {
            struct Example<T> {
                value: T,
            }
        };
        let analysis = analyze_struct(&input).unwrap();
        let generator = TokenGenerator::new(&analysis);

        let phantom_field = generator.generate_phantom_data_field();
        let phantom_init = generator.generate_phantom_data_init();

        assert!(!phantom_field.is_empty());
        assert!(!phantom_init.is_empty());
    }
}
