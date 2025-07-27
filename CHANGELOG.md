# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/welf/type-state-builder/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/welf/type-state-builder/releases/tag/v0.1.0

