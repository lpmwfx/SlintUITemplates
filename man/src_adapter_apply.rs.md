# src/adapter/apply.rs

## `pub fn apply_settings(&self, settings: &crate::settings::AppSettings)`

*Line 32 · fn*

Apply settings to the Slint UI — also caches zoom scale.

---

## `pub fn apply_theme(&self)`

*Line 38 · fn*

Sync UI theme with OS dark mode preference.

---

## `pub fn apply_grid(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>>`

*Line 43 · fn*

Load grid layout from config file, apply row ratios, and update cache.

---

