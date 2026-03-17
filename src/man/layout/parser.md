# `layout/parser.rs`

## `pub enum SplitDir { H, V }  /// A recursive tree node representing either a leaf panel or a split container. #[derive(Debug, Clone)] /// P an el no de enum. pub enum PanelNode`
*Line 16 · enum*

Direction of a layout split: horizontal (columns) or vertical (rows).
S pl it di r enum.

---

## `pub enum PanelNode`
*Line 21 · enum*

A recursive tree node representing either a leaf panel or a split container.
P an el no de enum.

---

## `pub fn ratio(&self) -> f32`
*Line 28 · fn*

Return the ratio weight of this node, whether leaf or split.

---

## `pub fn parse(dsl: &str) -> PanelNode`
*Line 38 · fn*

Parse a DSL string into a PanelNode tree.
Top-level is always an HSplit if ":" is present, VSplit if only "/" is present.

---

