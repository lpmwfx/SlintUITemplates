/// Platform-native window chrome and shell lifecycle utilities.
pub mod platform;
pub use platform::Platform;

pub mod models;

use crate::{NavItem, ShellToolbarItem};

/// Rust-side navigation item matching the Slint NavItem struct.
#[derive(Debug, Clone)]
pub struct NavItemConfig {
    pub id:    String,
    pub label: String,
    pub icon:  String,
}

/// Toolbar icon button.
#[derive(Debug, Clone)]
pub struct ToolbarItemConfig {
    pub id:      String,
    pub icon:    String,
    pub tooltip: String,
}

/// Full shell configuration — passed to AppAdapter::register_shell().
/// MenuBar is declared statically in the Window (.slint), not via ShellConfig.
#[derive(Debug, Default)]
pub struct ShellConfig {
    pub platform:     Platform,
    pub title:        String,
    pub nav:          Vec<NavItemConfig>,
    pub toolbar:      Vec<ToolbarItemConfig>,
    pub show_toolbar: bool,
}

impl ShellConfig {
    /// Create a new ShellConfig with the given platform and window title.
    pub fn new(platform: Platform, title: impl Into<String>) -> Self {
        Self {
            platform,
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set nav items. Replaces any previously set items.
    pub fn with_nav(mut self, items: Vec<NavItemConfig>) -> Self {
        self.nav = items;
        self
    }

    /// Set toolbar items. Also sets show_toolbar if items is non-empty.
    pub fn with_toolbar(mut self, items: Vec<ToolbarItemConfig>) -> Self {
        self.show_toolbar = !items.is_empty();
        self.toolbar = items;
        self
    }
}
