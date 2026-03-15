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
