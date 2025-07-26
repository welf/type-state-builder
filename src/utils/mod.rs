//! Shared Utilities
//!
//! This module contains utilities and helper functions that are shared across
//! the entire type-state builder implementation. It's organized into several
//! submodules based on functionality:
//!
//! - [`identifiers`]: Identifier processing and manipulation utilities
//! - [`generics`]: Generic type analysis and transformation utilities  
//! - [`field_utils`]: Field processing and utility functions
//!
//! # Design Principles
//!
//! All utilities in this module follow these principles:
//! - **Pure functions**: No side effects, predictable outputs
//! - **Well documented**: Comprehensive documentation with examples
//! - **Error handling**: Proper error propagation with descriptive messages
//! - **Performance**: Efficient implementations for proc macro context

pub mod extensions;
pub mod field_utils;
pub mod generics;
pub mod identifiers;

// Re-export commonly used utilities for convenience
