use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use rhai::Engine;

use crate::dsl::Toolbar;
use super::{ViewConfig, ViewConfigError};

/// Maximum fields in a colon-separated toolbar DSL string (id:icon:tooltip).
const TOOLBAR_MAX_FIELDS: usize = 3;

fn parse_toolbar_dsl(s: &str) -> Option<Toolbar> {
    let parts: Vec<&str> = s.splitn(TOOLBAR_MAX_FIELDS, ':').collect();
    match parts.as_slice() {
        [id, icon, tip] => Some(Toolbar::new(*id, *icon, *tip)),
        [id, icon]      => Some(Toolbar::new(*id, *icon, "")),
        _               => None,
    }
}

fn new_cfg() -> Rc<RefCell<ViewConfig>> {
    // REASON: Multiple Rhai closures (view_status + view_toolbar) need shared access to cfg
    Rc::new(RefCell::new(ViewConfig::default()))
}

fn dynamic_to_toolbar(v: &rhai::Dynamic) -> Option<Toolbar> {
    parse_toolbar_dsl(&v.clone().into_string().ok()?)
}

/// Evaluate a Rhai script string and return a `ViewConfig`.
pub fn eval_script(script: &str) -> Result<ViewConfig, ViewConfigError> {
    let cfg = new_cfg();

    let result_eval = {
        let mut engine = Engine::new();

        let c = cfg.clone();
        engine.register_fn("view_status", move |text: String| {
            c.borrow_mut().status = Some(text);
        });

        let c = cfg.clone();
        engine.register_fn("view_toolbar", move |items: rhai::Array| {
            let toolbar: Vec<Toolbar> = items.iter()
                .filter_map(dynamic_to_toolbar)
                .collect();
            c.borrow_mut().toolbar = toolbar;
        });

        engine.run(script).map_err(ViewConfigError::Rhai)
        // engine dropped here → Rc refcounts return to 1
    };

    result_eval?;
    Rc::try_unwrap(cfg)
        .map_err(|_| ViewConfigError::RcUnwrap)
        .map(|c| c.into_inner())
}

/// Load and evaluate a single `.rhai` file (delegated to gateway).
pub fn eval_file(path: &Path) -> Result<ViewConfig, ViewConfigError> {
    crate::gateway::view_config::eval_file(path)
}

/// Scan `dir/*.rhai` and return a map of `{stem → ViewConfig}` (delegated to gateway).
pub fn load_all(dir: &Path) -> Result<HashMap<String, ViewConfig>, ViewConfigError> {
    crate::gateway::view_config::load_all(dir)
}
