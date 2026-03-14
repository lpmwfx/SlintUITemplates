# `src/settings/mod.rs`

## `pub mod apply;`
*Line 1 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct AppSettings`
*Line 7 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct ZoomSettings`
*Line 19 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub enum ThemeMode`
*Line 25 · enum*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct ThemeSettings`
*Line 32 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub enum IconStyle`
*Line 40 · enum*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct IconSettings`
*Line 46 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn style_str(&self) -> &'static str`
*Line 53 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct FontSettings`
*Line 62 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>>`
*Line 101 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn from_file_or_default(path: &Path) -> Self`
*Line 107 · fn*

Load from file, falling back to defaults if the file is absent or unparseable.

---

## `pub fn to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>>`
*Line 111 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

