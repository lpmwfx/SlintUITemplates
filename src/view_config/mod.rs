//! Per-view Rhai configuration — auto-applied when the user navigates.
//!
//! Each view can have a `.rhai` file in a `views/` directory that sets
//! view-specific chrome (status bar text, toolbar buttons) without any
//! manual wiring in the host application.
//!
//! # Example — `views/home.rhai`
//! ```rhai
//! view_status("Welcome");
//! ```
//!
//! # Example — `views/list.rhai`
//! ```rhai
//! view_status("Browse");
//! view_toolbar(["add:add:New", "delete:delete:Remove"]);
//! ```

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use rhai::Engine;
use slint::VecModel;

/// Maximum fields in a colon-separated toolbar DSL string (e.g. "id:icon:tooltip").
const TOOLBAR_MAX_FIELDS: usize = 3;

use crate::AppWindow;
use crate::ShellToolbarItem;
use crate::dsl::Toolbar;
use crate::dsl::icons::fluent_icon;

/// Per-view shell chrome configuration.
/// `None` / empty = "don't override the DSL value".
#[derive(Debug, Default, Clone)]
pub struct ViewConfig {
    /// Status bar text override.  `None` → keep current value.
    pub status:  Option<String>,
    /// Toolbar items.  Empty → keep current toolbar unchanged.
    pub toolbar: Vec<Toolbar>,
}

// ── Evaluation ────────────────────────────────────────────────────────────────

/// Evaluate a Rhai script string and return a `ViewConfig`.
pub fn eval_script(script: &str) -> Result<ViewConfig, Box<dyn Error>> {
    // REASON: Multiple Rhai closures (view_status + view_toolbar) need shared access to cfg
    let cfg = Rc::new(RefCell::new(ViewConfig::default()));

    // Scope the engine so its captured Rc clones are released before try_unwrap.
    let result_eval = {
        let mut engine = Engine::new();

        let c = Rc::clone(&cfg);
        engine.register_fn("view_status", move |text: String| {
            c.borrow_mut().status = Some(text);
        });

        let c = Rc::clone(&cfg);
        engine.register_fn("view_toolbar", move |items: rhai::Array| {
            let toolbar: Vec<Toolbar> = items.iter().filter_map(|v| {
                let s = v.clone().into_string().ok()?;
                let parts: Vec<&str> = s.splitn(TOOLBAR_MAX_FIELDS, ':').collect();
                match parts.as_slice() {
                    [id, icon, tip] => Some(Toolbar::new(*id, *icon, *tip)),
                    [id, icon]      => Some(Toolbar::new(*id, *icon, "")),
                    _               => None,
                }
            }).collect();
            c.borrow_mut().toolbar = toolbar;
        });

        engine.run(script).map_err(|e| e as Box<dyn Error>)
        // engine dropped here → Rc refcounts return to 1
    };

    result_eval?;
    let cfg = Rc::try_unwrap(cfg)
        .map_err(|_| "ViewConfig Rc still has multiple owners")?;
    Ok(cfg.into_inner())
}

/// Load and evaluate a single `.rhai` file.
pub fn eval_file(path: &Path) -> Result<ViewConfig, Box<dyn Error>> {
    let script = std::fs::read_to_string(path)?;
    eval_script(&script)
}

/// Scan `dir/*.rhai` and return a map of `{stem → ViewConfig}`.
/// Missing or empty directory is not an error — returns an empty map.
pub fn load_all(dir: &Path) -> Result<HashMap<String, ViewConfig>, Box<dyn Error>> {
    let mut map = HashMap::new();
    if !dir.exists() {
        return Ok(map);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("rhai") {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if !stem.is_empty() {
                map.insert(stem, eval_file(&path)?);
            }
        }
    }
    Ok(map)
}

// ── Apply ─────────────────────────────────────────────────────────────────────

/// Apply non-None / non-empty `ViewConfig` fields to the `AppWindow`.
/// Fields that are `None` or empty are left unchanged.
pub fn apply(ui: &AppWindow, cfg: &ViewConfig) {
    if let Some(ref text) = cfg.status {
        ui.set_status_text(text.clone().into());
    }
    if !cfg.toolbar.is_empty() {
        let items: Vec<ShellToolbarItem> = cfg.toolbar.iter().filter_map(|t| {
            let icon_code = fluent_icon(&t.icon)?;  // silent skip on unknown icon
            Some(ShellToolbarItem {
                id:      t.id.clone().into(),
                icon:    icon_code.into(),
                tooltip: t.tooltip.clone().into(),
            })
        }).collect();
        // REASON: Slint VecModel requires Rc for shared ownership with the UI runtime
        ui.set_toolbar_items(Rc::new(VecModel::from(items)).into());
        ui.set_show_toolbar(true);
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
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
