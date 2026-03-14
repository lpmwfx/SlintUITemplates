# `src/shell/mod.rs`

## `pub mod platform;`
*Line 2 · mod*

Platform-native window chrome and shell lifecycle utilities.

---

## `pub mod models;`
*Line 6 · mod*

Slint VecModel builders for ShellConfig (nav_model, toolbar_model).

---

## `pub struct NavItemConfig`
*Line 12 · struct*

Rust-side navigation item matching the Slint NavItem struct.

---

## `pub struct ToolbarItemConfig`
*Line 20 · struct*

Toolbar icon button.

---

## `pub struct ShellConfig`
*Line 29 · struct*

Full shell configuration — passed to AppAdapter::register_shell().
MenuBar is declared statically in the Window (.slint), not via ShellConfig.

---

## `pub fn new(platform: Platform, title: impl Into<String>) -> Self`
*Line 39 · fn*

Create a new ShellConfig with the given platform and window title.

---

## `pub fn with_nav(mut self, items: Vec<NavItemConfig>) -> Self`
*Line 48 · fn*

Set nav items. Replaces any previously set items.

---

## `pub fn with_toolbar(mut self, items: Vec<ToolbarItemConfig>) -> Self`
*Line 54 · fn*

Set toolbar items. Also sets show_toolbar if items is non-empty.

---

