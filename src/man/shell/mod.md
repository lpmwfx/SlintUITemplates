# `shell/mod.rs`

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
N av it em co nf ig struct.

---

## `pub struct ToolbarItemConfig`
*Line 24 · struct*

Toolbar icon button.
T oo lb ar it em co nf ig struct.

---

## `pub struct ShellConfig`
*Line 37 · struct*

Full shell configuration — passed to AppAdapter::register_shell().
MenuBar is declared statically in the Window (.slint), not via ShellConfig.
S he ll co nf ig struct.

---

## `pub fn new(platform: Platform, title: impl Into<String>) -> Self`
*Line 52 · fn*

Create a new ShellConfig with the given platform and window title.

---

## `pub fn with_nav(mut self, items: Vec<NavItemConfig>) -> Self`
*Line 61 · fn*

Set nav items. Replaces any previously set items.

---

## `pub fn with_toolbar(mut self, items: Vec<ToolbarItemConfig>) -> Self`
*Line 67 · fn*

Set toolbar items. Also sets show_toolbar if items is non-empty.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
