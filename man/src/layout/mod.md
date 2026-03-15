# `src/layout/mod.rs`

## `pub mod constraints;`
*Line 1 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub mod dsl_v2;`
*Line 2 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub mod parser;`
*Line 3 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub mod ratio_solver;`
*Line 4 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub mod solver;`
*Line 5 · mod*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn build(dsl: &str, win_w: f32, win_h: f32) -> Vec<SolvedItem>`
*Line 14 · fn*

Parse a DSL string and solve layout for given window dimensions.
Returns a flat Vec<SolvedItem> ready to push to Slint as [PanelItem].

---

## `pub fn build_v2(dsl: &str) -> Vec<Panel>`
*Line 21 · fn*

Parse a DSL v2 named-slot string into resizable Panels.
Use `drag()` on the returned panels to apply interactive resize.

---

