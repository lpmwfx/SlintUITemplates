# src/adapter/config.rs

## `pub fn load_view_configs(&self, dir: &std::path::Path) -> Result<(), crate::view_config::ViewConfigError>`

*Line 13 · fn*

Load all `*.rhai` files from `dir` and register as per-view configs.
Auto-applied by the navigate handler on every nav event.

---

## `pub fn set_window_size(&self, width: u32, height: u32)`

*Line 20 · fn*

Resize the window (logical pixels).

---

## `pub fn set_bg_style_str(&mut self, style: &str)`

*Line 25 · fn*

Set the OS-level backdrop style ("mica" | "acrylic" | anything else → solid).

---

