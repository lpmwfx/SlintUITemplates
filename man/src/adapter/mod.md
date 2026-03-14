# `src/adapter/mod.rs`

## `pub struct AppAdapter`
*Line 12 · struct*

Adapter layer between host app and Slint UI window.
Owns AppWindow handle, ViewConfig registry, menu actions, and navigation state.

---

## `pub fn new() -> Result<Self, slint::PlatformError>`
*Line 22 · fn*

Create a new adapter, initializing the Slint window and wiring event dispatch.

---

## `pub fn apply_dsl(&mut self, dsl: &AppDsl)`
*Line 59 · fn*

Apply a validated `AppDsl` — composition rules already enforced.

---

## `pub fn load_view_configs(&self, dir: &Path) -> Result<(), Box<dyn std::error::Error>>`
*Line 68 · fn*

Load all `*.rhai` files from `dir` and register as per-view configs.
Auto-applied by the navigate handler on every nav event.

---

## `pub fn set_window_size(&self, width: u32, height: u32)`
*Line 77 · fn*

Resize the window (logical pixels).

---

## `pub fn set_bg_style_str(&mut self, style: &str)`
*Line 82 · fn*

Set the OS-level backdrop style ("mica" | "acrylic" | anything else → solid).

---

## `pub fn on_navigate(&self, f: impl Fn(slint::SharedString) + 'static)`
*Line 94 · fn*

Register a handler for the navigate callback (nav item clicked).
Replaces any previously registered handler.

---

## `pub fn on_toolbar_clicked(&self, f: impl Fn(slint::SharedString) + 'static)`
*Line 99 · fn*

Register a handler for toolbar button clicks.

---

## `pub fn on_menu_action(&self, id: impl Into<String>, f: impl Fn() + 'static)`
*Line 105 · fn*

Register a handler for a specific MenuBar action id.
Multiple calls accumulate — each id maps to one handler.

---

## `pub fn apply_settings(&self, settings: &crate::settings::AppSettings)`
*Line 112 · fn*

Apply settings to the Slint UI (zoom, theme, icons, font).

---

## `pub fn apply_theme(&self)`
*Line 117 · fn*

Sync UI theme with OS dark mode preference.

---

## `pub fn apply_grid(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>>`
*Line 122 · fn*

Load grid layout from config file and apply row ratios.

---

## `pub fn set_active_view(&self, name: &str)`
*Line 136 · fn*

Switch the active view by name.

---

## `pub fn get_active_view(&self) -> String`
*Line 141 · fn*

Get the currently active view name.

---

## `pub fn set_dark_mode(&self, on: bool)`
*Line 146 · fn*

Set dark mode on/off (true = dark, false = light).

---

## `pub fn get_dark_mode(&self) -> bool`
*Line 151 · fn*

Check if dark mode is currently active.

---

## `pub fn set_status(&self, text: &str)`
*Line 156 · fn*

Set the status bar text.

---

## `pub fn run(self) -> Result<(), slint::PlatformError>`
*Line 161 · fn*

Show window and apply OS-level backdrop effects, then enter event loop.

---

## `pub(crate) fn is_dark_mode() -> bool`
*Line 169 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

