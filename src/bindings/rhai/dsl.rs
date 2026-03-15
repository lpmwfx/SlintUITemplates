use rhai::Engine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::adapter::AppAdapter;
use crate::dsl::{AppDsl, Nav};

/// Maximum fields in a colon-separated Rhai DSL string (e.g. "id:icon:tooltip").
const RHAI_MAX_FIELDS: usize = 3;

/// Register DSL API functions into the Rhai engine: nav, toolbar, window size, bg style.
pub fn register(engine: &mut Engine, adapter: Rc<RefCell<AppAdapter>>) {
    let a = Rc::clone(&adapter);
    engine.register_fn("set_nav", move |items: rhai::Array| {
        let nav: Vec<Nav> = items.iter().filter_map(|v| {
            let s = v.clone().into_string().ok()?;
            let parts: Vec<&str> = s.splitn(RHAI_MAX_FIELDS, ':').collect();
            match parts.as_slice() {
                [id, label, icon] => Some(Nav::new(*id, *label, *icon)),
                [id, label]       => Some(Nav::new(*id, *label, "list")),
                _                 => None,
            }
        }).collect();

        match AppDsl::builder("").nav(nav).build() {
            Ok(dsl)   => a.borrow_mut().apply_dsl(&dsl),
            Err(errs) => { for e in &errs { tracing::warn!("[dsl] {e}"); } }
        }
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_window_size", move |width: i64, height: i64| {
        a.borrow().set_window_size(width as u32, height as u32);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_bg_style", move |style: String| {
        a.borrow_mut().set_bg_style_str(&style);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_toolbar", move |items: rhai::Array| {
        use crate::dsl::Toolbar;
        let toolbar: Vec<Toolbar> = items.iter().filter_map(|v| {
            let s = v.clone().into_string().ok()?;
            let parts: Vec<&str> = s.splitn(RHAI_MAX_FIELDS, ':').collect();
            match parts.as_slice() {
                [id, icon, tip] => Some(Toolbar::new(*id, *icon, *tip)),
                [id, icon]      => Some(Toolbar::new(*id, *icon, "")),
                _               => None,
            }
        }).collect();

        let placeholder = vec![Nav::new("_", "_", "home")];
        match AppDsl::builder("").nav(placeholder).toolbar(toolbar).build() {
            Ok(dsl)   => a.borrow_mut().apply_dsl(&dsl),
            Err(errs) => { for e in &errs { tracing::warn!("[dsl] {e}"); } }
        }
    });
}
