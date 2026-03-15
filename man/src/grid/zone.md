# `src/grid/zone.rs`

## `pub struct ZoneModel`
*Line 4 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct RowZone`
*Line 10 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub enum RowKind`
*Line 17 · enum*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub struct ColumnZone`
*Line 24 · struct*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn from_config(config: &GridConfig) -> Self`
*Line 31 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn total_row_ratio(&self) -> u32`
*Line 40 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn row(&self, name: &str) -> Option<&RowZone>`
*Line 44 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn column(&self, row_name: &str, col_name: &str) -> Option<&ColumnZone>`
*Line 48 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn set_module(&mut self, row_name: &str, col_name: &str, module: &str)`
*Line 55 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn total_column_ratio(&self) -> u32`
*Line 95 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

