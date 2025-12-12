# Implementation Plan: `#[builder(const)]` Attribute

## Overview

This document outlines the implementation plan for adding `#[builder(const)]` support to TypeStateBuilder, enabling compile-time constant builder patterns in Rust.

### Target API

```rust
#[derive(TypeStateBuilder)]
#[builder(const)]
pub struct Game {
    #[builder(required)]
    name: &'static str,

    #[builder(default = SceneId(0))]
    initial_scene: SceneId,

    // Closure syntax preserved - we generate a const fn from it
    #[builder(default = Assets::DEFAULT, converter = |assets: &[Asset]| {
        let mut arr = [None; 32];
        let mut i = 0;
        while i < assets.len() && i < 32 {
            arr[i] = Some(assets[i]);
            i += 1;
        }
        arr
    })]
    assets: [Option<Asset>; 32],
}

// Enables const construction:
const GAME: Game = Game::builder()
    .name("Snake")
    .initial_scene(SceneId(1))
    .build();
```

---

## Part 1: Rust `const fn` Constraints Analysis

### 1.1 What Works in `const fn`

- Arithmetic operations and comparisons
- Control flow (`if`, `match`, `loop`, `while`)
- Struct construction and field access
- Array/tuple construction and indexing
- References and raw pointers (limited)
- `PhantomData` construction
- `Option::None`, `Option::Some(x)` where `x` is const
- Calling other `const fn` functions
- `core::mem::*` functions (many are const)

### 1.2 What Does NOT Work in `const fn`

| Feature | Why It Fails | Impact on Builder |
|---------|--------------|-------------------|
| `impl Into<T>` bounds | Trait methods not const-callable | Must disable `impl_into` |
| `Default::default()` | `Default` trait not const | Require explicit defaults |
| `value.into()` | Trait method call | Direct value assignment |
| Heap allocation | No runtime allocator | Use `&'static str`, arrays |
| `format!`, `String::from` | Heap allocation | Use static strings |

### 1.3 Rust Version Considerations

- **Rust 1.46+**: `const fn` with control flow (if, match, loop, while)
- **Rust 1.70+**: Current MSRV, has all features we need
- **Rust 1.79+**: `const { }` blocks (nice-to-have, not required)

**Decision: No feature flag needed.** Basic `const fn` generation works on Rust 1.70 (current MSRV). All required const fn features have been stable since Rust 1.46.

### 1.4 Const Verification Strategy

**Proc-macros cannot verify const-ness.** They operate purely on syntax (tokens/AST) and have no access to type information, trait implementations, or whether a function/expression is const-evaluable.

**Our approach:**
1. Generate code assuming const-ness
2. Let the Rust compiler validate during compilation
3. Users get clear compiler errors if something isn't const

Example compiler error for non-const code:
```
error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in constant functions
```

This is acceptable because compiler errors are clear and actionable.

---

## Part 2: Codebase Impact Analysis

### 2.1 Files to Modify

```
src/
├── attributes/
│   ├── mod.rs              # Re-export new types
│   ├── struct_attrs.rs     # Add `const_builder` field + parsing
│   └── field_attrs.rs      # Add validation for const context
├── generation/
│   ├── mod.rs              # Pass const flag to generators
│   ├── tokens.rs           # Add const-aware method generation
│   ├── type_state_builder.rs  # Generate const fn methods
│   └── regular_builder.rs     # Generate const fn methods
├── utils/
│   └── field_utils.rs      # Handle const-compatible parameter config
├── validation/
│   └── struct_validator.rs # Validate const constraints
└── lib.rs                  # No changes needed
```

### 2.2 Current Method Signatures (Non-const)

```rust
// struct_constructor (type_state_builder.rs:362)
pub fn builder() -> InitialBuilder<T> { ... }

// initial_constructor (type_state_builder.rs:424)
pub fn new() -> Self { ... }

// required_field_setter (type_state_builder.rs:554-555)
pub fn field_name(self, value: ParamType) -> NextBuilder<T> { ... }

// optional_field_setter (field_analysis.rs:444-446)
pub fn field_name(mut self, value: ParamType) -> ReturnType { ... }

// build_method (type_state_builder.rs:803)
pub fn build(self) -> TargetStruct<T> { ... }
```

### 2.3 Target Method Signatures (Const)

```rust
// struct_constructor
pub const fn builder() -> InitialBuilder<T> { ... }

// initial_constructor
pub const fn new() -> Self { ... }

// setters - note: `mut self` becomes just `self` and we reconstruct
pub const fn field_name(self, value: ParamType) -> NextBuilder<T> { ... }

// build_method
pub const fn build(self) -> TargetStruct<T> { ... }
```

**Critical Note**: `mut self` in setter methods is not allowed in const fn. We must change:
```rust
// FROM:
pub fn setter(mut self, value: T) -> Self {
    self.field = value;
    self
}

// TO:
pub const fn setter(self, value: T) -> Self {
    Self { field: value, ..self }
}
```

---

## Part 3: Implementation Tasks

### 3.1 Phase 1: Attribute Parsing & Validation

#### Task 1.1: Add `const_builder` to `StructAttributes`

**File**: `src/attributes/struct_attrs.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructAttributes {
    pub build_method_name: Option<String>,
    pub setter_prefix: Option<String>,
    pub impl_into: bool,
    pub const_builder: bool,  // NEW
}

impl Default for StructAttributes {
    fn default() -> Self {
        Self {
            build_method_name: None,
            setter_prefix: None,
            impl_into: false,
            const_builder: false,  // NEW
        }
    }
}
```

#### Task 1.2: Parse `#[builder(const)]` Attribute

**File**: `src/attributes/struct_attrs.rs` in `parse_struct_attributes()`

Add parsing for the `const` keyword:
```rust
// In the attribute parsing loop:
"const" => {
    if result.const_builder {
        return Err(syn::Error::new(
            meta.path().span(),
            "duplicate `const` attribute",
        ));
    }
    result.const_builder = true;
}
```

#### Task 1.3: Validate Attribute Combinations

**File**: `src/attributes/struct_attrs.rs` in `StructAttributes::validate()`

```rust
pub fn validate(&self) -> syn::Result<()> {
    // ... existing validation ...

    // NEW: const and impl_into are mutually exclusive
    if self.const_builder && self.impl_into {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "`const` and `impl_into` cannot be used together. \
             `impl Into<T>` requires trait bounds which are not \
             supported in const fn.",
        ));
    }

    Ok(())
}
```

#### Task 1.4: Add `get_const_builder()` Accessor

```rust
impl StructAttributes {
    pub fn get_const_builder(&self) -> bool {
        self.const_builder
    }
}
```

### 3.2 Phase 2: Field-Level Validation for Const

#### Task 2.1: Validate Field Attributes in Const Context

**File**: `src/analysis/field_analysis.rs` in `FieldInfo::validate_configuration()`

Add validation that runs when the struct has `const_builder = true`:

```rust
pub fn validate_for_const(&self) -> syn::Result<()> {
    let attrs = self.attributes();

    // impl_into not allowed with const
    if attrs.impl_into == Some(true) {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "Field `{}`: `impl_into` cannot be used with `#[builder(const)]`. \
                 Remove the `impl_into` attribute or remove `const` from the struct.",
                self.clean_name()
            ),
        ));
    }

    // Note: Closure converters ARE allowed - we generate const fn from them
    // The Rust compiler will validate if the closure body is const-compatible

    Ok(())
}
```

#### Task 2.2: Helper to Detect Closure Expressions

**File**: `src/utils/field_utils.rs`

```rust
/// Determines if an expression is a closure rather than a function path.
pub fn is_closure_expression(expr: &syn::Expr) -> bool {
    matches!(expr, syn::Expr::Closure(_))
}

/// Extracts components from a closure for const fn generation.
pub fn extract_closure_components(closure: &syn::ExprClosure) -> ClosureComponents {
    ClosureComponents {
        inputs: closure.inputs.clone(),
        output: closure.output.clone(),
        body: closure.body.clone(),
    }
}

pub struct ClosureComponents {
    pub inputs: syn::punctuated::Punctuated<syn::Pat, syn::Token![,]>,
    pub output: syn::ReturnType,
    pub body: Box<syn::Expr>,
}
```

### 3.3 Phase 3: Generation Infrastructure

#### Task 3.1: Add Const Flag to `GenerationConfig`

**File**: `src/generation/mod.rs`

```rust
#[derive(Debug, Clone)]
pub struct GenerationConfig {
    pub include_documentation: bool,
    pub include_error_guidance: bool,
    pub generate_debug_impls: bool,
    pub use_qualified_paths: bool,
    pub const_builder: bool,  // NEW
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            include_documentation: true,
            include_error_guidance: true,
            generate_debug_impls: false,
            use_qualified_paths: true,
            const_builder: false,  // NEW
        }
    }
}
```

#### Task 3.2: Pass Const Flag Through TokenGenerator

**File**: `src/generation/tokens.rs`

The `TokenGenerator` already has a `config: GenerationConfig` field. Update initialization:

```rust
impl<'a> TokenGenerator<'a> {
    pub fn new(analysis: &'a StructAnalysis) -> Self {
        let mut config = GenerationConfig::default();
        // NEW: propagate const_builder from struct attributes
        config.const_builder = analysis.struct_attributes().get_const_builder();

        Self {
            analysis,
            config,
            phantom_data_field_name: Self::compute_phantom_data_field_name(analysis),
        }
    }

    // NEW: accessor for const_builder
    pub fn is_const_builder(&self) -> bool {
        self.config.const_builder
    }
}
```

#### Task 3.3: Add Const Keyword Generation Helper

**File**: `src/generation/tokens.rs`

```rust
impl<'a> TokenGenerator<'a> {
    /// Returns `const` token if const builder is enabled, otherwise empty.
    pub fn const_keyword(&self) -> proc_macro2::TokenStream {
        if self.config.const_builder {
            quote::quote! { const }
        } else {
            quote::quote! {}
        }
    }
}
```

### 3.4 Phase 4: Type-State Builder Generation (Const-Aware)

#### Task 4.1: Modify Struct Constructor Method

**File**: `src/generation/type_state_builder.rs` in `generate_struct_constructor_method()`

```rust
fn generate_struct_constructor_method(&self) -> syn::Result<proc_macro2::TokenStream> {
    // ... existing code to get identifiers ...

    let const_kw = self.token_generator.const_keyword();

    Ok(quote! {
        impl #impl_generics #struct_name #type_generics #where_clause {
            #doc
            pub #const_kw fn builder() -> #initial_builder_ident #type_generics {
                #initial_builder_ident::new()
            }
        }
    })
}
```

#### Task 4.2: Modify Initial Constructor Method

**File**: `src/generation/type_state_builder.rs` in `generate_initial_constructor_method()`

```rust
fn generate_initial_constructor_method(&self) -> syn::Result<proc_macro2::TokenStream> {
    // ... existing code ...

    let const_kw = self.token_generator.const_keyword();

    Ok(quote! {
        impl #impl_generics #initial_builder_ident #type_generics #where_clause {
            #doc
            pub #const_kw fn new() -> Self {
                Self {
                    #field_init
                }
            }
        }
    })
}
```

#### Task 4.3: Modify Required Field Setter Generation

**File**: `src/generation/type_state_builder.rs` in `generate_required_field_setter()`

The key change: when const, don't use `impl Into<T>` and change the body pattern:

```rust
fn generate_required_field_setter(
    &self,
    field: &crate::analysis::FieldInfo,
    field_index: usize,
    current_state: &StateCombination,
) -> syn::Result<proc_macro2::TokenStream> {
    // ... existing setup code ...

    let const_kw = self.token_generator.const_keyword();
    let is_const = self.token_generator.is_const_builder();

    // When const, force impl_into to false
    let use_impl_into = if is_const {
        false
    } else {
        resolve_effective_impl_into(field_impl_into, struct_impl_into)
    };

    let param_config = resolve_setter_parameter_config(field_type, converter, use_impl_into);

    // ... generate method signature and body ...

    let signature = quote! {
        pub #const_kw fn #setter_ident(self, value: #param_type) -> #output_builder_ident #type_generics
    };

    // Body uses struct reconstruction pattern (works in const)
    let body = quote! {
        #output_builder_ident {
            #field_assignments
        }
    };

    Ok(quote! {
        impl #impl_generics #input_builder_ident #type_generics #where_clause {
            #doc
            #signature {
                #body
            }
        }
    })
}
```

#### Task 4.4: Modify Optional Field Setter Generation

**File**: `src/analysis/field_analysis.rs` in `generate_setter_method()`

The current implementation uses `mut self` and `self.field = value`. For const, we need:

```rust
pub fn generate_setter_method(
    &self,
    return_type: &Type,
    struct_setter_prefix: Option<&str>,
    struct_impl_into: bool,
    is_const: bool,  // NEW parameter
) -> syn::Result<proc_macro2::TokenStream> {
    // ... existing setup ...

    let const_kw = if is_const {
        quote! { const }
    } else {
        quote! {}
    };

    // When const, force impl_into to false
    let use_impl_into = if is_const {
        false
    } else {
        resolve_effective_impl_into(field_impl_into, struct_impl_into)
    };

    let param_config = resolve_setter_parameter_config(field_type, converter, use_impl_into);

    if is_const {
        // Const-compatible pattern: reconstruct struct
        Ok(quote! {
            #[doc = #doc_comment]
            pub #const_kw fn #setter_ident(self, value: #param_type) -> #return_type {
                Self { #field_name: #field_assignment_expr, ..self }
            }
        })
    } else {
        // Original pattern with mut self
        Ok(quote! {
            #[doc = #doc_comment]
            pub fn #setter_ident(mut self, value: #param_type) -> #return_type {
                self.#field_name = #field_assignment_expr;
                self
            }
        })
    }
}
```

#### Task 4.5: Modify Build Method Generation

**File**: `src/generation/type_state_builder.rs` in `generate_complete_build_method()`

```rust
fn generate_complete_build_method(
    &self,
    state_combination: &StateCombination,
) -> syn::Result<proc_macro2::TokenStream> {
    // ... existing code ...

    let const_kw = self.token_generator.const_keyword();

    Ok(quote! {
        impl #impl_generics #builder_ident #type_generics #where_clause {
            #doc
            pub #const_kw fn #build_method_ident(self) -> #struct_name #type_generics {
                #struct_name {
                    #struct_field_assignments
                }
            }
        }
    })
}
```

### 3.5 Phase 5: Regular Builder Generation (Const-Aware)

Apply similar changes to `src/generation/regular_builder.rs`:

#### Task 5.1: Modify Constructor and Build Methods

Same pattern as type-state builder - add `const_kw` before `fn`.

#### Task 5.2: Modify Default Implementation

**Important**: When const is enabled, we cannot generate `impl Default`. The `Default` trait's `default()` method is not const. We must skip this:

```rust
fn generate_default_implementation(&self) -> syn::Result<proc_macro2::TokenStream> {
    if self.token_generator.is_const_builder() {
        // Cannot implement Default for const builders
        // Users must use ::new() or the struct's builder() method
        return Ok(quote! {});
    }

    // ... existing Default impl generation ...
}
```

### 3.6 Phase 6: Default Value Handling for Const

#### Task 6.1: Modify Field Initialization for Const

**File**: `src/analysis/field_analysis.rs` in `generate_initialization()`

When const is enabled and no custom default is provided, we cannot use `Default::default()`:

```rust
pub fn generate_initialization(
    &self,
    is_required_unset: bool,
    is_const: bool,  // NEW parameter
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name = self.name();

    if is_required_unset {
        // Required field in unset state - initialize as None (works in const)
        Ok(quote! {
            #field_name: ::core::option::Option::None,
        })
    } else {
        let default_config = self.create_default_config();

        if let Some(default_expr) = default_config.default_expression {
            // Use custom default value (must be const-evaluable if is_const)
            Ok(quote! {
                #field_name: #default_expr,
            })
        } else if is_const {
            // ERROR: const builders require explicit defaults
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Field `{}` requires an explicit default value when using `#[builder(const)]`. \
                     Add `#[builder(default = <const_expr>)]` to this field. \
                     `Default::default()` cannot be used in const context.",
                    self.clean_name()
                ),
            ));
        } else {
            // Non-const: use Default::default() (existing code)
            // ... existing implementation ...
        }
    }
}
```

### 3.7 Phase 7: Converter Handling for Const

#### Task 7.1: Generate Const Fn from Closure Converters

When `const_builder` is enabled and a closure converter is provided, we generate a `const fn` from the closure body:

```rust
// User writes:
#[builder(const, converter = |sprites: &[Sprite]| {
    let mut arr = [None; 32];
    let mut i = 0;
    while i < sprites.len() && i < 32 {
        arr[i] = Some(sprites[i]);
        i += 1;
    }
    arr
})]
pub sprites: [Option<Sprite>; 32],

// We generate:
const fn __const_sprites_setter_converter(sprites: &[Sprite]) -> [Option<Sprite>; 32] {
    let mut arr = [None; 32];
    let mut i = 0;
    while i < sprites.len() && i < 32 {
        arr[i] = Some(sprites[i]);
        i += 1;
    }
    arr
}

// And the setter calls it:
pub const fn sprites(self, value: &[Sprite]) -> NextBuilder {
    NextBuilder {
        sprites: __const_sprites_setter_converter(value),
        ..self
    }
}
```

#### Task 7.2: Implement Const Fn Generation from Closure

**File**: `src/generation/const_converter.rs` (new file)

```rust
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{ExprClosure, Ident, Type};

/// Generates a const fn from a closure expression for use in const builders.
pub fn generate_const_converter_fn(
    field_name: &Ident,
    field_type: &Type,
    closure: &ExprClosure,
) -> syn::Result<(TokenStream, Ident)> {
    // Generate meaningful function name: __const_{field}_setter_converter
    let fn_name = format_ident!("__const_{}_setter_converter", field_name);

    // Extract closure parameters
    let params = &closure.inputs;

    // Extract closure body
    let body = &closure.body;

    // Use explicit return type if provided, otherwise use field type
    let return_type = match &closure.output {
        syn::ReturnType::Default => quote! { -> #field_type },
        syn::ReturnType::Type(arrow, ty) => quote! { #arrow #ty },
    };

    let const_fn = quote! {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        const fn #fn_name(#params) #return_type {
            #body
        }
    };

    Ok((const_fn, fn_name))
}
```

#### Task 7.3: Integrate Const Converter Generation

**File**: `src/generation/type_state_builder.rs`

When generating setters for const builders with closure converters:

```rust
fn generate_required_field_setter(
    &self,
    field: &FieldInfo,
    // ... other params
) -> syn::Result<proc_macro2::TokenStream> {
    let is_const = self.token_generator.is_const_builder();
    let converter = field.attributes().converter.as_ref();

    // For const builders with closure converters, generate const fn
    let (converter_fn_def, converter_call) = if is_const {
        if let Some(conv_expr) = converter {
            if let syn::Expr::Closure(closure) = conv_expr {
                let (fn_def, fn_name) = generate_const_converter_fn(
                    field.name(),
                    field.field_type(),
                    closure,
                )?;
                (Some(fn_def), quote! { #fn_name(value) })
            } else {
                // Function path - call directly
                (None, quote! { #conv_expr(value) })
            }
        } else {
            (None, quote! { value })
        }
    } else {
        // Non-const: use existing logic
        // ...
    };

    // Include converter_fn_def in the output if present
    // ...
}
```

#### Task 7.4: Function Path Converters Also Supported

Users can also provide a function path instead of a closure:

```rust
// Option 1: Closure (converted to const fn)
#[builder(converter = |x: &[T]| to_array(x))]

// Option 2: Function path (called directly)
#[builder(converter = my_const_fn)]
```

Both work. For function paths, we already extract the parameter type from usage context or require explicit type in the closure parameter.

**Note**: If a user provides a non-const function path, the Rust compiler will error:
```
error[E0015]: cannot call non-const fn `my_non_const_fn` in constant functions
```

---

## Part 4: Testing Strategy

### 4.1 Unit Tests

Add to existing test modules:

```rust
// src/attributes/struct_attrs.rs tests
#[test]
fn test_parse_const_attribute() { ... }

#[test]
fn test_const_with_impl_into_error() { ... }

#[test]
fn test_get_const_builder() { ... }
```

### 4.2 Integration Tests

**File**: `tests/const_builder_basic.rs`

```rust
use type_state_builder::TypeStateBuilder;

#[test]
fn test_const_builder_simple() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Point {
        #[builder(required)]
        x: i32,
        #[builder(required)]
        y: i32,
        #[builder(default = 0)]
        z: i32,
    }

    const ORIGIN: Point = Point::builder().x(0).y(0).build();
    const POINT_3D: Point = Point::builder().x(1).y(2).z(3).build();

    assert_eq!(ORIGIN, Point { x: 0, y: 0, z: 0 });
    assert_eq!(POINT_3D, Point { x: 1, y: 2, z: 3 });
}

#[test]
fn test_const_builder_with_static_str() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Config {
        #[builder(required)]
        name: &'static str,
        #[builder(default = 8080)]
        port: u16,
    }

    const DEFAULT_CONFIG: Config = Config::builder()
        .name("my-app")
        .build();

    assert_eq!(DEFAULT_CONFIG.name, "my-app");
    assert_eq!(DEFAULT_CONFIG.port, 8080);
}
```

**File**: `tests/const_builder_converter.rs`

```rust
use type_state_builder::TypeStateBuilder;

#[test]
fn test_const_builder_with_closure_converter() {
    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Data {
        // Closure converter - we generate const fn from it
        #[builder(required, converter = |slice: &[i32]| {
            let mut arr = [0i32; 3];
            let mut i = 0;
            while i < slice.len() && i < 3 {
                arr[i] = slice[i];
                i += 1;
            }
            arr
        })]
        values: [i32; 3],
    }

    const DATA: Data = Data::builder().values(&[1, 2, 3]).build();
    assert_eq!(DATA.values, [1, 2, 3]);

    const PARTIAL: Data = Data::builder().values(&[42]).build();
    assert_eq!(PARTIAL.values, [42, 0, 0]);
}

#[test]
fn test_const_builder_with_fn_path_converter() {
    const fn my_converter(s: &str) -> usize {
        s.len()
    }

    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Info {
        #[builder(required, converter = my_converter)]
        name_len: usize,
    }

    const INFO: Info = Info::builder().name_len("hello").build();
    assert_eq!(INFO.name_len, 5);
}

#[test]
fn test_const_builder_optional_to_array() {
    #[derive(TypeStateBuilder, Debug, PartialEq, Clone, Copy)]
    #[builder(const)]
    struct Sprite {
        #[builder(required)]
        id: u8,
    }

    #[derive(TypeStateBuilder, Debug, PartialEq)]
    #[builder(const)]
    struct Assets {
        #[builder(default = [None; 4], converter = |sprites: &[Sprite]| {
            let mut arr: [Option<Sprite>; 4] = [None; 4];
            let mut i = 0;
            while i < sprites.len() && i < 4 {
                arr[i] = Some(sprites[i]);
                i += 1;
            }
            arr
        })]
        sprites: [Option<Sprite>; 4],
    }

    const SPRITE_A: Sprite = Sprite::builder().id(1).build();
    const SPRITE_B: Sprite = Sprite::builder().id(2).build();

    const ASSETS: Assets = Assets::builder()
        .sprites(&[SPRITE_A, SPRITE_B])
        .build();

    assert_eq!(ASSETS.sprites[0], Some(Sprite { id: 1 }));
    assert_eq!(ASSETS.sprites[1], Some(Sprite { id: 2 }));
    assert_eq!(ASSETS.sprites[2], None);
}
```

### 4.3 UI Tests (Compile-Fail) & Error Documentation

UI tests serve two purposes:
1. **Regression testing** - ensure errors are caught
2. **Error message documentation** - the `.stderr` files document expected messages

Each UI test requires both a `.rs` file and a `.stderr` file.

#### Test 1: `const` + `impl_into` Mutual Exclusion

**File**: `tests/ui/const-with-impl-into.rs`

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(const, impl_into)]  // Should fail
struct Invalid {
    #[builder(required)]
    name: String,
}

fn main() {}
```

**File**: `tests/ui/const-with-impl-into.stderr`

```text
error: `const` and `impl_into` cannot be used together. `impl Into<T>` requires trait bounds which are not supported in const fn.
 --> tests/ui/const-with-impl-into.rs:4:1
  |
4 | #[builder(const, impl_into)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

#### Test 2: Missing Explicit Default

**File**: `tests/ui/const-missing-default.rs`

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(const)]
struct Invalid {
    count: u32,  // No explicit default - error with const builder
}

fn main() {}
```

**File**: `tests/ui/const-missing-default.stderr`

```text
error: Field `count` requires an explicit default value with `#[builder(const)]`. Add `#[builder(default = <const_expr>)]`. `Default::default()` cannot be called in const context.
 --> tests/ui/const-missing-default.rs:6:5
  |
6 |     count: u32,
  |     ^^^^^
```

#### Test 3: Field-Level `impl_into` with `const`

**File**: `tests/ui/const-field-impl-into.rs`

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(const)]
struct Invalid {
    #[builder(required, impl_into)]  // Should fail
    name: String,
}

fn main() {}
```

**File**: `tests/ui/const-field-impl-into.stderr`

```text
error: Field `name`: `impl_into` cannot be used with `#[builder(const)]`. Remove the `impl_into` attribute or remove `const` from the struct.
 --> tests/ui/const-field-impl-into.rs:6:5
  |
6 |     #[builder(required, impl_into)]
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

#### Test 4: Non-Const Closure Body (Compiler Error)

**File**: `tests/ui/const-non-const-closure.rs`

```rust
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(const)]
struct Invalid {
    // This closure body calls a non-const fn - compiler will error
    #[builder(required, converter = |s: &str| s.to_string())]
    name: String,
}

fn main() {}
```

**File**: `tests/ui/const-non-const-closure.stderr`

```text
error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in constant functions
  --> tests/ui/const-non-const-closure.rs:7:47
   |
7  |     #[builder(required, converter = |s: &str| s.to_string())]
   |                                               ^^^^^^^^^^^^^
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
```

#### Running UI Tests

```bash
# Run UI tests (requires ui-tests feature)
cargo test --features ui-tests

# Update .stderr files after intentional error message changes
TRYBUILD=overwrite cargo test --features ui-tests
```

---

## Part 5: Implementation Order

### Milestone 1: Foundation (Estimated: 2-3 hours)
1. Add `const_builder` field to `StructAttributes`
2. Implement parsing for `#[builder(const)]`
3. Add validation for `const` + `impl_into` conflict
4. Add `get_const_builder()` accessor
5. Write unit tests for parsing and validation

### Milestone 2: Basic Const Generation (Estimated: 3-4 hours)
1. Add `const_builder` to `GenerationConfig`
2. Add `const_keyword()` helper to `TokenGenerator`
3. Modify `builder()` method generation
4. Modify `new()` method generation
5. Modify `build()` method generation
6. Skip `Default` impl for const builders
7. Write basic integration test

### Milestone 3: Setter Methods (Estimated: 3-4 hours)
1. Modify required field setter generation (type-state)
2. Modify optional field setter generation
3. Change `mut self` pattern to struct reconstruction
4. Handle `impl_into` disabling for const
5. Write integration tests for setters

### Milestone 4: Default Values (Estimated: 2-3 hours)
1. Modify `generate_initialization()` for const
2. Error on missing defaults in const context
3. Write tests for default value handling
4. Write UI tests for error cases

### Milestone 5: Converter Support (Estimated: 2-3 hours)
1. Add `is_closure_expression()` helper
2. Validate converters are function paths for const
3. Generate const-compatible converter calls
4. Write converter tests

### Milestone 6: UI Tests & Error Documentation (Estimated: 2-3 hours)
1. Create UI tests for all error conditions:
   - `const-with-impl-into.rs` - validates mutual exclusion
   - `const-missing-default.rs` - validates explicit default requirement
   - `const-field-impl-into.rs` - validates field-level impl_into rejection
2. Create `.stderr` files documenting expected error messages
3. Ensure error messages are clear and actionable
4. Run `cargo test --features ui-tests` to verify

### Milestone 7: Documentation & Polish (Estimated: 2 hours)
1. Update README with `#[builder(const)]` documentation
2. Add doc comments to new public APIs
3. Test edge cases (generics, lifetimes, const generics)
4. Final review and cleanup

---

## Part 6: Design Decisions (Resolved)

### D1: Feature Flag for MSRV
**Decision**: No feature flag needed.

Basic `const fn` generation works on Rust 1.70 (current MSRV). The `const fn` keyword with control flow has been stable since Rust 1.46. If future enhancements require newer Rust features, we can add a feature flag then.

### D2: Closure Converter Syntax
**Decision**: Keep closure syntax, generate `const fn` from closure body.

Users write the familiar closure syntax:
```rust
#[builder(converter = |x: &str| x.len())]
```

We generate a `const fn` with the closure's parameters and body. If the body isn't const-compatible, the Rust compiler produces a clear error. This preserves ergonomics while enabling const builders.

### D3: Default Values for Optional Fields
**Decision**: Require explicit `#[builder(default = expr)]` for all optional fields when `const` is enabled.

`Default::default()` cannot be called in const context. Rather than special-casing known types, we require explicit defaults. This is clear and predictable.

### D4: Const Verification
**Decision**: Let the Rust compiler verify const-ness.

Proc-macros cannot determine if code is const-evaluable. We generate code assuming const-ness, and the compiler validates it. Compiler errors for non-const code are clear and actionable.

### D5: Regular Builder const Support
**Decision**: Yes, apply same changes to regular builders.

Regular builders (no required fields) also support `#[builder(const)]`. The implementation is simpler since there's only one builder type.

### D6: Generics with Const Bounds
**Decision**: Already supported, no additional work needed.

Const generic parameters like `<const N: usize>` already work (see existing tests).

---

## Part 7: Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Breaking existing API | High | Const is opt-in, no breaking changes |
| Complex converter handling | Medium | Start simple, iterate |
| Edge cases with generics | Medium | Extensive testing |
| Future Rust const changes | Low | Design for current stable |

---

## Part 8: Success Criteria

1. All existing tests pass (no regressions)
2. New const builder tests pass:
   - Basic const builder with required fields
   - Const builder with explicit default values
   - Const builder with closure converters (generates const fn)
   - Const builder with function path converters
3. UI tests verify and document error messages:
   - `const-with-impl-into.rs` + `.stderr` - mutual exclusion
   - `const-missing-default.rs` + `.stderr` - explicit default requirement
   - `const-field-impl-into.rs` + `.stderr` - field-level impl_into rejection
   - `const-non-const-closure.rs` + `.stderr` - compiler catches non-const code
4. Documentation is updated (README, doc comments)
5. Can build the WOWCube-style example from spec:
   ```rust
   #[derive(TypeStateBuilder)]
   #[builder(const)]
   pub struct Game {
       #[builder(required)]
       name: &'static str,

       #[builder(default = SceneId(0))]
       initial_scene: SceneId,

       #[builder(default = [None; 8], converter = |scenes: &[Scene]| {
           let mut arr = [None; 8];
           let mut i = 0;
           while i < scenes.len() && i < 8 {
               arr[i] = Some(scenes[i]);
               i += 1;
           }
           arr
       })]
       scenes: [Option<Scene>; 8],
   }

   const GAME: Game = Game::builder()
       .name("Snake")
       .initial_scene(SceneId(1))
       .build();
   ```

---

## Part 9: Summary of Key Decisions

| Question | Decision |
|----------|----------|
| Feature flag for MSRV? | No - Rust 1.70 has all needed features |
| Closure converter syntax? | Keep it - generate `const fn` from closure |
| `Default::default()` for optionals? | Require explicit `default = expr` |
| Verify const-ness in proc-macro? | No - let Rust compiler validate |
| Support regular builders? | Yes - same changes apply |
