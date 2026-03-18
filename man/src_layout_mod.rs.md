# src/layout/mod.rs

## `pub mod constraints;`

*Line 2 · mod*

Ratio-space min/max constraints for panel sizing.

---

## `pub mod dsl_v2;`

*Line 4 · mod*

DSL v2 parser with named slot declarations.

---

## `pub mod named_parser;`

*Line 6 · mod*

Named-slot parse_named implementation.

---

## `pub mod parser;`

*Line 8 · mod*

DSL v1 parser that converts layout strings into a `PanelNode` tree.

---

## `pub mod ratio_solver;`

*Line 10 · mod*

Bi-directional ratio solver for interactive panel resizing.

---

## `pub mod solver;`

*Line 12 · mod*

Flattens a `PanelNode` tree into positioned `SolvedItem` rectangles.

---

## `pub fn build(dsl: &str, win_w: f32, win_h: f32) -> Vec<SolvedItem>`

*Line 22 · fn*

Parse a DSL string and solve layout for given window dimensions.
Returns a flat Vec<SolvedItem> ready to push to Slint as [PanelItem].

---

## `pub fn build_v2(dsl: &str) -> Vec<Panel>`

*Line 29 · fn*

Parse a DSL v2 named-slot string into resizable Panels.
Use `drag()` on the returned panels to apply interactive resize.

---

