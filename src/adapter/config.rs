use slint::ComponentHandle;

impl super::AppAdapter_adp {
    /// Load all `*.rhai` files from `dir` and register as per-view configs.
    /// Auto-applied by the navigate handler on every nav event.
    #[cfg(feature = "rhai")]
    pub fn load_view_configs(&self, dir: &std::path::Path) -> Result<(), crate::view_config::ViewConfigError> {
        let configs = crate::view_config::load_all(dir)?;
        *self.view_configs.borrow_mut() = configs;
        Ok(())
    }

    /// Resize the window (logical pixels).
    pub fn set_window_size(&self, width: u32, height: u32) {
        self.ui.window().set_size(slint::PhysicalSize::new(width, height));
    }

    /// Set the OS-level backdrop style ("mica" | "acrylic" | anything else → solid).
    pub fn set_bg_style_str(&mut self, style: &str) {
        self.bg_style = crate::dsl::BgStyle::from_str(style);
    }
}
