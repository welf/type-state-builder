//! Type-State Builder Generation with Advanced State Machine Logic
//!
//! This module implements the **type-state builder pattern** generation for structs
//! with required fields. The type-state pattern uses Rust's type system to create
//! a compile-time state machine that prevents invalid builder usage.
//!
//! # The Type-State Revolution
//!
//! Traditional builders allow dangerous code where you can call build() before setting
//! required fields, leading to runtime panics. Type-state builders make such code
//! impossible to compile by using the type system to track which fields have been set.
//!
//! # Core State Machine Theory
//!
//! The type-state builder creates a **finite state automaton** where:
//!
//! ## State Space
//! For N required fields, we generate **2^N states** representing all possible
//! combinations of set/unset required fields:
//!
//! For a struct with N required fields, we generate 2^N states representing all possible
//! combinations of set/unset required fields. For example, with 2 required fields (name and email),
//! we get 4 states: neither field set, only email set, only name set, and both fields set.
//! Each state corresponds to a distinct builder type name that clearly indicates completion status.
//!
//! ## State Transitions
//! Each setter method creates a deterministic transition from one state to another.
//! For example, calling the name() setter transitions from the initial state to a state
//! where the name field is set. The transitions form a directed graph toward completion.
//!
//! ## Terminal State
//! Only the final state (where all required fields are set) has a build() method.
//! This ensures that the struct can only be constructed when all required data is available.
//!
//! ## Key Guarantees
//! 1. **Impossibility of Invalid States** - Cannot call `build()` until all required fields are set
//! 2. **Progress Monotonicity** - Can only move toward completion (no "unsetting")
//! 3. **Zero Runtime Cost** - All validation is compile-time only
//! 4. **Perfect Ergonomics** - Method chaining works seamlessly
//!
//! # Generated Code Structure
//!
//! For a struct with required fields, this generates:
//! - Multiple concrete builder types for different states
//! - Constructor method on the original struct
//! - Setter methods that transition between states
//! - Build method only available in the final state
//! - PhantomData handling for generic parameters
//!

use crate::analysis::StructAnalysis;
use crate::generation::TokenGenerator;
use crate::utils::identifiers::{capitalize_first_letter, strip_raw_identifier_prefix};
use quote::quote;
use syn::Ident;

/// Generates a complete type-state builder implementation.
///
/// This is the main entry point for type-state builder generation. It creates
/// all the necessary components for a type-state builder including multiple
/// builder types, state transitions, and compile-time validation.
///
/// # Arguments
///
/// * `analysis` - Complete struct analysis containing all necessary information
///
/// # Returns
///
/// A `syn::Result<proc_macro2::TokenStream>` containing the complete type-state
/// builder implementation or an error if generation fails.
///
/// # Generated Components
///
/// The function generates:
/// 1. **Concrete Builder Types** - Different types for different completion states
/// 2. **Constructor Method** - `YourStruct::builder()` method
/// 3. **Setter Methods** - Methods that transition between builder states
/// 4. **Build Method** - Final method to construct the target struct
/// 5. **Generic Handling** - Proper PhantomData and generic parameter tracking
///
pub fn generate_type_state_builder(
    analysis: &StructAnalysis,
) -> syn::Result<proc_macro2::TokenStream> {
    let token_generator = TokenGenerator::new(analysis);
    generate_with_token_generator(&token_generator)
}

/// Generates a type-state builder using a specific token generator.
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
    let builder_coordinator = TypeStateBuilderCoordinator::new(token_generator);
    builder_coordinator.generate_complete_implementation()
}

/// Coordinator for type-state builder generation.
///
/// This struct encapsulates the logic for generating all components of a
/// type-state builder, providing methods for creating different aspects
/// of the implementation.
struct TypeStateBuilderCoordinator<'a> {
    /// Token generator for consistent code generation
    token_generator: &'a TokenGenerator<'a>,

    /// Cached state combinations for efficient generation
    state_combinations: Vec<StateCombination>,

    /// Base name for builder types
    _base_builder_name: String,
}

impl<'a> TypeStateBuilderCoordinator<'a> {
    /// Creates a new coordinator with the given token generator.
    ///
    /// # Arguments
    ///
    /// * `token_generator` - Token generator to use for code generation
    ///
    /// # Returns
    ///
    /// A new `TypeStateBuilderCoordinator` ready for generation.
    fn new(token_generator: &'a TokenGenerator<'a>) -> Self {
        let struct_name = token_generator.analysis().struct_name();
        let base_builder_name = format!("{struct_name}Builder");

        let state_combinations =
            Self::generate_state_combinations(token_generator.analysis(), &base_builder_name);

        Self {
            token_generator,
            state_combinations,
            _base_builder_name: base_builder_name,
        }
    }

    /// Generates the complete type-state builder implementation.
    ///
    /// This orchestrates the generation of all components needed for a
    /// functional type-state builder.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the complete implementation.
    fn generate_complete_implementation(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut tokens = proc_macro2::TokenStream::new();

        // Generate all concrete builder types
        tokens.extend(self.generate_concrete_builder_types()?);

        // Generate the constructor method on the original struct
        tokens.extend(self.generate_struct_constructor_method()?);

        // Generate setter methods for all states and fields
        tokens.extend(self.generate_all_setter_methods()?);

        // Generate build methods for all states (complete and incomplete)
        tokens.extend(self.generate_all_build_methods()?);

        Ok(tokens)
    }

    /// Generates all concrete builder type definitions.
    ///
    /// Creates struct definitions for each possible state combination of
    /// required fields.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing all builder type definitions.
    fn generate_concrete_builder_types(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut tokens = proc_macro2::TokenStream::new();

        for state_combination in &self.state_combinations {
            tokens.extend(self.generate_single_builder_type(state_combination)?);
        }

        Ok(tokens)
    }

    /// Generates a single concrete builder type.
    ///
    /// # Arguments
    ///
    /// * `state_combination` - The state combination this builder type represents
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the builder type definition.
    fn generate_single_builder_type(
        &self,
        state_combination: &StateCombination,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let builder_ident = syn::parse_str::<Ident>(&state_combination.concrete_type_name)?;
        let analysis = self.token_generator.analysis();

        // Generate generic parameters for the builder type
        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        // Generate field declarations for this builder state
        let field_declarations = self.generate_builder_fields(state_combination)?;

        // Generate documentation for this builder type
        let doc = self.token_generator.generate_method_documentation(
            &state_combination.concrete_type_name,
            &format!(
                "Builder for {} with {} of {} required fields set",
                analysis.struct_name(),
                state_combination.set_fields.len(),
                analysis.required_fields().len()
            ),
            Some(
                "This builder type represents a specific state in the type-state building process.",
            ),
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

    /// Generates field declarations for a builder in a specific state.
    ///
    /// Fields are either stored as their actual type (if set in this state)
    /// or as Option<Type> (if not yet set).
    ///
    /// # Arguments
    ///
    /// * `state_combination` - The state this builder represents
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing field declarations.
    fn generate_builder_fields(
        &self,
        state_combination: &StateCombination,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let mut field_tokens = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Generate required fields (either as Type or Option<Type>)
        for (field_index, required_field) in analysis.required_fields().iter().enumerate() {
            let field_name = required_field.name();
            let field_type = required_field.field_type();
            let is_set = state_combination.set_fields.contains(&field_index);

            let doc = self.token_generator.generate_field_documentation(
                &required_field.clean_name(),
                &quote! { #field_type }.to_string(),
                true,
                if is_set {
                    "Set required"
                } else {
                    "Unset required"
                },
            );

            if is_set {
                // Field is set in this state - store actual value
                field_tokens.extend(quote! {
                    #doc
                    #field_name: #field_type,
                });
            } else {
                // Field is not set yet - store as Option
                let option_type = self.token_generator.generate_type_path("Option");
                field_tokens.extend(quote! {
                    #doc
                    #field_name: #option_type<#field_type>,
                });
            }
        }

        // Generate optional fields (always as their actual type)
        for optional_field in analysis.optional_fields() {
            let field_name = optional_field.name();
            let field_type = optional_field.field_type();

            let doc = self.token_generator.generate_field_documentation(
                &optional_field.clean_name(),
                &quote! { #field_type }.to_string(),
                false,
                "Optional",
            );

            field_tokens.extend(quote! {
                #doc
                #field_name: #field_type,
            });
        }

        // Add PhantomData field if needed
        field_tokens.extend(self.token_generator.generate_phantom_data_field());

        Ok(field_tokens)
    }

    /// Generates the constructor method on the original struct.
    ///
    /// This creates the `YourStruct::builder()` method that returns the
    /// initial builder state.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the constructor method.
    fn generate_struct_constructor_method(&self) -> syn::Result<proc_macro2::TokenStream> {
        let analysis = self.token_generator.analysis();
        let struct_name = analysis.struct_name();

        // Get the initial state (no fields set)
        let initial_state = self
            .state_combinations
            .iter()
            .find(|combo| combo.set_fields.is_empty())
            .ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "No initial state found in state combinations",
                )
            })?;

        let initial_builder_ident = syn::parse_str::<Ident>(&initial_state.concrete_type_name)?;

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        let doc = self.token_generator.generate_method_documentation(
            "builder",
            "Creates a new type-safe builder for constructing an instance",
            Some("This builder uses the type-state pattern to ensure all required fields are set before building.")
        );

        Ok(quote! {
            impl #impl_generics #struct_name #type_generics #where_clause {
                #doc
                pub fn builder() -> #initial_builder_ident #type_generics {
                    #initial_builder_ident::new()
                }
            }
        })
    }

    /// Generates all setter methods for all builder states.
    ///
    /// This creates setter methods that transition between builder states
    /// as required fields are set.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing all setter methods.
    fn generate_all_setter_methods(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut tokens = proc_macro2::TokenStream::new();

        // Generate constructor method for initial state
        tokens.extend(self.generate_initial_constructor_method()?);

        // Generate setter methods for required fields (with state transitions)
        tokens.extend(self.generate_required_field_setters()?);

        // Generate setter methods for optional fields (no state transitions)
        tokens.extend(self.generate_optional_field_setters()?);

        Ok(tokens)
    }

    /// Generates the constructor method for the initial builder state.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the constructor implementation.
    fn generate_initial_constructor_method(&self) -> syn::Result<proc_macro2::TokenStream> {
        let initial_state = self
            .state_combinations
            .iter()
            .find(|combo| combo.set_fields.is_empty())
            .ok_or_else(|| {
                syn::Error::new(proc_macro2::Span::call_site(), "No initial state found")
            })?;

        let initial_builder_ident = syn::parse_str::<Ident>(&initial_state.concrete_type_name)?;

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        // Generate field initializations
        let field_init = self.generate_initial_field_initializations()?;

        let doc = self.token_generator.generate_method_documentation(
            "new",
            "Creates a new builder with all required fields unset and optional fields at default values",
            None
        );

        Ok(quote! {
            impl #impl_generics #initial_builder_ident #type_generics #where_clause {
                #doc
                pub fn new() -> Self {
                    Self {
                        #field_init
                    }
                }
            }
        })
    }

    /// Generates initial field initialization code for the initial builder state.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing field initialization code.
    fn generate_initial_field_initializations(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut field_init = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Initialize required fields as None
        for required_field in analysis.required_fields() {
            let field_name = required_field.name();
            let none_option = self.token_generator.generate_type_path("Option");
            field_init.extend(quote! {
                #field_name: #none_option::None,
            });
        }

        // Initialize optional fields with defaults
        for optional_field in analysis.optional_fields() {
            let field_init_code = optional_field.generate_initialization(false)?;
            field_init.extend(field_init_code);
        }

        // Initialize PhantomData if needed
        field_init.extend(self.token_generator.generate_phantom_data_init());

        Ok(field_init)
    }

    /// Generates setter methods for required fields with state transitions.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing required field setters.
    fn generate_required_field_setters(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut tokens = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // For each state combination, generate setters for unset required fields
        for state_combination in &self.state_combinations {
            for (field_index, required_field) in analysis.required_fields().iter().enumerate() {
                // Only generate setter if this field is not set in this state
                if !state_combination.set_fields.contains(&field_index) {
                    tokens.extend(self.generate_required_field_setter(
                        required_field,
                        field_index,
                        state_combination,
                    )?);
                }
            }
        }

        Ok(tokens)
    }

    /// Generates a single required field setter method.
    ///
    /// # Arguments
    ///
    /// * `field` - The field to generate a setter for
    /// * `field_index` - Index of the field in the required fields list
    /// * `current_state` - The current builder state
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the setter method.
    fn generate_required_field_setter(
        &self,
        field: &crate::analysis::FieldInfo,
        field_index: usize,
        current_state: &StateCombination,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let input_builder_ident = syn::parse_str::<Ident>(&current_state.concrete_type_name)?;
        let field_type = field.field_type();

        // Determine the output state (current state + this field set)
        let output_state =
            self.find_state_with_field_set(field_index, &current_state.set_fields)?;
        let output_builder_ident = syn::parse_str::<Ident>(&output_state.concrete_type_name)?;

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        // Generate setter name with prefix support
        let struct_setter_prefix = self
            .token_generator
            .analysis()
            .struct_attributes()
            .get_setter_prefix();
        let setter_config = field.create_setter_config(struct_setter_prefix);
        let setter_ident = syn::parse_str::<Ident>(&setter_config.setter_name)?;

        let doc = self.token_generator.generate_method_documentation(
            &setter_config.setter_name,
            &{
                let field_name = field.clean_name();
                format!("Sets the required field `{field_name}`")
            },
            Some("This method transitions the builder to a new state where this field is set."),
        );

        // Generate field assignments for the transition
        let field_assignments = self.generate_field_assignments_for_transition(field_index)?;

        Ok(quote! {
            impl #impl_generics #input_builder_ident #type_generics #where_clause {
                #doc
                pub fn #setter_ident(self, value: #field_type) -> #output_builder_ident #type_generics {
                    #output_builder_ident {
                        #field_assignments
                    }
                }
            }
        })
    }

    /// Generates field assignments for a state transition.
    ///
    /// # Arguments
    ///
    /// * `setting_field_index` - Index of the field being set
    /// * `current_state` - The current builder state
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing field assignment code.
    fn generate_field_assignments_for_transition(
        &self,
        setting_field_index: usize,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let mut assignments = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Handle required fields
        for (field_index, required_field) in analysis.required_fields().iter().enumerate() {
            let field_name = required_field.name();

            if field_index == setting_field_index {
                // This is the field being set
                assignments.extend(quote! {
                    #field_name: value,
                });
            } else {
                // Copy from existing state
                assignments.extend(quote! {
                    #field_name: self.#field_name,
                });
            }
        }

        // Copy all optional fields
        for optional_field in analysis.optional_fields() {
            let field_name = optional_field.name();
            assignments.extend(quote! {
                #field_name: self.#field_name,
            });
        }

        // Copy PhantomData if present
        if analysis.needs_phantom_data() {
            let marker_name = self.token_generator.get_phantom_data_field_name();
            let marker_ident = syn::parse_str::<Ident>(marker_name)?;
            assignments.extend(quote! {
                #marker_ident: self.#marker_ident,
            });
        }

        Ok(assignments)
    }

    /// Generates setter methods for optional fields.
    ///
    /// Optional field setters don't cause state transitions - they work
    /// the same way in all builder states.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing optional field setters.
    fn generate_optional_field_setters(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut tokens = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Generate setters for each builder state
        for state_combination in &self.state_combinations {
            let builder_ident = syn::parse_str::<Ident>(&state_combination.concrete_type_name)?;
            let mut setter_methods = proc_macro2::TokenStream::new();

            // Generate setter for each optional field
            let struct_setter_prefix = analysis.struct_attributes().get_setter_prefix();
            for optional_field in analysis.optional_fields() {
                if optional_field.should_generate_setter() {
                    let setter_method = optional_field
                        .generate_setter_method(&syn::parse_quote!(Self), struct_setter_prefix)?;
                    setter_methods.extend(setter_method);
                }
            }

            if !setter_methods.is_empty() {
                let impl_generics = self.token_generator.impl_generics_tokens();
                let type_generics = self.token_generator.type_generics_tokens();
                let where_clause = self.token_generator.where_clause_tokens();

                tokens.extend(quote! {
                    impl #impl_generics #builder_ident #type_generics #where_clause {
                        #setter_methods
                    }
                });
            }
        }

        Ok(tokens)
    }

    /// Generates the build method for the final builder state.
    ///
    /// The build method is only available when all required fields have been set.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing the build method.
    /// Generates field assignments for the final struct construction.
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing struct field assignments.
    fn generate_final_struct_assignments(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut assignments = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();

        // Assign required fields (stored as actual values in final state)
        for required_field in analysis.required_fields() {
            let field_name = required_field.name();
            assignments.extend(quote! {
                #field_name: self.#field_name,
            });
        }

        // Assign optional fields
        for optional_field in analysis.optional_fields() {
            let field_name = optional_field.name();
            assignments.extend(quote! {
                #field_name: self.#field_name,
            });
        }

        Ok(assignments)
    }

    /// Generates build methods for all builder states.
    ///
    /// This method creates build methods for every builder state:
    /// - Complete states get normal build methods that construct the struct
    /// - Incomplete states get #[doc(hidden)] build methods with compile_error!
    ///   to provide helpful messages to AI coding assistants
    ///
    /// # Returns
    ///
    /// A `syn::Result<proc_macro2::TokenStream>` containing all build method implementations.
    fn generate_all_build_methods(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut tokens = proc_macro2::TokenStream::new();
        let analysis = self.token_generator.analysis();
        let num_required_fields = analysis.required_fields().len();
        let final_state_value = (1 << num_required_fields) - 1; // All bits set

        for state_combination in &self.state_combinations {
            // Calculate state value from set_fields
            let state_value = state_combination
                .set_fields
                .iter()
                .fold(0, |acc, &field_index| acc | (1 << field_index));

            // Only generate build methods for complete states
            if state_value == final_state_value {
                let build_method = self.generate_complete_build_method(state_combination)?;
                tokens.extend(build_method);
            }
        }

        Ok(tokens)
    }

    /// Generates a normal build method for a complete builder state.
    fn generate_complete_build_method(
        &self,
        state_combination: &StateCombination,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let analysis = self.token_generator.analysis();
        let struct_name = analysis.struct_name();
        let builder_ident = syn::parse_str::<Ident>(&state_combination.concrete_type_name)?;

        let impl_generics = self.token_generator.impl_generics_tokens();
        let type_generics = self.token_generator.type_generics_tokens();
        let where_clause = self.token_generator.where_clause_tokens();

        // Generate field assignments for the final struct
        let struct_field_assignments = self.generate_final_struct_assignments()?;

        // Get build method name
        let build_method_name = analysis.struct_attributes().get_build_method_name();
        let build_method_ident = syn::parse_str::<Ident>(build_method_name)?;

        let doc = self.token_generator.generate_method_documentation(
            build_method_name,
            "Builds the final instance after all required fields have been set",
            Some("This method is only available when all required fields have been provided."),
        );

        Ok(quote! {
            impl #impl_generics #builder_ident #type_generics #where_clause {
                #doc
                pub fn #build_method_ident(self) -> #struct_name #type_generics {
                    #struct_name {
                        #struct_field_assignments
                    }
                }
            }
        })
    }

    // Helper methods for state management

    /// Generates the complete state space for the type-state builder's finite automaton.
    ///
    /// This is the **core algorithm** that creates the state machine structure for compile-time
    /// validation. It implements a **systematic enumeration** of all possible combinations of
    /// required field states, creating the foundation for type-safe builder generation.
    ///
    /// # The State Generation Algorithm
    ///
    /// The algorithm uses **binary enumeration** to generate all possible states:
    ///
    /// ## Phase 1: State Space Calculation
    /// For N required fields, generate exactly 2^N states using bit manipulation.
    /// Each bit in the state mask represents a required field (0 = not set, 1 = set).
    ///
    /// ## Phase 2: Bit-to-Field Mapping
    /// Convert each bit position to field indices by checking if each bit is set
    /// in the current state mask, building a list of which fields are set in each state.
    ///
    /// ## Phase 3: Type Name Generation
    /// Create unique, deterministic type names:
    /// - **Empty state**: `{BaseBuilder}`
    /// - **Populated states**: `{BaseBuilder}_Has{Field1}_Has{Field2}...`
    ///
    /// # Example State Generation
    ///
    /// For a struct with required fields name and email, the algorithm generates 4 states:
    /// - State 0: no fields set → "UserBuilder"
    /// - State 1: name field set → "UserBuilder_HasName"
    /// - State 2: email field set → "UserBuilder_HasEmail"  
    /// - State 3: both fields set → "UserBuilder_HasName_HasEmail"
    ///
    /// # Algorithmic Properties
    ///
    /// The algorithm guarantees:
    /// 1. **Completeness**: Every possible field combination is represented
    /// 2. **Uniqueness**: Each state has a distinct type name
    /// 3. **Determinism**: Identical inputs produce identical state spaces
    /// 4. **Correctness**: State transitions are mathematically sound
    /// 5. **Efficiency**: O(2^N) time complexity (optimal for complete enumeration)
    ///
    /// # Theoretical Soundness
    ///
    /// This approach is **theoretically complete** because:
    /// - The bit enumeration covers all 2^N possible boolean combinations
    /// - Each state corresponds to exactly one concrete builder type
    /// - State transitions form a **directed acyclic graph** toward completion
    /// - The final state (all bits set) is the unique terminal state
    ///
    /// # Arguments
    ///
    /// * `analysis` - The struct analysis containing required field information
    /// * `base_builder_name` - Base name for builder types used in naming
    ///
    /// # Returns
    ///
    /// A vector containing all `StateCombination` instances representing the complete state space.
    fn generate_state_combinations(
        analysis: &StructAnalysis,
        base_builder_name: &str,
    ) -> Vec<StateCombination> {
        let mut combinations = Vec::new();
        let num_required_fields = analysis.required_fields().len();

        // Generate all possible combinations (2^n states)
        for state_mask in 0..(1 << num_required_fields) {
            let mut set_fields = Vec::new();
            let mut has_parts = Vec::new();
            let mut missing_parts = Vec::new();

            // Process all required fields to determine their state
            for field_index in 0..num_required_fields {
                let field_name = &analysis.required_fields()[field_index].name().to_string();
                let clean_name = strip_raw_identifier_prefix(field_name);
                let capitalized_name = capitalize_first_letter(&clean_name);

                if (state_mask & (1 << field_index)) != 0 {
                    // Field is set
                    set_fields.push(field_index);
                    has_parts.push(format!("Has{capitalized_name}"));
                } else {
                    // Field is missing
                    missing_parts.push(format!("Missing{capitalized_name}"));
                }
            }

            // Build type name: Has fields first, then Missing fields
            let mut type_name_parts = Vec::new();
            type_name_parts.extend(has_parts);
            type_name_parts.extend(missing_parts);

            let concrete_type_name = if type_name_parts.is_empty() {
                // This should never happen since we always have required fields
                base_builder_name.to_string()
            } else {
                let joined_parts = type_name_parts.join("_");
                format!("{base_builder_name}_{joined_parts}")
            };

            combinations.push(StateCombination {
                set_fields,
                concrete_type_name,
            });
        }

        combinations
    }

    /// Finds a state combination with a specific field set.
    ///
    /// # Arguments
    ///
    /// * `field_index` - Index of the field to set
    /// * `current_set_fields` - Currently set fields
    ///
    /// # Returns
    ///
    /// A reference to the state combination or an error if not found.
    fn find_state_with_field_set(
        &self,
        field_index: usize,
        current_set_fields: &[usize],
    ) -> syn::Result<&StateCombination> {
        let mut new_set_fields = current_set_fields.to_vec();
        if !new_set_fields.contains(&field_index) {
            new_set_fields.push(field_index);
            new_set_fields.sort_unstable();
        }

        self.state_combinations
            .iter()
            .find(|combo| combo.set_fields == new_set_fields)
            .ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("No state combination found for set_fields: {new_set_fields:?}"),
                )
            })
    }
}

/// Represents a single state in the type-state builder's finite state automaton.
///
/// This is the **fundamental unit** of the type-state pattern - each `StateCombination`
/// corresponds to one concrete builder type in the generated state machine. The type
/// system uses these distinct types to enforce compile-time validation.
///
/// # State Machine Encoding
///
/// Each state is encoded as a **bit vector** where each bit represents whether a
/// required field has been set:
///
/// For a struct with required fields name (index 0) and email (index 1), the binary
/// enumeration creates states where each bit represents field completion status:
/// - Binary 00: no fields set → ExampleBuilder
/// - Binary 01: email set → ExampleBuilder_HasEmail
/// - Binary 10: name set → ExampleBuilder_HasName  
/// - Binary 11: both set → ExampleBuilder_HasName_HasEmail
///
/// # Type Name Generation
///
/// The `concrete_type_name` follows a **systematic naming convention**:
/// - **Initial state**: `{BaseBuilder}` (no suffixes)
/// - **Intermediate states**: `{BaseBuilder}_Has{Field1}_Has{Field2}...`
/// - **Final state**: `{BaseBuilder}_Has{All_Required_Fields}`
///
/// This creates **unique, human-readable type names** that clearly indicate
/// which fields have been set in each state.
///
/// # Invariants
///
/// This struct maintains several critical invariants:
///
/// 1. **Field Index Validity**: All indices in `set_fields` must be valid
///    indices into the required fields array
/// 2. **Sorted Order**: `set_fields` is always sorted for deterministic naming
/// 3. **No Duplicates**: Each field index appears at most once
/// 4. **Type Name Uniqueness**: Each state has a unique `concrete_type_name`
///
/// # Implementation Notes
///
/// The state generation algorithm creates exactly **2^N states** for N required
/// fields, ensuring complete coverage of the state space while maintaining
/// deterministic transitions between states.
#[derive(Debug, Clone)]
struct StateCombination {
    /// Indices of required fields that are set in this state.
    ///
    /// This vector is always:
    /// - **Sorted** in ascending order for deterministic naming
    /// - **Unique** - no duplicate indices
    /// - **Valid** - all indices reference actual required fields
    ///
    /// Example: `[0, 2]` means required fields at indices 0 and 2 are set.
    set_fields: Vec<usize>,

    /// The unique concrete type name for this state's builder type.
    ///
    /// Generated using a deterministic algorithm based on `set_fields`:
    /// - Empty `set_fields` → base builder name
    /// - Non-empty → base name + "_Has" + field names
    ///
    /// Example: `"ExampleBuilder_HasName_HasEmail"`
    concrete_type_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::analyze_struct;
    use syn::parse_quote;

    #[test]
    fn test_generate_type_state_builder() {
        let input = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
                #[builder(required)]
                email: String,
                age: Option<u32>,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let result = generate_type_state_builder(&analysis);

        assert!(result.is_ok());
        let code = result.unwrap().to_string();

        // Should contain builder types for different states
        assert!(code.contains("builder"));
        // Should contain setter methods
        assert!(code.contains("name"));
        assert!(code.contains("email"));
        // Should contain build method
        assert!(code.contains("build"));
    }

    #[test]
    fn test_state_combinations_generation() {
        let input = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
                #[builder(required)]
                email: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let combinations =
            TypeStateBuilderCoordinator::generate_state_combinations(&analysis, "TestBuilder");

        // Should have 2^2 = 4 combinations for 2 required fields
        assert_eq!(combinations.len(), 4);

        // Check that we have the expected states
        let state_names: Vec<&String> = combinations
            .iter()
            .map(|combo| &combo.concrete_type_name)
            .collect();

        assert!(state_names
            .iter()
            .any(|name| name.as_str() == "TestBuilder_MissingName_MissingEmail")); // No fields set
        assert!(state_names.iter().any(|name| name.contains("HasName"))); // Name field set
        assert!(state_names.iter().any(|name| name.contains("HasEmail"))); // Email field set
        assert!(state_names
            .iter()
            .any(|name| name.as_str() == "TestBuilder_HasName_HasEmail")); // Both set
    }

    #[test]
    fn test_coordinator_creation() {
        let input = parse_quote! {
            struct Example {
                #[builder(required)]
                name: String,
            }
        };

        let analysis = analyze_struct(&input).unwrap();
        let token_generator = TokenGenerator::new(&analysis);
        let coordinator = TypeStateBuilderCoordinator::new(&token_generator);

        assert_eq!(coordinator.state_combinations.len(), 2); // 2^1 = 2 states
        assert_eq!(coordinator._base_builder_name, "ExampleBuilder");
    }
}
