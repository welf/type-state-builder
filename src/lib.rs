//! Type-State Builder Pattern Implementation
//!
//! This crate provides a derive macro for implementing the Type-State Builder pattern
//! in Rust, enabling compile-time validation of required fields and zero-cost builder
//! abstractions.
//!
//! # Overview
//!
//! The Type-State Builder pattern uses Rust's type system to enforce compile-time
//! validation of required fields. It automatically selects between two builder patterns:
//!
//! - **Type-State Builder**: For structs with required fields, providing compile-time
//!   safety that prevents calling `build()` until all required fields are set
//! - **Regular Builder**: For structs with only optional fields, providing a simple
//!   builder with immediate `build()` availability
//!
//! # Key Features
//!
//! - **Zero Runtime Cost**: All validation happens at compile time
//! - **Automatic Pattern Selection**: Chooses the best builder pattern for your struct
//! - **Comprehensive Generic Support**: Handles complex generic types and lifetimes
//! - **Flexible Configuration**: Extensive attribute-based customization
//! - **Excellent Error Messages**: Clear guidance for fixing configuration issues
//!
//! # Quick Start
//!
//! Add the derive macro to your struct and mark required fields:
//!
//! ```
//! use type_state_builder::TypeStateBuilder;
//!
//! #[derive(TypeStateBuilder)]
//! struct User {
//!     #[builder(required)]
//!     name: String,
//!     
//!     #[builder(required)]
//!     email: String,
//!     
//!     age: Option<u32>,
//!     active: bool, // Will use Default::default()
//! }
//!
//! // Usage - this enforces that name and email are set
//! let user = User::builder()
//!     .name("Alice".to_string())
//!     .email("alice@example.com".to_string())
//!     .age(Some(30))
//!     .build();
//! ```
//!
//! # Supported Attributes
//!
//! ## Struct-level Attributes
//!
//! - `#[builder(build_method = "method_name")]` - Custom build method name
//! - `#[builder(setter_prefix = "prefix_")]` - Prefix for all setter method names
//!
//! ## Field-level Attributes
//!
//! - `#[builder(required)]` - Mark field as required
//! - `#[builder(setter_name = "name")]` - Custom setter method name
//! - `#[builder(setter_prefix = "prefix_")]` - Custom prefix for setter method name
//! - `#[builder(default = "expr")]` - Custom default value expression
//! - `#[builder(skip_setter)]` - Don't generate setter (requires default)
//!
//! # Advanced Examples
//!
//! ## Custom Defaults and Setter Names
//!
//! ```
//! use type_state_builder::TypeStateBuilder;
//!
//! #[derive(TypeStateBuilder)]
//! #[builder(build_method = "create")]
//! struct Document {
//!     #[builder(required)]
//!     title: String,
//!     
//!     #[builder(required, setter_name = "set_content")]
//!     content: String,
//!     
//!     #[builder(default = "42")]
//!     page_count: u32,
//!     
//!     #[builder(default = "String::from(\"draft\")", skip_setter)]
//!     status: String,
//! }
//!
//! let doc = Document::builder()
//!     .title("My Document".to_string())
//!     .set_content("Document content here".to_string())
//!     .create(); // Custom build method name
//! ```
//!
//! ## Generic Types and Lifetimes
//!
//! ```
//! use type_state_builder::TypeStateBuilder;
//!
//! #[derive(TypeStateBuilder)]
//! struct Container<T: Clone>
//! where
//!     T: Send
//! {
//!     #[builder(required)]
//!     value: T,
//!     
//!     #[builder(required)]
//!     name: String,
//!     
//!     tags: Vec<String>,
//! }
//!
//! let container = Container::builder()
//!     .value(42)
//!     .name("test".to_string())
//!     .tags(vec!["tag1".to_string()])
//!     .build();
//! ```
//!
//! ## Setter Prefix Examples
//!
//! ```
//! use type_state_builder::TypeStateBuilder;
//!
//! // Struct-level setter prefix applies to all fields
//! #[derive(TypeStateBuilder)]
//! #[builder(setter_prefix = "with_")]
//! struct Config {
//!     #[builder(required)]
//!     host: String,
//!     
//!     #[builder(required)]
//!     port: u16,
//!     
//!     timeout: Option<u32>,
//! }
//!
//! let config = Config::builder()
//!     .with_host("localhost".to_string())
//!     .with_port(8080)
//!     .with_timeout(Some(30))
//!     .build();
//! ```
//!
//! ```
//! use type_state_builder::TypeStateBuilder;
//!
//! // Field-level setter prefix overrides struct-level prefix
//! #[derive(TypeStateBuilder)]
//! #[builder(setter_prefix = "with_")]
//! struct Database {
//!     #[builder(required)]
//!     connection_string: String,
//!     
//!     #[builder(required, setter_prefix = "set_")]
//!     credentials: String,
//!     
//!     #[builder(setter_name = "timeout_seconds")]
//!     timeout: Option<u32>,
//! }
//!
//! let db = Database::builder()
//!     .with_connection_string("postgresql://...".to_string())
//!     .set_credentials("user:pass".to_string())
//!     .with_timeout_seconds(Some(60))
//!     .build();
//! ```
//!
//! ## Optional-Only Structs (Regular Builder)
//!
//! ```
//! use type_state_builder::TypeStateBuilder;
//!
//! // No required fields = regular builder pattern
//! #[derive(TypeStateBuilder)]
//! struct Settings {
//!     debug: bool,
//!     max_connections: Option<u32>,
//!     
//!     #[builder(default = "\"default.log\".to_string()")]
//!     log_file: String,
//!     
//!     #[builder(skip_setter, default = "42")]
//!     magic_number: i32,
//! }
//!
//! // Can call build() immediately since no required fields
//! let settings = Settings::builder()
//!     .debug(true)
//!     .max_connections(Some(100))
//!     .build();
//! ```
//!
//! # Error Prevention
//!
//! The macro prevents common mistakes at compile time:
//!
//! ```compile_fail
//! use type_state_builder::TypeStateBuilder;
//!
//! #[derive(TypeStateBuilder)]
//! struct User {
//!     #[builder(required)]
//!     name: String,
//! }
//!
//! let user = User::builder().build(); // ERROR: required field not set
//! ```
//!
//! ```compile_fail
//! use type_state_builder::TypeStateBuilder;
//!
//! #[derive(TypeStateBuilder)]
//! struct BadConfig {
//!     #[builder(required, default = "test")]  // ERROR: Invalid combination
//!     name: String,
//! }
//! ```
//!
//! # Module Organization
//!
//! The crate is organized into several modules that handle different aspects
//! of the builder generation process. Most users will only interact with the
//! [`TypeStateBuilder`] derive macro.
//!
//! For complete documentation, examples, and guides, see the
//! [README](https://github.com/welf/type-state-builder#readme).

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// Internal modules - not exported due to proc-macro restrictions
mod analysis;
mod attributes;
mod generation;
mod utils;
mod validation;

/// Derives a type-safe builder for a struct with compile-time validation of required fields.
///
/// This macro automatically generates an appropriate builder pattern based on the struct
/// configuration:
/// - **Type-State Builder** for structs with required fields
/// - **Regular Builder** for structs with only optional fields
///
/// # Basic Usage
///
/// ```
/// use type_state_builder::TypeStateBuilder;
///
/// #[derive(TypeStateBuilder)]
/// struct Person {
///     #[builder(required)]
///     name: String,
///     age: Option<u32>,
/// }
///
/// let person = Person::builder()
///     .name("Alice".to_string())
///     .build();
/// ```
///
/// # Attribute Reference
///
/// ## Struct Attributes
///
/// - `#[builder(build_method = "name")]` - Custom build method name (default: "build")
/// - `#[builder(setter_prefix = "prefix_")]` - Prefix for all setter method names
///
/// ## Field Attributes
///
/// - `#[builder(required)]` - Field must be set before build() (creates type-state builder)
/// - `#[builder(setter_name = "name")]` - Custom setter method name
/// - `#[builder(setter_prefix = "prefix_")]` - Custom prefix for this field's setter (overrides struct-level)
/// - `#[builder(default = "expr")]` - Custom default value (must be valid Rust expression)
/// - `#[builder(skip_setter)]` - Don't generate setter method (requires default value)
///
/// # Generated Methods
///
/// The macro generates:
/// - `YourStruct::builder()` - Creates a new builder instance
/// - `.field_name(value)` - Setter methods for each field (unless skipped)
/// - `.build()` - Constructs the final instance (or custom name from `build_method`)
///
/// # Compile-Time Safety
///
/// The type-state builder (used when there are required fields) prevents:
/// - Calling `build()` before setting required fields
/// - Setting the same required field multiple times
/// - Invalid attribute combinations
///
/// # Error Messages
///
/// The macro provides clear error messages for common mistakes:
/// - Missing required fields at build time
/// - Invalid attribute combinations
/// - Generic parameter mismatches
/// - Syntax errors in default values
///
/// # Examples
///
/// ```
/// use type_state_builder::TypeStateBuilder;
///
/// #[derive(TypeStateBuilder)]
/// struct User {
///     #[builder(required)]
///     name: String,
///     age: Option<u32>,
/// }
///
/// let user = User::builder()
///     .name("Alice".to_string())
///     .build();
/// ```
#[proc_macro_derive(TypeStateBuilder, attributes(builder))]
pub fn derive_type_state_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_builder_implementation(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generates the complete builder implementation for a struct.
///
/// This is the main implementation function that coordinates the analysis,
/// validation, and generation process.
///
/// # Arguments
///
/// * `input` - The parsed derive input from the struct definition
///
/// # Returns
///
/// A `syn::Result<proc_macro2::TokenStream>` containing the complete builder
/// implementation or detailed error information.
///
/// # Process
///
/// 1. **Analysis** - Parse and analyze the struct definition
/// 2. **Validation** - Ensure the configuration is valid
/// 3. **Generation** - Create the appropriate builder code
/// 4. **Assembly** - Combine all components into the final output
///
/// # Error Handling
///
/// Errors are returned with detailed information about:
/// - What went wrong during analysis or generation
/// - How to fix configuration issues
/// - Examples of correct usage
fn generate_builder_implementation(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    // Step 1: Analyze the struct definition
    let analysis = analysis::analyze_struct(input)?;

    // Step 2: Validate the analysis for builder generation
    analysis.validate_for_generation()?;

    // Step 3: Generate the appropriate builder implementation
    generation::generate_builder(&analysis)
}

// Internal types for testing - not exported due to proc-macro restrictions

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_derive_macro_with_required_fields() {
        let input: DeriveInput = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
                age: Option<u32>,
            }
        };

        let result = generate_builder_implementation(&input);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let code = tokens.to_string();

        // Should contain builder method
        assert!(code.contains("builder"));
        // Should contain setter for required field
        assert!(code.contains("name"));
        // Should contain build method
        assert!(code.contains("build"));
    }

    #[test]
    fn test_derive_macro_with_optional_only() {
        let input: DeriveInput = parse_quote! {
            struct Example {
                name: Option<String>,
                age: u32,
            }
        };

        let result = generate_builder_implementation(&input);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let code = tokens.to_string();

        // Should generate regular builder for optional-only struct
        assert!(code.contains("builder"));
        assert!(code.contains("build"));
    }

    #[test]
    fn test_derive_macro_with_custom_attributes() {
        let input: DeriveInput = parse_quote! {
            #[builder(build_method = "create")]
            struct Example {
                #[builder(required, setter_name = "set_name")]
                name: String,

                #[builder(default = "42")]
                count: i32,
            }
        };

        let result = generate_builder_implementation(&input);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let code = tokens.to_string();

        // Should respect custom build method name
        assert!(code.contains("create"));
        // Should respect custom setter name
        assert!(code.contains("set_name"));
    }

    #[test]
    fn test_derive_macro_with_generics() {
        let input: DeriveInput = parse_quote! {
            struct Example<T: Clone> {
                #[builder(required)]
                value: T,
                name: String,
            }
        };

        let result = generate_builder_implementation(&input);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let code = tokens.to_string();

        // Should handle generics properly
        assert!(code.contains("<"));
        assert!(code.contains("T"));
    }

    #[test]
    fn test_derive_macro_with_undeclared_generic() {
        // This test verifies that undeclared generics are allowed through our validation
        // and the Rust compiler will catch the error later
        let input: DeriveInput = parse_quote! {
            struct Example {
                value: T,  // T is not declared - Rust compiler will catch this
            }
        };

        let result = generate_builder_implementation(&input);
        // Should succeed in our validation (Rust compiler will catch undeclared generic later)
        assert!(result.is_ok());

        let code = result.unwrap().to_string();
        // Should generate code that includes the undeclared generic
        assert!(code.contains("value"));
    }

    #[test]
    fn test_unsupported_input_types() {
        // Test enum - should fail
        let enum_input: DeriveInput = parse_quote! {
            enum Example {
                A, B
            }
        };
        let result = generate_builder_implementation(&enum_input);
        assert!(result.is_err());

        // Test tuple struct - should fail
        let tuple_input: DeriveInput = parse_quote! {
            struct Example(String, i32);
        };
        let result = generate_builder_implementation(&tuple_input);
        assert!(result.is_err());

        // Test unit struct - should fail
        let unit_input: DeriveInput = parse_quote! {
            struct Example;
        };
        let result = generate_builder_implementation(&unit_input);
        assert!(result.is_err());
    }
}
