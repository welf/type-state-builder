//! Validation Module
//!
//! This module provides comprehensive validation for struct analysis to ensure
//! that builder generation will succeed and produce correct code. It validates
//! both the overall struct configuration and individual field configurations.
//!
//! # Validation Strategy
//!
//! The validation process follows a focused approach:
//! 1. **Field Attribute Validation** - Check individual field configurations
//! 2. **Cross-Field Validation** - Ensure field combinations are valid
//!
//! # Validation Rules
//!
//! The module enforces these key rules:
//! - Required fields cannot have default values or skip setters
//! - Fields that skip setters must have default values
//! - Custom setter names must be valid identifiers
//!
//! Note: Generic and lifetime validation is left to the Rust compiler for better error messages.
//!

use crate::analysis::StructAnalysis;
use crate::validation::{StructValidator, ValidationContext};

/// Validates that a struct analysis is suitable for builder generation.
///
/// This is the main validation entry point that performs comprehensive
/// validation of the entire struct configuration. It checks for consistency
/// in generic parameter usage, field attribute combinations, and overall
/// builder configuration.
///
/// # Arguments
///
/// * `analysis` - The complete struct analysis to validate
///
/// # Returns
///
/// A `syn::Result<()>` indicating success or containing detailed validation
/// errors describing what needs to be fixed.
///
///
///
/// # Validation Process
///
/// The validation process focuses on builder-specific concerns:
/// 1. **Field Attribute Validation** - Checks individual field configurations
/// 2. **Cross-Field Validation** - Validates field combinations and dependencies
///
/// Note: Generic and lifetime validation is left to the Rust compiler.
///
/// # Errors
///
/// Returns specific errors for:
/// - Invalid field attribute combinations (e.g., required + default)
/// - Required fields with conflicting attributes
/// - Skip_setter fields without default values
pub fn validate_struct_for_generation(analysis: &StructAnalysis) -> syn::Result<()> {
    // Use the new centralized validation system
    let mut context = ValidationContext::new();
    let mut validator = StructValidator::new(&mut context);
    validator.validate_struct_for_generation(analysis)
}
