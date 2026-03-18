# src/dsl/builder.rs

## `pub struct AppDslBuilder`

*Line 29 · struct*

Fluent builder for [`AppDsl`] — the validated shell configuration.

# Example
```rust,no_run
use slint_ui_templates::dsl::{AppDsl, Nav, Toolbar};
use slint_ui_templates::dsl::BgStyle;

let dsl = AppDsl::builder("My App")
    .nav(vec![
        Nav::new("home", "Home", "home"),
        Nav::new("settings", "Settings", "settings"),
    ])
    .toolbar(vec![Toolbar::new("add", "add", "New item")])
    .status("Ready")
    .bg_style(BgStyle::Mica)
    .window_size(1200, 800)
    .build()
    .unwrap();
```

---

## `pub(crate) fn new(title: impl Into<String>) -> Self`

*Line 75 · fn*

Create a new builder with the given window title.

---

## `pub fn platform(mut self, p: Platform) -> Self`

*Line 90 · fn*

Set the target platform (affects nav-item limits and layout rules).

---

## `pub fn nav(mut self, items: Vec<Nav>) -> Self`

*Line 96 · fn*

Set the sidebar navigation items (validated at `build()` time).

---

## `pub fn status(mut self, text: impl Into<String>) -> Self`

*Line 102 · fn*

Set the initial status-bar text (defaults to "Ready").

---

## `pub fn toolbar(mut self, items: Vec<Toolbar>) -> Self`

*Line 108 · fn*

Set the toolbar icon buttons (validated at `build()` time).

---

## `pub fn window_size(mut self, width: u32, height: u32) -> Self`

*Line 114 · fn*

Set the initial window dimensions in logical pixels.

---

## `pub fn bg_style(mut self, style: super::BgStyle) -> Self`

*Line 120 · fn*

Set OS-level window backdrop (Windows 11 only; no-op elsewhere).

---

## `pub fn views(mut self, ids: Vec<&str>) -> Self`

*Line 126 · fn*

Register ViewSlot ids — validated to match nav ids 1:1.

---

## `pub fn build(self) -> Result<AppDsl, Vec<DslError>>`

*Line 132 · fn*

Validate all composition rules. Returns sealed `AppDsl` or list of errors.

---

