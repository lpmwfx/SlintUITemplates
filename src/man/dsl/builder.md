# `dsl/builder.rs`

## `pub struct AppDslBuilder`
*Line 11 · struct*

Fluent builder for constructing a validated `AppDsl` configuration.

---

## `pub(crate) fn new(title: impl Into<String>) -> Self`
*Line 58 · fn*

Create a new builder with the given window title.

---

## `pub fn platform(mut self, p: Platform) -> Self`
*Line 73 · fn*

Set the target platform (affects nav-item limits and layout rules).

---

## `pub fn nav(mut self, items: Vec<Nav>) -> Self`
*Line 79 · fn*

Set the sidebar navigation items (validated at `build()` time).

---

## `pub fn status(mut self, text: impl Into<String>) -> Self`
*Line 85 · fn*

Set the initial status-bar text (defaults to "Ready").

---

## `pub fn toolbar(mut self, items: Vec<Toolbar>) -> Self`
*Line 91 · fn*

Set the toolbar icon buttons (validated at `build()` time).

---

## `pub fn window_size(mut self, width: u32, height: u32) -> Self`
*Line 97 · fn*

Set the initial window dimensions in logical pixels.

---

## `pub fn bg_style(mut self, style: super::BgStyle) -> Self`
*Line 103 · fn*

Set OS-level window backdrop (Windows 11 only; no-op elsewhere).

---

## `pub fn views(mut self, ids: Vec<&str>) -> Self`
*Line 109 · fn*

Register ViewSlot ids — validated to match nav ids 1:1.

---

## `pub fn build(self) -> Result<AppDsl, Vec<DslError>>`
*Line 115 · fn*

Validate all composition rules. Returns sealed `AppDsl` or list of errors.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
