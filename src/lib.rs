#![doc = include_str!("../README.md")]

/// Adapter layer between the host app and the Slint UI grid/shell.
pub mod adapter;
/// Script-engine bindings (Rhai) exposed to configuration scripts.
pub mod bindings;
/// Markdown parser that converts CommonMark + GFM to `DocBlock` models.
pub mod docs;
/// Composition DSL — fluent builder API for shell and window configuration.
pub mod dsl;
/// Grid zone model and target-config loader for responsive layouts.
pub mod grid;
/// Layout DSL parser, constraint solver, and ratio-based panel engine.
pub mod layout;
/// File I/O gateway — all `std::fs` calls are funnelled through this module.
pub mod gateway;
/// OS-level window backdrop and composition effects (Mica, Acrylic).
pub mod pal;
/// Persistent application settings (zoom, theme, icons, font).
pub mod settings;
/// Platform-native window chrome, navigation models, and shell lifecycle.
pub mod shell;
/// Per-view Rhai configuration auto-applied on navigation changes.
pub mod view_config;

pub use grid::{ZoneModel, TargetConfig};
pub use layout::{build, SolvedItem, ItemKind, build_v2, Panel, drag, normalize};

slint::include_modules!();
