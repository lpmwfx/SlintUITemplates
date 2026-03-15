# `src/dsl/mod.rs`

## `pub mod apply;`
*Line 24 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub mod icons;`
*Line 25 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub enum BgStyle`
*Line 36 · enum*

Windows Composition API backdrop style.
Mica and Acrylic require Windows 11 (22H2+) — silently falls back to Solid
on older OS versions or non-Windows platforms.

---

## `pub struct Nav`
*Line 50 · struct*

A navigation destination — icon name resolved to codepoint at build().

---

## `pub fn new( id:    impl Into<String>, label: impl Into<String>, icon:  impl Into<String>, ) -> Self`
*Line 57 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct Toolbar`
*Line 68 · struct*

A toolbar icon button — icon name resolved to codepoint at build().

---

## `pub fn new( id:      impl Into<String>, icon:    impl Into<String>, tooltip: impl Into<String>, ) -> Self`
*Line 75 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub enum DslError`
*Line 88 · enum*

Rule violations detected at `build()` time — never silent visual failures.

---

## `pub(crate) struct ResolvedNav`
*Line 133 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub(crate) struct ResolvedToolbar`
*Line 140 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct AppDsl`
*Line 151 · struct*

Validated, sealed shell configuration.
Can only be constructed via `AppDsl::builder().build()`.

---

## `pub struct AppDslBuilder`
*Line 166 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn builder(title: impl Into<String>) -> AppDslBuilder`
*Line 178 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn platform(mut self, p: Platform) -> Self`
*Line 193 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn nav(mut self, items: Vec<Nav>) -> Self`
*Line 198 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn status(mut self, text: impl Into<String>) -> Self`
*Line 203 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn toolbar(mut self, items: Vec<Toolbar>) -> Self`
*Line 208 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn window_size(mut self, width: u32, height: u32) -> Self`
*Line 213 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn bg_style(mut self, style: BgStyle) -> Self`
*Line 219 · fn*

Set OS-level window backdrop (Windows 11 only; no-op elsewhere).

---

## `pub fn views(mut self, ids: Vec<&str>) -> Self`
*Line 226 · fn*

Register ViewSlot ids — validated to match nav ids 1:1.
Optional: if omitted, nav↔view consistency is not checked.

---

## `pub fn build(self) -> Result<AppDsl, Vec<DslError>>`
*Line 232 · fn*

Validate all composition rules. Returns sealed `AppDsl` or list of errors.

---

