//! Centralized Validation System
//!
//! This module provides a unified validation system that eliminates duplication
//! across the codebase and provides consistent error messages and validation
//! logic for all components.
//!
//! # Design Principles
//!
//! - **Single Source of Truth**: All validation rules defined in one place
//! - **Consistent Error Messages**: Standardized error formatting and guidance
//! - **Composable Validators**: Small, focused validation functions that can be combined
//! - **Clear Error Context**: Rich error information with helpful suggestions
//!

pub mod error_messages;
pub mod field_validator;
pub mod struct_validator;
pub mod validation_context;

pub use error_messages::ErrorMessages;
pub use field_validator::FieldValidator;
pub use struct_validator::StructValidator;
pub use validation_context::ValidationContext;
