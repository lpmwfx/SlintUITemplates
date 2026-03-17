use slint::ComponentHandle;
use crate::Theme;
use crate::shell::Platform;

/// # Example
/// ```rust,no_run
/// # let app = slint_ui_templates::AppAdapter::new().unwrap();
/// app.set_active_view("settings");
/// app.set_dark_mode(true);
/// app.set_status("Loading...");
/// assert_eq!(app.get_active_view(), "settings");
/// ```
impl super::AppAdapter_adp {
    /// Switch the active view by name — updates cache and UI.
    pub fn set_active_view(&self, name: &str) {
        *self.active_view.borrow_mut() = name.to_string();
        self.ui.set_active_view(name.into());
    }

    /// Get the currently active view name from the adapter cache.
    pub fn get_active_view(&self) -> String {
        self.active_view.borrow().clone()
    }

    /// Set dark mode on/off — updates cache and UI Theme global.
    pub fn set_dark_mode(&self, on: bool) {
        *self.dark_mode.borrow_mut() = on;
        self.ui.global::<Theme>().set_dark(on);
    }

    /// Check if dark mode is currently active (reads from cache).
    pub fn get_dark_mode(&self) -> bool {
        *self.dark_mode.borrow()
    }

    /// Set the status bar text — updates cache and UI.
    pub fn set_status(&self, text: &str) {
        *self.status_text.borrow_mut() = text.to_string();
        self.ui.set_status_text(text.into());
    }

    /// Get the current status bar text from the adapter cache.
    pub fn get_status(&self) -> String {
        self.status_text.borrow().clone()
    }

    /// Get the current UI zoom scale from the adapter cache.
    pub fn get_zoom(&self) -> f32 {
        *self.zoom.borrow()
    }

    /// Get the top row ratio from the adapter cache.
    pub fn get_row_top_ratio(&self) -> f32 {
        *self.row_top_ratio.borrow()
    }

    /// Get the main row ratio from the adapter cache.
    pub fn get_row_main_ratio(&self) -> f32 {
        *self.row_main_ratio.borrow()
    }

    /// Set the active platform — updates cache and Theme.platform token router.
    /// Call once at startup before `run()` (or when switching platforms).
    pub fn set_platform(&self, platform: Platform) {
        self.ui.global::<Theme>().set_platform(platform.as_str().into());
        *self.platform.borrow_mut() = platform;
    }

    /// Get the current platform from the adapter cache.
    pub fn get_platform(&self) -> Platform {
        self.platform.borrow().clone()
    }
}
