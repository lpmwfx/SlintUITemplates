# `src/layout/parser.rs`

## `pub enum SplitDir { H, V }  #[derive(Debug, Clone)] pub enum PanelNode`
*Line 14 · enum*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub enum PanelNode`
*Line 17 · enum*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn ratio(&self) -> f32`
*Line 23 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn parse(dsl: &str) -> PanelNode`
*Line 33 · fn*

Parse a DSL string into a PanelNode tree.
Top-level is always an HSplit if ":" is present, VSplit if only "/" is present.

---

