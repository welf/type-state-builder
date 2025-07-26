//! Validation Context
//!
//! This module provides context information for validation operations,
//! allowing validators to access shared state and configuration.

use std::collections::HashMap;

/// Context information for validation operations.
///
/// This struct provides shared state and configuration that validators
/// can use to make consistent decisions across the validation process.
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Options for controlling validation behavior
    _options: ValidationOptions,
    /// Cached setter names for conflict detection
    setter_names: HashMap<String, String>,
}

/// Configuration options for validation behavior.
#[derive(Debug, Clone)]
pub struct ValidationOptions {
    /// Whether to perform strict validation (catch more edge cases)
    pub _strict_mode: bool,
    /// Whether to include detailed error guidance
    pub _detailed_errors: bool,
    /// Whether to validate attribute combinations exhaustively
    pub _exhaustive_attribute_validation: bool,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            _strict_mode: true,
            _detailed_errors: true,
            _exhaustive_attribute_validation: true,
        }
    }
}

impl ValidationContext {
    /// Creates a new validation context with default options.
    ///
    /// # Returns
    ///
    /// A new `ValidationContext` ready for use.
    pub fn new() -> Self {
        Self {
            _options: ValidationOptions::default(),
            setter_names: HashMap::new(),
        }
    }

    /// Records a setter name for conflict detection.
    ///
    /// # Arguments
    ///
    /// * `setter_name` - The setter method name
    /// * `field_name` - The field that uses this setter name
    ///
    /// # Returns
    ///
    /// `Some(existing_field_name)` if there's a conflict, `None` otherwise.
    pub fn record_setter_name(
        &mut self,
        setter_name: String,
        field_name: String,
    ) -> Option<String> {
        self.setter_names.insert(setter_name, field_name)
    }

    /// Clears all recorded setter names.
    pub fn clear_setter_names(&mut self) {
        self.setter_names.clear();
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}
