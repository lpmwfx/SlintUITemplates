# `lib.rs`

## `pub mod adapter;`
*Line 4 · mod*

Adapter layer between the host app and the Slint UI grid/shell.

---

## `pub mod bindings;`
*Line 6 · mod*

Script-engine bindings (Rhai) exposed to configuration scripts.

---

## `pub mod docs;`
*Line 8 · mod*

Markdown parser that converts CommonMark + GFM to `DocBlock` models.

---

## `pub mod dsl;`
*Line 10 · mod*

Composition DSL — fluent builder API for shell and window configuration.

---

## `pub mod grid;`
*Line 12 · mod*

Grid zone model and target-config loader for responsive layouts.

---

## `pub mod layout;`
*Line 14 · mod*

Layout DSL parser, constraint solver, and ratio-based panel engine.

---

## `pub mod gateway;`
*Line 16 · mod*

File I/O gateway — all `std::fs` calls are funnelled through this module.

---

## `pub mod pal;`
*Line 18 · mod*

OS-level window backdrop and composition effects (Mica, Acrylic).

---

## `pub mod settings;`
*Line 20 · mod*

Persistent application settings (zoom, theme, icons, font).

---

## `pub mod shell;`
*Line 22 · mod*

Platform-native window chrome, navigation models, and shell lifecycle.

---

## `pub mod view_config;`
*Line 24 · mod*

Per-view Rhai configuration auto-applied on navigation changes.

---

