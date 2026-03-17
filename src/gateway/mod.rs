/// File I/O gateway for settings persistence.
pub mod settings;
/// File I/O gateway for view-config loading.
#[cfg(feature = "rhai")]
pub mod view_config;
/// File I/O gateway for grid-config loading.
pub mod grid;
/// File I/O gateway for project scaffolding.
pub mod scaffold;
/// File I/O gateway for loading Rhai scripts.
#[cfg(feature = "rhai")]
pub mod scripts;
