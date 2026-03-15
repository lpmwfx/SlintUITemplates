# `dsl/mod.rs`

## `pub mod apply;`
*Line 25 · mod*

Applies a validated `AppDsl` configuration to a live Slint window.

---

## `pub(crate) mod builder;`
*Line 27 · mod*

Builder implementation for `AppDslBuilder`.

---

## `pub mod icons;`
*Line 29 · mod*

Fluent icon name-to-codepoint registry used for icon resolution.

---

## `pub mod types;`
*Line 31 · mod*

Public input types: `BgStyle`, `Nav`, `Toolbar`.

---

## `pub mod error;`
*Line 33 · mod*

Validation error type returned by `AppDslBuilder::build()`.

---

## `pub(crate) struct ResolvedNav`
*Line 47 · struct*

Nav item after icon-name resolution — holds the resolved codepoint string.

---

## `pub(crate) struct ResolvedToolbar`
*Line 58 · struct*

Toolbar item after icon-name resolution — holds the resolved codepoint string.

---

## `pub struct AppDsl`
*Line 73 · struct*

Validated, sealed shell configuration.
Can only be constructed via `AppDsl::builder().build()`.
A pp ds l struct.

---

## `pub fn builder(title: impl Into<String>) -> AppDslBuilder`
*Line 85 · fn*

Start building a new DSL configuration with the given window title.

---

