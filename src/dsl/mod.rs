//! Composition DSL — fluent builder API for SlintUI shell configuration.
//!
//! Rules from `rules/slint/composition.md` are enforced **by construction**:
//! wrong configurations are `DslError`, not silent visual failures.
//!
//! # Example
//! ```rust,no_run
//! use slint_ui_templates::dsl::{AppDsl, Nav, Toolbar};
//! use slint_ui_templates::shell::Platform;
//!
//! let dsl = AppDsl::builder("My App")
//!     .platform(Platform::Windows)
//!     .window_size(1100, 720)
//!     .nav(vec![
//!         Nav::new("home",     "Home",     "home"),
//!         Nav::new("list",     "List",     "list"),
//!         Nav::new("settings", "Settings", "settings"),
//!     ])
//!     .status("Ready")
//!     .build()
//!     .expect("invalid DSL configuration");
//! ```

/// Applies a validated `AppDsl` configuration to a live Slint window.
pub mod apply;
/// Builder implementation for `AppDslBuilder`.
pub(crate) mod builder;
/// Fluent icon name-to-codepoint registry used for icon resolution.
pub mod icons;
/// Public input types: `BgStyle`, `Nav`, `Toolbar`.
pub mod types;
/// Validation error type returned by `AppDslBuilder::build()`.
pub mod error;

pub use types::{BgStyle, Nav, Toolbar};
pub use error::DslError;
pub use builder::AppDslBuilder;
pub use crate::shell::Platform;

#[cfg(test)]
mod tests;

// ── Resolved internal types ───────────────────────────────────────────────────

/// Nav item after icon-name resolution — holds the resolved codepoint string.
#[derive(Debug, Clone)]
pub(crate) struct ResolvedNav {
    /// I d.
    pub id:        String,
    /// L ab el.
    pub label:     String,
    /// I co n c od e.
    pub icon_code: String,
}

/// Toolbar item after icon-name resolution — holds the resolved codepoint string.
#[derive(Debug, Clone)]
pub(crate) struct ResolvedToolbar {
    /// I d.
    pub id:        String,
    /// I co n c od e.
    pub icon_code: String,
    /// T oo lt ip.
    pub tooltip:   String,
}

// ── Validated configuration ───────────────────────────────────────────────────

/// Validated, sealed shell configuration.
/// Can only be constructed via `AppDsl::builder().build()`.
#[derive(Debug)]
/// A pp ds l struct.
pub struct AppDsl {
    pub(crate) nav:          Vec<ResolvedNav>,
    pub(crate) status:       String,
    pub(crate) toolbar:      Vec<ResolvedToolbar>,
    pub(crate) show_toolbar: bool,
    pub(crate) window_size:  Option<(u32, u32)>,
    pub(crate) bg_style:     BgStyle,
}

impl AppDsl {
    /// Start building a new DSL configuration with the given window title.
    pub fn builder(title: impl Into<String>) -> AppDslBuilder {
        AppDslBuilder::new(title)
    }
}
