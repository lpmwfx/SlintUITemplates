# `gateway/settings.rs`

## `pub fn from_file(path: &Path) -> Result<AppSettings, Box<dyn std::error::Error>>`
*Line 5 · fn*

Deserializes settings from a TOML file at the given path.

---

## `pub fn from_file_or_default(path: &Path) -> AppSettings`
*Line 11 · fn*

Load from file, falling back to defaults if the file is absent or unparseable.

---

## `pub fn to_file(settings: &AppSettings, path: &Path) -> Result<(), Box<dyn std::error::Error>>`
*Line 16 · fn*

Serializes settings to a TOML file, creating parent directories if needed.

---

