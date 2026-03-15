# `settings/mod.rs`

## `pub mod apply;`
*Line 2 · mod*

Applies loaded settings to the live Slint UI globals.

---

## `pub struct AppSettings`
*Line 11 · struct*

Top-level application settings loaded from a TOML configuration file.
A pp se tt in gs struct.

---

## `pub struct ZoomSettings`
*Line 29 · struct*

UI zoom / DPI-scale configuration.
Z oo ms et ti ng s struct.

---

## `pub enum ThemeMode`
*Line 38 · enum*

Light, dark, or system-detected colour-scheme selection.
T he me mo de enum.

---

## `pub struct ThemeSettings`
*Line 47 · struct*

Theme configuration: colour-scheme mode and optional accent override.
T he me se tt in gs struct.

---

## `pub enum IconStyle`
*Line 58 · enum*

Filled or outlined icon variant for the Fluent icon set.
I co ns ty le enum.

---

## `pub struct IconSettings`
*Line 66 · struct*

Icon appearance configuration: style variant and colour strategy.
I co ns et ti ng s struct.

---

## `pub struct FontSettings`
*Line 76 · struct*

Font family and scale configuration.
F on ts et ti ng s struct.

---

## `pub fn from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>>`
*Line 87 · fn*

Deserializes settings from a TOML file at the given path.

---

## `pub fn from_file_or_default(path: &std::path::Path) -> Self`
*Line 92 · fn*

Load from file, falling back to defaults if the file is absent or unparseable.

---

## `pub fn to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>>`
*Line 97 · fn*

Serializes settings to a TOML file, creating parent directories if needed.

---

