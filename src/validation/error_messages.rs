//! Centralized Error Message Management
//!
//! This module provides consistent, helpful error messages across the entire
//! codebase. All error message formatting is centralized here to ensure
//! consistency and maintainability.

/// Centralized error message generator for consistent formatting.
///
/// This struct provides methods for generating standardized error messages
/// with consistent formatting, helpful examples, and clear guidance for fixes.
pub struct ErrorMessages;

impl ErrorMessages {
    /// Creates a structured error with separate error, note, and help messages.
    ///
    /// This helper method creates a single syn::Error with a multi-line message
    /// following Rust's standard diagnostic format with separate error/note/help components.
    ///
    /// # Arguments
    ///
    /// * `span` - The syntax element to attach the error to
    /// * `error_msg` - The main error message
    /// * `note_msg` - Optional contextual note (will be prefixed with "note:")
    /// * `help_msg` - Optional help/suggestion message (will be prefixed with "help:")
    ///
    /// # Returns
    ///
    /// A single syn::Error with structured diagnostic information.
    pub fn structured_error<T: quote::ToTokens>(
        span: &T,
        error_msg: &str,
        note_msg: Option<&str>,
        help_msg: Option<&str>,
    ) -> syn::Error {
        let mut full_message = error_msg.to_string();

        if let Some(note) = note_msg {
            full_message.push_str(&format!("\nnote: {note}"));
        }

        if let Some(help) = help_msg {
            full_message.push_str(&format!("\nhelp: {help}"));
        }

        syn::Error::new_spanned(span, full_message)
    }

    /// Creates a structured error with a raw span (for use with proc_macro2::Span).
    ///
    /// This variant is used when working with raw spans that don't implement ToTokens.
    ///
    /// # Arguments
    ///
    /// * `span` - The proc_macro2::Span to attach the error to
    /// * `error_msg` - The main error message
    /// * `note_msg` - Optional contextual note (will be prefixed with "note:")
    /// * `help_msg` - Optional help/suggestion message (will be prefixed with "help:")
    ///
    /// # Returns
    ///
    /// A single syn::Error with structured diagnostic information.
    pub fn structured_error_span(
        span: proc_macro2::Span,
        error_msg: &str,
        note_msg: Option<&str>,
        help_msg: Option<&str>,
    ) -> syn::Error {
        let mut full_message = error_msg.to_string();

        if let Some(note) = note_msg {
            full_message.push_str(&format!("\nnote: {note}"));
        }

        if let Some(help) = help_msg {
            full_message.push_str(&format!("\nhelp: {help}"));
        }

        syn::Error::new(span, full_message)
    }
}
