use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use slint::ComponentHandle;
use crate::AppWindow;
use crate::dsl::{AppDsl, BgStyle};
use crate::shell::Platform;

mod apply;
mod callbacks;
mod config;
mod state;

/// Adapter layer between host app and Slint UI window.
/// Owns AppWindow handle, ViewConfig registry, menu actions, and all cached state.
///
/// # Example
/// ```rust,no_run
/// use slint_ui_templates::AppAdapter;
/// use slint_ui_templates::dsl::{AppDsl, Nav};
///
/// let mut app = AppAdapter::new().unwrap();
/// let dsl = AppDsl::builder("My App")
///     .nav(vec![Nav::new("home", "Home", "home")])
///     .build().unwrap();
/// app.apply_dsl(&dsl);
/// app.run().unwrap();
/// ```
/// A pp ad ap te r adp struct.
pub struct AppAdapter_adp {
    pub(super) ui:           AppWindow,
    // REASON: Multiple closures (menu handler + navigate handler) need shared access
    pub(super) menu_actions: Rc<RefCell<HashMap<String, Box<dyn Fn()>>>>,
    pub(super) bg_style:     BgStyle,
    // REASON: Navigate handler and on_navigate callback share config registry
    pub(super) view_configs: Rc<RefCell<HashMap<String, crate::view_config::ViewConfig>>>,
    // REASON: Navigate handler can call user callback; on_navigate() replaces it
    pub(super) navigate_cb:  Rc<RefCell<Option<Box<dyn Fn(slint::SharedString)>>>>,
    // REASON: Toolbar handler dispatches to user callback registered via on_toolbar_clicked()
    pub(super) toolbar_cb:   Rc<RefCell<Option<Box<dyn Fn(slint::SharedString)>>>>,
    // State cache — adapter is source of truth, not the UI widget
    pub(super) active_view:    RefCell<String>,
    pub(super) dark_mode:      RefCell<bool>,
    pub(super) platform:       RefCell<Platform>,
    // REASON: Navigate handler closure updates status cache on view_config apply
    pub(super) status_text:    Rc<RefCell<String>>,
    pub(super) row_top_ratio:  RefCell<f32>,
    pub(super) row_main_ratio: RefCell<f32>,
    pub(super) zoom:           RefCell<f32>,
}

impl AppAdapter_adp {
    /// Create a new adapter, initializing the Slint window and wiring event dispatch.
    pub fn new() -> Result<Self, slint::PlatformError> {
        let ui = AppWindow::new()?;
        let menu_actions: Rc<RefCell<HashMap<String, Box<dyn Fn()>>>> =
            // REASON: Multiple closures (menu handler + navigate handler) need shared access
            Rc::new(RefCell::new(HashMap::new()));
        let view_configs: Rc<RefCell<HashMap<String, crate::view_config::ViewConfig>>> =
            // REASON: Navigate handler and on_navigate callback share config registry
            Rc::new(RefCell::new(HashMap::new()));
        let navigate_cb: Rc<RefCell<Option<Box<dyn Fn(slint::SharedString)>>>> =
            // REASON: Navigate handler can call user callback; on_navigate() replaces it
            Rc::new(RefCell::new(None));
        let toolbar_cb: Rc<RefCell<Option<Box<dyn Fn(slint::SharedString)>>>> =
            // REASON: Toolbar handler dispatches to user callback registered via on_toolbar_clicked
            Rc::new(RefCell::new(None));
        // REASON: Navigate handler closure updates status cache on view_config apply
        let status_text: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

        // why shared? on_menu_action closure dispatches by id from the shared action map
        let actions = Rc::clone(&menu_actions);
        ui.on_menu_action(move |id| {
            if let Some(f) = actions.borrow().get(id.as_str()) { f(); }
        });

        // why shared? on_navigate closure applies view config, updates status, calls user callback
        let vc     = Rc::clone(&view_configs);
        let nav_cb = Rc::clone(&navigate_cb);
        let ui_nav = ui.clone_strong();
        let status = Rc::clone(&status_text);
        ui.on_navigate(move |id| {
            // active-view is `in property` — only Rust can set it
            ui_nav.set_active_view(id.clone());
            if let Some(cfg) = vc.borrow().get(id.as_str()) {
                crate::view_config::apply(&ui_nav, cfg);
                if let Some(ref text) = cfg.status {
                    *status.borrow_mut() = text.clone();
                }
            }
            if let Some(ref f) = *nav_cb.borrow() { f(id); }
        });

        // why shared? on_toolbar_clicked closure dispatches to user-registered callback
        let tb_cb = Rc::clone(&toolbar_cb);
        ui.on_toolbar_clicked(move |id| {
            if let Some(ref f) = *tb_cb.borrow() { f(id); }
        });

        Ok(Self {
            ui,
            menu_actions,
            bg_style: BgStyle::Solid,
            view_configs,
            navigate_cb,
            toolbar_cb,
            status_text,
            active_view:    RefCell::new(String::new()),
            dark_mode:      RefCell::new(false),
            platform:       RefCell::new(Platform::default()),
            row_top_ratio:  RefCell::new(0.0),
            row_main_ratio: RefCell::new(0.0),
            zoom:           RefCell::new(1.0),
        })
    }

    /// Apply a validated `AppDsl` — composition rules already enforced.
    pub fn apply_dsl(&mut self, dsl: &AppDsl) {
        self.bg_style = dsl.bg_style;
        *self.status_text.borrow_mut() = dsl.status.clone();
        crate::dsl::apply::apply(&self.ui, dsl);
    }

    /// Show window and apply OS-level backdrop effects, then enter event loop.
    pub fn run(self) -> Result<(), slint::PlatformError> {
        self.ui.show()?;
        crate::pal::apply_backdrop(&self.ui.window(), self.bg_style);
        self.ui.run()
    }
}

#[cfg(test)]
mod tests;
