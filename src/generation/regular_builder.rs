//! Regular Builder Generation
//!
//! This module implements simple builder pattern generation for structs that
//! have only optional fields. Unlike the type-state builder, this generates
//! a straightforward builder with immediate build() method availability.
//!
//! # Regular Builder Pattern Overview
//!
//! The regular builder pattern is used when:
//! - All struct fields are optional (no required fields)
//! - Simple, immediate builder usage is preferred
//! - Compile-time validation of required fields is not needed
//!
//! # Generated Code Structure
//!
//! For a struct with only optional fields, this generates:
//! - Single builder struct with all fields
//! - Constructor method on the original struct
//! - Setter methods for each field (unless skipped)
//! - Immediately available build method
//! - Default trait implementation for the builder
//!

use crate::analysis::StructAnalysis;
use crate::generation::TokenGenerator;
use quote::quote;
use syn::Ident;

/// Generates a complete regular builder implementation.
///
/// This is the main entry point for regular builder generation. It creates
/// a simple builder pattern suitable for structs with only optional fields.
///
/// # Arguments
///
/// * `analysis` - Complete struct analysis containing all necessary information
///
/// # Returns
///
/// A `syn::Result<proc_macro2::TokenStream>` containing the complete regular
/// builder implementation or an error if generation fails.
///
/// # Generated Components
///
/// The function generates:
/// 1. **Builder Struct** - Single struct containing all fields
/// 2. **Constructor Method** - `YourStruct::builder()` method
/// 3. **Default Implementation** - For easy initialization
/// 4. **Setter Methods** - For each field that should have a setter
/// 5. **Build Method** - Immediately available to construct the target struct
///
pub fn generate_regular_builder(
    analysis: &StructAnalysis,
) -> syn::Result<proc_macro2::TokenStream> {
    let token_generator = TokenGenerator::new(analysis);
    generate_with_token_generator(&token_generator)
}

/// Generates a regular builder using a specific token generator.
///
/// This function is used internally and for custom configuration scenarios
/// where a pre-configured token generator is available.
///
/// # Arguments
///
/// * `token_generator` - Configured token generator for code generation
///
/// # Returns
///
/// A `syn::Result<proc_macro2::TokenStream>` containing the builder implementation.
pub fn generate_with_token_generator(
    token_generator: &TokenGenerator,
) -> syn::Result<proc_macro2::TokenStream> {
    let builder_coordinator = RegularBuilderCoordinator::new(token_generator);
    builder_coordinator.generate_complete_implementation()
}

/// Coordinator for regular builder generation.
///
/// This struct encapsulates the logic for generating all components of a
/// regular builder, providing methods for creating different aspects of
/// the implementation.
struct RegularBuilderCoordinator<'a> {
    /// Token generator for consistent code generation
    token_generator: &'a TokenGenerator<'a>,

    /// Unique name for the builder struct
    builder_name: String,
}

impl<'a> RegularBuilderCoordinator<'a> {
    /// Creates a new coordinator with the given token generator.
    ///
    /// # Arguments
    ///
    /// * `token_generator` - Token generator to use for code generation
    ///
    /// # Returns
    ///
    /// A new `RegularBuilderCoordinator` ready for generation.
    fn new(token_generator: &'a TokenGenerator<'a>) -> Self {
        let struct_name = token_generator.analysis().struct_name();
        let builder_name = format!("{struct_name}Builder");

        Self {
            token_generator,
            builder_name,
        }
    }

    /// Generates the complete regular builder implementation.
    ///
    /// This orchestrates the generation of all components needed for a
    /// functional regular builder.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the complete implementation.
    fn generate_complete_implementation(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut tokens = proc_macro2::TokenStream::new();

        // Generate the builder struct definition
        tokens.extend(self.generate_builder_struct()?);

        // Generate the constructor method on the original struct
        tokens.extend(self.generate_struct_constructor_method()?);

        // Generate the Default implementation for the builder
        tokens.extend(self.generate_default_implementation()?);

        // Generate the main implementation block with all methods
        tokens.extend(self.generate_builder_implementation()?);

        Ok(tokens)
    }

    /// Generates the builder struct definition.
    ///
    /// Creates the struct that will hold all field values during the
    /// building process.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the builder struct definition.
    fn generate_builder_struct(&self) -> syn::Result<proc_macro2::TokenStream> {
        let builder_ident = syn::parse_str::<Ident>(&self.builder_name)?;
        let analysis = self.token_generator.analysis();

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        // Generate field declarations
        let field_declarations = self.generate_builder_field_declarations()?;

        // Generate documentation for the builder struct
        let doc = self.token_generator.generate_method_documentation(
            &self.builder_name,
            &{
                let struct_name = analysis.struct_name();
                format!("Builder struct for {struct_name} with optional field customization")
            },
            Some(&{
                let build_method = analysis.struct_attributes().get_build_method_name();
                format!("This builder provides a simple pattern for structs with only optional fields, allowing immediate construction via {build_method}() method.")
            })
        );

        // Generate Debug implementation if configured
        let debug_impl = self
            .token_generator
            .generate_debug_impl(&quote! { #builder_ident }, &type_generics);

        Ok(quote! {
            #doc
            struct #builder_ident #impl_generics #where_clause {
                #field_declarations
            }

            #debug_impl
        })
    }

    /// Generates field declarations for the builder struct.
    ///
    /// All fields are stored as their actual types since there are no
    /// required fields to track with Option wrappers.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing all field declarations.
    fn generate_builder_field_declarations(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut field_declarations = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Generate declarations for all optional fields
        for optional_field in analysis.optional_fields() {
            let field_name = optional_field.name();
            let field_type = optional_field.field_type();

            let doc = self.token_generator.generate_field_documentation(
                &optional_field.clean_name(),
                &quote! { #field_type }.to_string(),
                false,
                "Optional field",
            );

            field_declarations.extend(quote! {
                #doc
                #field_name: #field_type,
            });
        }

        // Add PhantomData field if needed for generic parameters
        field_declarations.extend(self.token_generator.generate_phantom_data_field());

        Ok(field_declarations)
    }

    /// Generates the constructor method on the original struct.
    ///
    /// This creates the `YourStruct::builder()` method that returns a
    /// default-initialized builder instance.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the constructor method.
    fn generate_struct_constructor_method(&self) -> syn::Result<proc_macro2::TokenStream> {
        let analysis = self.token_generator.analysis();
        let struct_name = analysis.struct_name();
        let builder_ident = syn::parse_str::<Ident>(&self.builder_name)?;

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        let doc = self.token_generator.generate_method_documentation(
            "builder",
            "Creates a new builder for constructing an instance with optional field customization",
            Some("All fields start with their default values and can be customized using setter methods.")
        );

        Ok(quote! {
            impl #impl_generics #struct_name #type_generics #where_clause {
                #doc
                pub fn builder() -> #builder_ident #type_generics {
                    #builder_ident::default()
                }
            }
        })
    }

    /// Generates the Default implementation for the builder.
    ///
    /// This allows the builder to be easily initialized with all fields
    /// set to their default values.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the Default implementation.
    fn generate_default_implementation(&self) -> syn::Result<proc_macro2::TokenStream> {
        let builder_ident = syn::parse_str::<Ident>(&self.builder_name)?;

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        // Generate default field initializations
        let default_field_init = self.generate_default_field_initializations()?;

        let doc = if self.token_generator.config().include_documentation {
            quote! {
                #[doc = "Creates a new builder with all fields initialized to their default values."]
            }
        } else {
            quote! {}
        };

        Ok(quote! {
            #doc
            impl #impl_generics Default for #builder_ident #type_generics #where_clause {
                fn default() -> Self {
                    Self {
                        #default_field_init
                    }
                }
            }
        })
    }

    /// Generates default field initialization code.
    ///
    /// Each field is initialized with either its custom default value
    /// or Default::default() for the field type.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing field initialization code.
    fn generate_default_field_initializations(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut field_init = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Initialize all optional fields with their defaults
        for optional_field in analysis.optional_fields() {
            let field_init_code = optional_field.generate_initialization(false)?;
            field_init.extend(field_init_code);
        }

        // Initialize PhantomData if needed
        field_init.extend(self.token_generator.generate_phantom_data_init());

        Ok(field_init)
    }

    /// Generates the main builder implementation with all methods.
    ///
    /// This creates the impl block containing the constructor, setter methods,
    /// and build method.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the complete implementation.
    fn generate_builder_implementation(&self) -> syn::Result<proc_macro2::TokenStream> {
        let builder_ident = syn::parse_str::<Ident>(&self.builder_name)?;

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        // Generate all method implementations
        let constructor_method = self.generate_constructor_method()?;
        let setter_methods = self.generate_setter_methods()?;
        let build_method = self.generate_build_method()?;

        Ok(quote! {
            impl #impl_generics #builder_ident #type_generics #where_clause {
                #constructor_method
                #setter_methods
                #build_method
            }
        })
    }

    /// Generates the constructor method for the builder.
    ///
    /// This provides an alternative to using Default::default().
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the constructor method.
    fn generate_constructor_method(&self) -> syn::Result<proc_macro2::TokenStream> {
        let builder_ident = syn::parse_str::<Ident>(&self.builder_name)?;
        let type_generics = self.token_generator.type_generics_tokens();

        let doc = self.token_generator.generate_method_documentation(
            "new",
            "Creates a new builder with all fields at default values",
            None,
        );

        Ok(quote! {
            #doc
            pub fn new() -> #builder_ident #type_generics {
                Self::default()
            }
        })
    }

    /// Generates setter methods for all optional fields.
    ///
    /// Each setter method allows customizing a field value and returns
    /// self for method chaining.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing all setter methods.
    fn generate_setter_methods(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut setter_methods = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Generate setter for each optional field that should have one
        let struct_setter_prefix = analysis.struct_attributes().get_setter_prefix();
        for optional_field in analysis.optional_fields() {
            if optional_field.should_generate_setter() {
                let setter_method = optional_field
                    .generate_setter_method(&syn::parse_quote!(Self), struct_setter_prefix)?;
                setter_methods.extend(setter_method);
            }
        }

        Ok(setter_methods)
    }

    /// Generates the build method that constructs the target struct.
    ///
    /// The build method is immediately available since there are no
    /// required fields to validate.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the build method.
    fn generate_build_method(&self) -> syn::Result<proc_macro2::TokenStream> {
        let analysis = self.token_generator.analysis();
        let struct_name = analysis.struct_name();
        let type_generics = self.token_generator.type_generics_tokens();

        // Generate field assignments for the target struct
        let struct_field_assignments = self.generate_struct_field_assignments()?;

        // Get build method name from configuration
        let build_method_name = analysis.struct_attributes().get_build_method_name();
        let build_method_ident = syn::parse_str::<Ident>(build_method_name)?;

        let doc = self.token_generator.generate_method_documentation(
            build_method_name,
            "Builds the final instance",
            Some("This method is immediately available since all fields are optional."),
        );

        Ok(quote! {
            #doc
            pub fn #build_method_ident(self) -> #struct_name #type_generics {
                #struct_name {
                    #struct_field_assignments
                }
            }
        })
    }

    /// Generates field assignments for the target struct construction.
    ///
    /// Simply copies all field values from the builder to the target struct.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing field assignment code.
    fn generate_struct_field_assignments(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut assignments = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Assign all optional fields by copying from builder
        for optional_field in analysis.optional_fields() {
            let field_name = optional_field.name();
            assignments.extend(quote! {
                #field_name: self.#field_name,
            });
        }

        Ok(assignments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::analyze_struct;
    use syn::parse_quote;

    #[test]
    fn test_generate_regular_builder() {
        let input = parse_quote! {
            struct Example {
                name: Option<String>,
                count: i32,
                active: bool,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_regular_builder(&analysis);

        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        // Should contain builder struct
        assert!(code.contains("struct"));
        assert!(code.contains("Builder"));

        // Should contain constructor method
        assert!(code.contains("builder"));

        // Should contain setter methods
        assert!(code.contains("name"));
        assert!(code.contains("count"));
        assert!(code.contains("active"));

        // Should contain build method
        assert!(code.contains("build"));

        // Should contain Default implementation
        assert!(code.contains("Default"));
    }

    #[test]
    fn test_regular_builder_with_custom_defaults() {
        let input = parse_quote! {
            struct Example {
                #[builder(default = "42")]
                count: i32,
                #[builder(default = "String::from(\"test\")")]
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_regular_builder(&analysis);

        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        // Should contain custom default values
        assert!(code.contains("42"));
        assert!(code.contains("String") && code.contains("from"));
    }

    #[test]
    fn test_regular_builder_with_skip_setter() {
        let input = parse_quote! {
            struct Example {
                name: String,
                #[builder(skip_setter, default = "Uuid::new_v4()")]
                id: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_regular_builder(&analysis);

        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        // Should contain setter for name but not for id
        assert!(code.contains("pub fn name"));
        assert!(!code.contains("pub fn id"));

        // Should contain default value for id
        assert!(code.contains("Uuid") && code.contains("new_v4"));
    }

    #[test]
    fn test_regular_builder_with_generics() {
        let input = parse_quote! {
            struct Example<T: Clone> {
                value: T,
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_regular_builder(&analysis);

        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        // Should handle generics properly
        assert!(code.contains("<"));
        assert!(code.contains("T"));

        // Should contain PhantomData for generic tracking
        assert!(code.contains("PhantomData"));
    }

    #[test]
    fn test_regular_builder_with_custom_build_method() {
        let input = parse_quote! {
            #[builder(build_method = "create")]
            struct Example {
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_regular_builder(&analysis);

        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        // Should use custom build method name
        assert!(code.contains("pub fn create"));
        // Should not contain the default build method (check for build method ending)
        assert!(!code.contains("pub fn build("));
    }

    #[test]
    fn test_coordinator_creation() {
        let input = parse_quote! {
            struct Example {
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let token_generator = TokenGenerator::new(&analysis);
        let coordinator = RegularBuilderCoordinator::new(&token_generator);

        assert_eq!(coordinator.builder_name, "ExampleBuilder");
    }

    #[test]
    fn test_regular_builder_empty_struct() {
        // Note: This should actually fail in validation, but let's test the generation path
        let input = parse_quote! {
            struct Empty {
                // This will have a PhantomData field added if needed
            }
        };

        // This would normally fail during analysis/validation due to no fields
        // but if it somehow got through, the generator should handle it gracefully
        if let Ok(analysis) = analyze_struct(&input) {
            let result = generate_regular_builder(&analysis);
            // Should either succeed with minimal builder or fail gracefully
            if result.is_ok() {
                let code = result.unwrap().to_string();
                assert!(code.contains("struct"));
            }
        }
    }
}
