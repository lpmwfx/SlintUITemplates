# `layout/dsl_v2.rs`

## `pub struct NamedPanel`
*Line 19 · struct*

A parsed named panel with resolved ratio.
N am ed pa ne l struct.

---

## `pub enum SplitDir { H, V }  /// Convert named panels to ratio_solver Panels (ready for drag()). pub fn to_panels(named: &[NamedPanel]) -> Vec<Panel>`
*Line 31 · enum*

Direction of a named-slot split: horizontal (columns) or vertical (rows).
S pl it di r enum.

---

## `pub fn to_panels(named: &[NamedPanel]) -> Vec<Panel>`
*Line 34 · fn*

Convert named panels to ratio_solver Panels (ready for drag()).

---

