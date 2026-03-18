# src/grid/zone.rs

## `pub struct ZoneModel`

*Line 6 · struct*

Runtime grid model holding resolved rows and their proportional ratios.
Z on em od el struct.

---

## `pub struct RowZone`

*Line 15 · struct*

A resolved grid row with its name, ratio weight, and content kind.
R ow zo ne struct.

---

## `pub enum RowKind`

*Line 27 · enum*

Describes how a row's content is arranged: fixed element, column split, or empty.
R ow ki nd enum.

---

## `pub struct ColumnZone`

*Line 36 · struct*

A resolved column within a row, with an optional module assignment.
C ol um nz on e struct.

---

## `pub fn from_config(config: &GridConfig) -> Self`

*Line 47 · fn*

Builds a `ZoneModel` from a parsed `GridConfig`.

---

## `pub fn total_row_ratio(&self) -> u32`

*Line 57 · fn*

Returns the sum of all row ratio weights in the grid.

---

## `pub fn row(&self, name: &str) -> Option<&RowZone>`

*Line 62 · fn*

Looks up a row by name, returning a reference if found.

---

## `pub fn column(&self, row_name: &str, col_name: &str) -> Option<&ColumnZone>`

*Line 67 · fn*

Looks up a column by row and column name, returning a reference if found.

---

## `pub fn set_module(&mut self, row_name: &str, col_name: &str, module: &str)`

*Line 75 · fn*

Assigns a module name to a specific column within a row.

---

## `pub fn total_column_ratio(&self) -> u32`

*Line 123 · fn*

Returns the sum of all column ratio weights in this row, or 0 if not column-based.

---

