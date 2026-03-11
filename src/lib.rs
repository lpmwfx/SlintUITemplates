pub mod grid;
pub mod layout;

pub use grid::{ZoneModel, TargetConfig};
pub use layout::{build, SolvedItem, ItemKind};

slint::include_modules!();
