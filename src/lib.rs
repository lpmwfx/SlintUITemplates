#![doc = include_str!("../README.md")]

pub mod adapter;
pub mod bindings;
pub mod dsl;
pub mod grid;
pub mod layout;
pub mod settings;
pub mod shell;

pub use grid::{ZoneModel, TargetConfig};
pub use layout::{build, SolvedItem, ItemKind, build_v2, Panel, drag, normalize};

slint::include_modules!();
