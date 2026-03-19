// NOTE(mother-child): apply() is the sole non-test fn defined here; remaining
// fns are test helpers in #[cfg(test)] — total logic is minimal.

//! Per-view Rhai configuration — auto-applied when the user navigates.
//!
//! Each view can have a `.rhai` file in a `views/` directory that sets
//! view-specific chrome (status bar text, toolbar buttons) without any
//! manual wiring in the host application.

/// Typed error for view-config evaluation and loading.
#[cfg(feature = "rhai")]
pub mod error;
/// Rhai evaluation and file loading helpers.
#[cfg(feature = "rhai")]
pub mod eval;

#[cfg(feature = "rhai")]
pub use error::ViewConfigError;
#[cfg(feature = "rhai")]
pub use eval::{eval_script, eval_file, load_all};

use std::rc::Rc;
use slint::VecModel;

use crate::AppWindow;
use crate::ShellToolbarItem;
use crate::dsl::icons::fluent_icon;


/// Per-view shell chrome configuration.
/// `None` / empty = "don't override the DSL value".
#[derive(Debug, Default, Clone)]
/// V ie wc on fi g struct.
pub struct ViewConfig {
    /// Status bar text override.  `None` → keep current value.
    pub status:  Option<String>,
    /// Toolbar items.  Empty → keep current toolbar unchanged.
    pub toolbar: Vec<crate::dsl::Toolbar>,
}

/// Apply non-None / non-empty `ViewConfig` fields to the `AppWindow`.
/// Fields that are `None` or empty are left unchanged.
/// A pp ly.
pub fn apply(ui: &AppWindow, cfg: &ViewConfig) {
    if let Some(ref text) = cfg.status {
        ui.set_status_text(text.as_str().into());
    }
    if !cfg.toolbar.is_empty() {
        let items: Vec<ShellToolbarItem> = cfg.toolbar.iter().filter_map(|t| {
            let icon_code = fluent_icon(&t.icon)?;
            Some(ShellToolbarItem {
                id:      t.id.as_str().into(),
                icon:    icon_code.into(),
                tooltip: t.tooltip.as_str().into(),
            })
        }).collect();
        // REASON: Slint VecModel requires Rc for shared ownership with the UI runtime
        ui.set_toolbar_items(Rc::new(VecModel::from(items)).into());
        ui.set_show_toolbar(true);
    }
}

#[cfg(test)]
#[cfg(feature = "rhai")]
mod tests {
    use super::*;

    #[test]
    fn parse_view_status() {
        let cfg = eval_script(r#"view_status("Welcome");"#).unwrap();
        assert_eq!(cfg.status, Some("Welcome".into()));
        assert!(cfg.toolbar.is_empty());
    }

    #[test]
    fn parse_view_toolbar() {
        let cfg = eval_script(r#"view_toolbar(["add:add:New"]);"#).unwrap();
        assert!(cfg.status.is_none());
        assert_eq!(cfg.toolbar.len(), 1);
        assert_eq!(cfg.toolbar[0].id, "add");
        assert_eq!(cfg.toolbar[0].icon, "add");
        assert_eq!(cfg.toolbar[0].tooltip, "New");
    }

    #[test]
    fn empty_rhai_gives_default() {
        let cfg = eval_script("").unwrap();
        assert!(cfg.status.is_none());
        assert!(cfg.toolbar.is_empty());
    }
}
