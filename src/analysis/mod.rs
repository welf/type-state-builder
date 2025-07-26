//! Analysis Module
//!
//! This module contains all the analysis and data structures used to understand
//! and process struct definitions for builder generation. It's organized into
//! three main components:
//!
//! - [`struct_analysis`]: Core struct analysis with comprehensive methods
//! - [`field_analysis`]: Field information processing and utilities
//! - [`validation`]: Validation logic for ensuring correct builder generation
//!
//! # Analysis Pipeline
//!
//! The analysis process follows this pipeline:
//!
//! 1. **Parse Input** - Convert `syn::DeriveInput` into structured data
//! 2. **Extract Fields** - Separate required and optional fields
//! 3. **Analyze Generics** - Track generic parameters and lifetimes
//! 4. **Validate Configuration** - Ensure attributes are consistent
//! 5. **Prepare for Generation** - Create complete analysis context
//!
//! # Key Types
//!
//! - [`StructAnalysis`]: Complete analysis of a struct and its context
//! - [`FieldInfo`]: Information about individual fields and their attributes
//! - Validation functions for ensuring correct configurations
//!

pub mod field_analysis;
pub mod struct_analysis;
pub mod validation;

// Re-export main types for convenience
pub use field_analysis::FieldInfo;
pub use struct_analysis::StructAnalysis;

// Re-export the main analysis function
pub use struct_analysis::analyze_struct;
