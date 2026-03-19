// NOTE(mother-child): These methods mutate Rc<RefCell<>> fields owned by the parent
// AppAdapter_adp struct. The shared state lives in mod.rs; this child only writes into it
// via the public registration API — no module-level statics here.

/// # Example
/// ```rust,no_run
/// # let app = slint_ui_templates::AppAdapter::new().unwrap();
/// app.on_navigate(|view| { println!("navigated to {view}"); });
/// app.on_toolbar_clicked(|id| { println!("toolbar: {id}"); });
/// app.on_menu_action("exit", || { std::process::exit(0); });
/// ```
impl super::AppAdapter_adp {
    /// Register a handler for the navigate callback (nav item clicked).
    /// Replaces any previously registered handler.
    pub fn on_navigate(&self, f: impl Fn(slint::SharedString) + 'static) {
        *self.navigate_cb.borrow_mut() = Some(Box::new(f));
    }

    /// Register a handler for toolbar button clicks.
    /// Stored in adapter cache; dispatched via the handler wired in new().
    pub fn on_toolbar_clicked(&self, f: impl Fn(slint::SharedString) + 'static) {
        *self.toolbar_cb.borrow_mut() = Some(Box::new(f));
    }

    /// Register a handler for a specific MenuBar action id.
    /// Multiple calls accumulate — each id maps to one handler.
    pub fn on_menu_action(&self, id: impl Into<String>, f: impl Fn() + 'static) {
        self.menu_actions.borrow_mut().insert(id.into(), Box::new(f));
    }
}
