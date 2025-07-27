//! Struct Analysis Implementation
//!
//! This module contains the core `StructAnalysis` type and its methods, which
//! represent a complete analysis of a struct for builder generation. The analysis
//! includes field categorization, generic parameter tracking, and all necessary
//! information for generating type-safe builders.
//!
//! # Analysis Process
//!
//! The analysis process extracts and organizes:
//! 1. **Basic Information** - Struct name, generics, attributes
//! 2. **Field Classification** - Required vs optional fields
//! 3. **Generic Tracking** - Which generics are actually used
//! 4. **Validation Context** - Information needed for consistency checks
//!

use crate::analysis::field_analysis::FieldInfo;
use crate::attributes::{parse_struct_attributes, StructAttributes};
use crate::utils::generics::needs_phantom_data;
use quote::quote;
use syn::{DeriveInput, Fields, Generics, Type};

/// Complete analysis of a struct for builder generation.
///
/// This struct contains all the information needed to generate a type-safe
/// builder for a given struct definition. It includes categorized field
/// information, generic parameter tracking, and validation context.
///
/// # Design Principles
///
/// The `StructAnalysis` follows these principles:
/// - **Immutable after creation** - Analysis is performed once and cached
/// - **Complete information** - Contains everything needed for generation
/// - **Validated data** - All inconsistencies caught during creation
/// - **Efficient access** - Methods provide O(1) access to common information
///
/// # Lifetime and Generic Tracking
///
/// The analysis tracks both declared and used generics/lifetimes:
/// - **Declared** - Parameters in the struct signature
/// - **Used** - Parameters actually referenced in field types
/// - **Bounds** - Type bounds and where clause constraints
///
/// This enables generating builders that only include actually-used generic
/// parameters, avoiding "parameter is never used" compiler errors.
#[derive(Debug, Clone)]
pub struct StructAnalysis {
    /// The name of the struct being analyzed
    struct_name: syn::Ident,

    /// The visibility of the struct (pub, pub(crate), private, etc.)
    struct_visibility: syn::Visibility,

    /// The complete generic parameter list from the struct
    struct_generics: Generics,

    /// Parsed struct-level builder attributes
    struct_attributes: StructAttributes,

    /// Fields marked as required (must be set before build())
    required_fields: Vec<FieldInfo>,

    /// Fields that are optional (have default values)
    optional_fields: Vec<FieldInfo>,
}

impl StructAnalysis {
    /// Creates a new struct analysis from a parsed derive input.
    ///
    /// This is the main entry point for creating a `StructAnalysis`. It performs
    /// all the necessary parsing, categorization, and validation to create a
    /// complete analysis context.
    ///
    /// # Arguments
    ///
    /// * `input` - The parsed derive input from the struct definition
    ///
    /// # Returns
    ///
    /// A `Result<StructAnalysis, syn::Error>` containing the complete analysis
    /// or an error if the struct cannot be analyzed.
    ///
    ///
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - Unsupported struct types (enums, unions, tuple structs, unit structs)
    /// - Invalid field or struct attributes
    /// - Inconsistent attribute combinations
    /// - Generic parameter mismatches
    fn from_derive_input(input: &DeriveInput) -> syn::Result<Self> {
        let struct_name = input.ident.clone();
        let struct_visibility = input.vis.clone();
        let struct_generics = input.generics.clone();
        let struct_attributes = parse_struct_attributes(&input.attrs)?;
        let fields = extract_named_fields(input)?;
        let (required_fields, optional_fields) = parse_fields(fields)?;

        Ok(StructAnalysis {
            struct_name,
            struct_visibility,
            struct_generics,
            struct_attributes,
            required_fields,
            optional_fields,
        })
    }

    // Accessors for basic information

    /// Returns the struct name.
    pub fn struct_name(&self) -> &syn::Ident {
        &self.struct_name
    }

    /// Returns the struct's visibility (pub, pub(crate), private, etc.).
    pub fn struct_visibility(&self) -> &syn::Visibility {
        &self.struct_visibility
    }

    /// Returns the struct's generic parameters and constraints.
    pub fn struct_generics(&self) -> &Generics {
        &self.struct_generics
    }

    /// Returns the struct's builder attributes.
    pub fn struct_attributes(&self) -> &StructAttributes {
        &self.struct_attributes
    }

    // Field accessors

    /// Returns the required fields that must be set before building.
    pub fn required_fields(&self) -> &[FieldInfo] {
        &self.required_fields
    }

    /// Returns the optional fields that have default values.
    pub fn optional_fields(&self) -> &[FieldInfo] {
        &self.optional_fields
    }

    /// Iterates over all fields (required and optional).
    ///
    /// This provides a convenient way to process all fields without
    /// separating them by type.
    ///
    /// # Returns
    ///
    /// An iterator over all field information.
    ///
    pub fn all_fields(&self) -> impl Iterator<Item = &FieldInfo> {
        self.required_fields
            .iter()
            .chain(self.optional_fields.iter())
    }

    /// Checks if the struct has only optional fields.
    ///
    /// This determines which builder pattern to use:
    /// - Only optional fields → simple builder with immediate build() availability
    /// - Has required fields → type-state builder with compile-time validation
    ///
    /// # Returns
    ///
    /// `true` if all fields are optional, `false` if any fields are required.
    ///
    pub fn has_only_optional_fields(&self) -> bool {
        self.required_fields.is_empty()
    }

    /// Checks if PhantomData is needed for the builder struct.
    ///
    /// PhantomData is needed when the struct has generic parameters or
    /// when field types contain generics/lifetimes that need to be tracked.
    ///
    /// # Returns
    ///
    /// `true` if PhantomData should be included in the builder, `false` otherwise.
    ///
    pub fn needs_phantom_data(&self) -> bool {
        let field_types: Vec<&Type> = self.all_fields().map(|f| f.field_type()).collect();
        needs_phantom_data(&self.struct_generics, field_types.iter().copied())
    }

    // Token generation methods

    /// Generates impl generics tokens for use in impl blocks.
    ///
    /// This includes all generic parameters with their bounds, suitable for
    /// use in the `impl<...>` part of implementation blocks.
    ///
    /// # Returns
    ///
    /// A token stream containing the impl generics.
    ///
    pub fn impl_generics_tokens(&self) -> proc_macro2::TokenStream {
        let (impl_generics, _ty_generics, _where_clause) = self.struct_generics.split_for_impl();
        quote! { #impl_generics }
    }

    /// Generates type generics tokens for use in type references.
    ///
    /// This includes only the generic parameter names without bounds,
    /// suitable for use when referencing the type (e.g., `MyStruct<T, U>`).
    ///
    /// # Returns
    ///
    /// A token stream containing the type generics.
    ///
    pub fn type_generics_tokens(&self) -> proc_macro2::TokenStream {
        let (_impl_generics, ty_generics, _where_clause) = self.struct_generics.split_for_impl();
        quote! { #ty_generics }
    }

    /// Generates where clause tokens for use in type definitions.
    ///
    /// This includes all where clause predicates from the original struct
    /// definition.
    ///
    /// # Returns
    ///
    /// A token stream containing the where clause, or empty if no where clause.
    ///
    pub fn where_clause_tokens(&self) -> proc_macro2::TokenStream {
        let (_impl_generics, _ty_generics, where_clause) = self.struct_generics.split_for_impl();
        quote! { #where_clause }
    }

    // Validation and analysis methods

    /// Validates the struct configuration for builder generation.
    ///
    /// This method performs comprehensive validation to ensure that the
    /// struct can have a builder generated for it and that all attributes
    /// are consistent.
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
    /// - Field attribute combinations are valid
    /// - Required fields don't have conflicting attributes
    /// - Optional fields with skip_setter have default values
    /// - Setter names don't conflict with each other
    /// - Build method name doesn't conflict with setter names
    ///
    /// Note: Generic and lifetime validation is left to the Rust compiler.
    ///
    /// # Errors
    ///
    /// Returns specific errors for:
    /// - Invalid field attribute combinations
    /// - Inconsistent builder configuration
    /// - Name conflicts between methods
    pub fn validate_for_generation(&self) -> syn::Result<()> {
        // Delegate to validation module for comprehensive checks
        crate::analysis::validation::validate_struct_for_generation(self)
    }
}

/// Analyzes a struct definition and creates a complete `StructAnalysis`.
///
/// This is the main entry point for struct analysis. It performs all necessary
/// parsing, validation, and organization to create a comprehensive analysis
/// context for builder generation.
///
/// # Arguments
///
/// * `input` - The parsed derive input from the struct definition
///
/// # Returns
///
/// A `Result<StructAnalysis, syn::Error>` containing the complete analysis
/// or an error if the struct cannot be analyzed.
///
///
///
/// # Errors
///
/// Returns errors for:
/// - Unsupported struct types (enums, unions, tuple structs, unit structs)  
/// - Invalid field or struct attributes
/// - Inconsistent attribute combinations
/// - Generic parameter usage errors
pub fn analyze_struct(input: &DeriveInput) -> syn::Result<StructAnalysis> {
    StructAnalysis::from_derive_input(input)
}

// Helper functions (these were previously standalone functions)

/// Extracts named fields from a struct definition.
///
/// This function validates that the struct has named fields (not tuple or unit)
/// and returns a reference to the field list.
///
/// # Arguments
///
/// * `input` - The derive input to extract fields from
///
/// # Returns
///
/// A `Result<&syn::FieldsNamed, syn::Error>` containing the named fields
/// or an error for unsupported struct types.
///
/// # Errors
///
/// Returns errors for:
/// - Tuple structs (only named fields supported)
/// - Unit structs (no fields to build)
/// - Enums (not supported)
/// - Unions (not supported)
fn extract_named_fields(input: &DeriveInput) -> syn::Result<&syn::FieldsNamed> {
    match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => Ok(fields_named),
            Fields::Unnamed(_) => Err(syn::Error::new_spanned(
                input,
                "TypeStateBuilder only supports structs with named fields",
            )),
            Fields::Unit => Err(syn::Error::new_spanned(
                input,
                "TypeStateBuilder does not support unit structs",
            )),
        },
        syn::Data::Enum(_) => Err(syn::Error::new_spanned(
            input,
            "TypeStateBuilder only supports structs, not enums",
        )),
        syn::Data::Union(_) => Err(syn::Error::new_spanned(
            input,
            "TypeStateBuilder only supports structs, not unions",
        )),
    }
}

/// Parses fields and separates them into required and optional categories.
///
/// This function processes all fields in the struct, parsing their attributes
/// and categorizing them based on whether they are required or optional for
/// the builder pattern.
///
/// # Arguments
///
/// * `fields_named` - The named fields from the struct definition
///
/// # Returns
///
/// A `Result<(Vec<FieldInfo>, Vec<FieldInfo>), syn::Error>` containing
/// required fields and optional fields, or an error for invalid field
/// configurations.
///
/// # Errors
///
/// Returns errors for:
/// - Invalid field attributes
/// - Inconsistent attribute combinations
/// - Missing field names
fn parse_fields(fields_named: &syn::FieldsNamed) -> syn::Result<(Vec<FieldInfo>, Vec<FieldInfo>)> {
    let mut required_fields = Vec::new();
    let mut optional_fields = Vec::new();

    for field in &fields_named.named {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Field must have a name"))?
            .clone();

        let field_info = FieldInfo::from_syn_field(field_name, field.ty.clone(), &field.attrs)?;

        if field_info.is_required() {
            required_fields.push(field_info);
        } else {
            optional_fields.push(field_info);
        }
    }

    Ok((required_fields, optional_fields))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use syn::parse_quote;

    #[test]
    fn test_analyze_simple_struct() {
        let input: DeriveInput = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
                age: Option<u32>,
            }
        };

        let analysis = analyze_struct(&input).unwrap();

        assert_eq!(analysis.struct_name().to_string(), "Example");
        assert_eq!(analysis.required_fields().len(), 1);
        assert_eq!(analysis.optional_fields().len(), 1);
        assert!(analysis.struct_generics().params.is_empty());
        assert!(!analysis.has_only_optional_fields());
    }

    #[test]
    fn test_analyze_generic_struct() {
        let input: DeriveInput = parse_quote! {
            struct Example<T, U> {
                value: T,
                #[builder(required)]
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();

        assert!(!analysis.struct_generics().params.is_empty());
        assert_eq!(analysis.struct_generics().params.len(), 2);
        // Test that the struct has a generic field (value: T)
        assert!(analysis
            .all_fields()
            .any(|f| f.field_type().to_token_stream().to_string() == "T"));
    }

    #[test]
    fn test_analyze_struct_with_lifetimes() {
        let input: DeriveInput = parse_quote! {
            struct Example<'a, 'b> {
                text: &'a str,
                #[builder(required)]
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();

        assert!(!analysis.struct_generics().params.is_empty());
        // Test that the struct has a field with lifetime (text: &'a str)
        assert!(analysis.all_fields().any(|f| f
            .field_type()
            .to_token_stream()
            .to_string()
            .contains("'a")));
    }

    #[test]
    fn test_analyze_struct_with_custom_build_method() {
        let input: DeriveInput = parse_quote! {
            #[builder(build_method = "create")]
            struct Example {
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();

        assert_eq!(
            analysis.struct_attributes().get_build_method_name(),
            "create"
        );
    }

    #[test]
    fn test_analyze_all_optional_fields() {
        let input: DeriveInput = parse_quote! {
            struct Example {
                name: Option<String>,
                age: u32,
            }
        };

        let analysis = analyze_struct(&input).unwrap();

        assert!(analysis.has_only_optional_fields());
        assert_eq!(analysis.required_fields().len(), 0);
        assert_eq!(analysis.optional_fields().len(), 2);
    }

    #[test]
    fn test_extract_named_fields_errors() {
        // Test tuple struct
        let tuple_input: DeriveInput = parse_quote!(
            struct Example(String, i32);
        );
        assert!(extract_named_fields(&tuple_input).is_err());

        // Test unit struct
        let unit_input: DeriveInput = parse_quote!(
            struct Example;
        );
        assert!(extract_named_fields(&unit_input).is_err());

        // Test enum
        let enum_input: DeriveInput = parse_quote!(
            enum Example {
                A,
                B,
            }
        );
        assert!(extract_named_fields(&enum_input).is_err());
    }

    #[test]
    fn test_token_generation() {
        let input: DeriveInput = parse_quote! {
            struct Example<T: Clone>
            where
                T: Send
            {
                value: T,
            }
        };

        let analysis = analyze_struct(&input).unwrap();

        let impl_generics = analysis.impl_generics_tokens();
        let type_generics = analysis.type_generics_tokens();
        let where_clause = analysis.where_clause_tokens();

        // These should contain the appropriate generic information
        assert!(!impl_generics.is_empty());
        assert!(!type_generics.is_empty());
        assert!(!where_clause.is_empty());
    }
}
