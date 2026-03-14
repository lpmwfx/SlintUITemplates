# `src/layout/dsl_v2.rs`

## `pub struct NamedPanel`
*Line 18 · struct*

A parsed named panel with resolved ratio.

---

## `pub enum SplitDir { H, V }  /// Parse DSL v2 string into a flat list of named panels (horizontal split only at top level). /// Nested VSplit ("header/content/footer") returns panels in row order. pub fn parse_named(dsl: &str) -> Vec<NamedPanel>`
*Line 25 · enum*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn parse_named(dsl: &str) -> Vec<NamedPanel>`
*Line 29 · fn*

Parse DSL v2 string into a flat list of named panels (horizontal split only at top level).
Nested VSplit ("header/content/footer") returns panels in row order.

---

## `pub fn to_panels(named: &[NamedPanel]) -> Vec<Panel>`
*Line 84 · fn*

Convert named panels to ratio_solver Panels (ready for drag()).

---

