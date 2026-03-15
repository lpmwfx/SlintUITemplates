# `view_config/mod.rs`

## `pub mod error;`
*Line 8 · mod*

Typed error for view-config evaluation and loading.

---

## `pub mod eval;`
*Line 10 · mod*

Rhai evaluation and file loading helpers.

---

## `pub struct ViewConfig`
*Line 27 · struct*

Per-view shell chrome configuration.
`None` / empty = "don't override the DSL value".
V ie wc on fi g struct.

---

## `pub fn apply(ui: &AppWindow, cfg: &ViewConfig)`
*Line 37 · fn*

Apply non-None / non-empty `ViewConfig` fields to the `AppWindow`.
Fields that are `None` or empty are left unchanged.
A pp ly.

---

