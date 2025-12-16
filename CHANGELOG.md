# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2025-12-16

### Added

- **Builder method entry point** with `#[builder(builder_method)]` attribute
  - Makes a required field's setter the entry point to the builder
  - Replaces `builder()` method: use `User::id(1).name("Alice").build()` instead of `User::builder().id(1).name("Alice").build()`
  - Works with `setter_name`, `setter_prefix`, `impl_into`, and `converter` attributes
  - Compatible with const builders

### Validation

- Only one field per struct can have `builder_method`
- The field must be required (not optional)
- Cannot be combined with `skip_setter`

## [0.5.0] - 2025-12-12

### Added

- **Const builder support** with `#[builder(const)]` attribute
  - All builder methods (`builder()`, setters, `build()`) generated as `const fn`
  - Enables compile-time constant construction for embedded systems, static configuration, etc.
  - Closure converters automatically transformed to `const fn` helpers
  - Uses struct reconstruction pattern (`Self { field, ..self }`) for const compatibility

### Validation

- Require explicit defaults for optional fields when using const builders (`Default::default()` is not const)
- Disallow `impl_into` attribute with const builders (trait bounds not supported in const fn)
- Skip `Default` trait implementation for const builders

## [0.4.0] - 2025-12-09

### Changed

- **BREAKING**: Default value syntax now uses direct expressions instead of string literals
  - Old syntax: `#[builder(default = "42")]` or `#[builder(default = "Vec::new()")]`
  - New syntax: `#[builder(default = 42)]` or `#[builder(default = Vec::new())]`
  - This is more ergonomic and allows proper IDE support for default value expressions
  - String literal defaults are now actual string values: `#[builder(default = "hello")]` sets the default to the string `"hello"`

### Migration Guide

To migrate from the old syntax:
1. Remove the outer quotes from numeric literals: `default = "42"` → `default = 42`
2. Remove the outer quotes from function calls: `default = "Vec::new()"` → `default = Vec::new()`
3. For string defaults, remove the escaped inner quotes: `default = "\"hello\""` → `default = "hello"`

## [0.3.1] - 2025-08-17

### Fixed

- Documentation example in generated `builder` method now uses `ignore` attribute to prevent doctest execution conflicts

## [0.3.0] - 2025-08-01

### Added

- **Custom Converter Attribute**: New `converter` attribute for advanced field transformations using closures
  - `#[builder(converter = |value: InputType| expression)]` for custom conversion logic
  - Support for complex transformations beyond simple `Into` conversions
  - Works with any valid Rust expression or closure
  - Examples:

    ```rust
    #[builder(converter = |s: &str| s.to_uppercase())]
    name: String,

    #[builder(converter = |items: Vec<&str>| items.iter().map(|s| s.to_string()).collect())]
    tags: Vec<String>,
    ```

  - Comprehensive validation prevents conflicts with `impl_into` and `skip_setter` attributes
  - Zero runtime overhead - all conversions happen at compile time

### Changed

- Error message format improved to follow Rust's standard diagnostic format with structured error/note/help components
- All validation error messages now provide clearer, more actionable guidance

### Improved

- **Developer Experience**: More flexible field transformation options beyond basic `Into` conversions
- **Documentation**: Comprehensive examples for converter usage patterns and best practices
- **Testing**: Added extensive test coverage for converter functionality with edge cases

### Fixed

- UI test expectations updated for improved error messages
- More validation tests for attribute conflicts and invalid combinations
- Added `ui-tests` feature to prevent running tests in CI (different environments generate slightly different output)

### Migration Guide

The `converter` attribute is a new optional feature that doesn't affect existing code. All existing
`#[derive(TypeStateBuilder)]` usage continues to work exactly as before.

**New converter functionality:**

```rust
#[derive(TypeStateBuilder)]
struct Config {
    // New: Custom converter for complex transformations
    #[builder(converter = |path: &str| PathBuf::from(path).canonicalize().unwrap())]
    config_path: PathBuf,

    // Existing functionality unchanged
    #[builder(required)]
    name: String,

    #[builder(impl_into, default = "description".to_string()")]
    description: String,
}
```

## [0.2.0] - 2025-07-28

### Added

- **Ergonomic conversions with `impl_into` attribute**: New attribute system for more developer-friendly setter methods
  - `#[builder(impl_into)]` at struct level applies to all setter methods
  - `#[builder(impl_into)]` and `#[builder(impl_into = false)]` at field level for fine-grained control
  - Field-level settings override struct-level defaults for maximum flexibility
  - Setter methods accept `impl Into<FieldType>` instead of `FieldType` directly
  - Enables ergonomic usage: `.name("Alice")` instead of `.name("Alice".to_string())`
  - Zero runtime cost - all conversions happen at compile time
  - Works with common conversions: `&str` → `String`, `&str` → `PathBuf`, etc.
  - Comprehensive validation prevents conflicts with `skip_setter` attribute
  - Extensive documentation with real-world examples and usage patterns

### Improved

- **Enhanced documentation**: Added comprehensive examples for all `impl_into` usage patterns
- **Better error messages**: Clear validation errors for invalid attribute combinations
- **Testing coverage**: Added 341+ tests including integration tests and UI tests for error validation

## [0.1.2] - 2025-07-27

### Improved

- **Type name readability**: Generated builder type names now use PascalCase for field names instead of preserving
  underscores
  - `LanguageConfigBuilder_HasLanguage_id_MissingFqn_separator` →
    `LanguageConfigBuilder_HasLanguageId_MissingFqnSeparator`
  - Improves readability and follows Rust type naming conventions
  - Makes generated type names in error messages much clearer

## [0.1.1] - 2025-07-27

### Fixed

- **Visibility inheritance**: Generated builder types now correctly inherit the visibility of the original struct
  - Public structs generate public builders for cross-module usage
  - Private structs generate private builders to respect Rust privacy rules
  - All visibility levels supported: `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`, and private
  - Fixes compilation errors when using builders across module boundaries

## [0.1.0] - 2025-07-27

### Added

- Initial implementation of the type-state builder pattern derive macro
- Support for required and optional fields with compile-time validation
- Custom setter names and prefixes for flexible API design
- Custom build method names for enhanced usability
- Comprehensive generic type support including lifetimes and where clauses
- Skip setter functionality for auto-generated fields
- Custom default values for optional fields
- Zero runtime overhead with compile-time state machine validation
- Automatic builder pattern selection (type-state vs regular) based on field requirements
- Extensive test coverage including UI tests for error messages
- Complete documentation with examples and usage patterns
- Type-state builder pattern implementation for compile-time safety
- Regular builder pattern for structs with only optional fields
- Comprehensive attribute support for customization
- Full generic type and lifetime support
- Documentation and examples
- MIT OR Apache-2.0 dual license

### Security

- All code follows secure coding practices with proper error handling
- No unsafe code blocks used throughout the implementation

[Unreleased]: https://github.com/welf/type-state-builder/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/welf/type-state-builder/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/welf/type-state-builder/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/welf/type-state-builder/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/welf/type-state-builder/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/welf/type-state-builder/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/welf/type-state-builder/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/welf/type-state-builder/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/welf/type-state-builder/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/welf/type-state-builder/releases/tag/v0.1.0
