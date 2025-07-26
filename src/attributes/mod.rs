//! Attribute Parsing Module
//!
//! This module handles parsing and processing of builder-specific attributes
//! from struct and field definitions. It provides a clean interface for
//! extracting attribute information and validating attribute combinations.
//!
//! # Supported Attributes
//!
//! ## Struct-level Attributes
//!
//! - `#[builder(build_method = "method_name")]` - Custom build method name
//!
//! ## Field-level Attributes
//!
//! - `#[builder(required)]` - Mark field as required
//! - `#[builder(setter_name = "name")]` - Custom setter method name
//! - `#[builder(default = "expr")]` - Custom default value expression
//! - `#[builder(skip_setter)]` - Don't generate a setter method
//!
//! # Usage
//!
//! This module provides functions to parse and extract builder attributes from
//! struct and field definitions. The parsed attributes control how the builder
//! generation process works, such as marking fields as required or customizing
//! method names.

pub mod field_attrs;
pub mod struct_attrs;

// Re-export main types for convenience
pub use field_attrs::{parse_field_attributes, FieldAttributes};
pub use struct_attrs::{parse_struct_attributes, StructAttributes};
