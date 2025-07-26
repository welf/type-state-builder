//! Identifier Processing Utilities
//!
//! This module provides utilities for processing and manipulating Rust identifiers,
//! particularly focusing on raw identifiers and naming conventions used in the
//! type-state builder pattern.
//!
//! # Key Concepts
//!
//! ## Raw Identifiers
//!
//! Rust allows using keywords as identifiers by prefixing them with `r#`. For example:
//! - `r#type` - uses the keyword `type` as an identifier
//! - `r#match` - uses the keyword `match` as an identifier
//!
//! When generating setter methods and documentation, we often need to strip this
//! prefix to get the "clean" name while preserving the original identifier for
//! actual field access.
//!
//! ## Naming Conventions
//!
//! The builder pattern uses several naming conventions:
//! - PascalCase for type names (e.g., `HasNameField`)
//! - snake_case for field and method names
//! - Unique suffixes to avoid naming conflicts
//!

use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Strips the `r#` prefix from raw identifiers.
pub fn strip_raw_identifier_prefix(ident_str: &str) -> Cow<str> {
    // Use strip_prefix for clean, efficient prefix removal
    // If the prefix isn't present, strip_prefix returns None and we use the original string
    if let Some(stripped) = ident_str.strip_prefix("r#") {
        Cow::Borrowed(stripped)
    } else {
        Cow::Borrowed(ident_str)
    }
}

/// Capitalizes the first letter of a string while preserving the rest.
pub fn capitalize_first_letter(s: &str) -> String {
    // Handle empty string case early
    if s.is_empty() {
        return String::new();
    }

    // Get an iterator over characters
    let mut chars = s.chars();

    // Extract the first character - we know it exists due to the empty check
    match chars.next() {
        None => String::new(), // This case is actually unreachable due to empty check
        Some(first) => {
            // Convert first character to uppercase (may yield multiple chars for some Unicode)
            // Collect into String and append the rest of the original string
            first.to_uppercase().collect::<String>() + chars.as_str()
        }
    }
}

/// Generates a unique identifier by appending a deterministic hash suffix.
///
/// This function creates unique identifiers to avoid naming conflicts in generated
/// code. It uses a combination of timestamp and content hashing to ensure uniqueness
/// while maintaining some predictability during the same compilation session.
///
/// # Purpose
///
/// Used for:
/// - Builder type names to avoid conflicts with user-defined types
/// - PhantomData field names to avoid conflicts with user fields
/// - Internal generated identifiers that must be unique
///
/// # Arguments
///
/// * `base_name` - The base name to make unique
///
/// # Returns
///
/// A unique identifier created by appending an 8-character hexadecimal suffix
/// to the base name, separated by an underscore.
///
///
///
/// # Implementation Details
///
/// The function combines:
/// 1. The base name (for human readability)
/// 2. Current system time as nanoseconds (for uniqueness across calls)
/// 3. A hash of both values (for deterministic shortened representation)
///
/// This approach ensures:
/// - **Uniqueness**: Different calls produce different results
/// - **Predictability**: Same inputs during same nanosecond produce same output
/// - **Compactness**: Only 8 additional characters regardless of base name length
/// - **Readability**: Maintains the original base name for debugging
pub fn generate_unique_identifier(base_name: &str) -> String {
    let mut hasher = DefaultHasher::new();

    // Include current timestamp for uniqueness across calls
    // Using nanoseconds provides high resolution for rapid successive calls
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default() // Handle the extremely unlikely case of clock going backwards
        .as_nanos();

    // Hash both the base name and timestamp
    // This ensures that even with the same base name, we get different results
    base_name.hash(&mut hasher);
    timestamp.hash(&mut hasher);

    let hash = hasher.finish();

    // Convert to a shorter hex string (8 characters)
    // This provides 2^32 possible values, which is more than sufficient
    // for avoiding conflicts in typical proc macro usage
    let suffix = format!("{:08x}", (hash as u32));

    format!("{base_name}_{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_raw_identifier_prefix() {
        // Test cases with raw identifiers
        assert_eq!(strip_raw_identifier_prefix("r#type"), "type");
        assert_eq!(strip_raw_identifier_prefix("r#match"), "match");
        assert_eq!(strip_raw_identifier_prefix("r#fn"), "fn");

        // Test cases with normal identifiers
        assert_eq!(strip_raw_identifier_prefix("field_name"), "field_name");
        assert_eq!(strip_raw_identifier_prefix("my_field"), "my_field");
        assert_eq!(strip_raw_identifier_prefix("x"), "x");

        // Edge cases
        assert_eq!(strip_raw_identifier_prefix(""), "");
        assert_eq!(strip_raw_identifier_prefix("r#"), "");
        assert_eq!(strip_raw_identifier_prefix("r#r#test"), "r#test"); // Nested raw prefix
    }

    #[test]
    fn test_capitalize_first_letter() {
        // Normal cases
        assert_eq!(capitalize_first_letter("field"), "Field");
        assert_eq!(capitalize_first_letter("field_name"), "Field_name");
        assert_eq!(capitalize_first_letter("user_id"), "User_id");
        assert_eq!(capitalize_first_letter("a"), "A");

        // Already capitalized
        assert_eq!(capitalize_first_letter("Field"), "Field");
        assert_eq!(capitalize_first_letter("A"), "A");

        // Edge cases
        assert_eq!(capitalize_first_letter(""), "");

        // Numbers and special characters
        assert_eq!(capitalize_first_letter("1field"), "1field"); // Numbers don't change
        assert_eq!(capitalize_first_letter("_field"), "_field"); // Underscore doesn't change
    }
}
