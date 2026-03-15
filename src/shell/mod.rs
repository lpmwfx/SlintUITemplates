/// Platform-native window chrome and shell lifecycle utilities.
pub mod platform;
pub use platform::Platform;

/// Slint VecModel builders for ShellConfig (nav_model, toolbar_model).
pub mod models;


/// Rust-side navigation item matching the Slint NavItem struct.
#[derive(Debug, Clone)]
/// N av it em co nf ig struct.
pub struct NavItemConfig {
    /// I d.
    pub id:    String,
    /// L ab el.
    pub label: String,
    /// I co n.
    pub icon:  String,
}

/// Toolbar icon button.
#[derive(Debug, Clone)]
/// T oo lb ar it em co nf ig struct.
pub struct ToolbarItemConfig {
    /// I d.
    pub id:      String,
    /// I co n.
    pub icon:    String,
    /// T oo lt ip.
    pub tooltip: String,
}

/// Full shell configuration — passed to AppAdapter::register_shell().
/// MenuBar is declared statically in the Window (.slint), not via ShellConfig.
#[derive(Debug, Default)]
/// S he ll co nf ig struct.
pub struct ShellConfig {
    /// P la tf or m.
    pub platform:     Platform,
    /// T it le.
    pub title:        String,
    /// N av.
    pub nav:          Vec<NavItemConfig>,
    /// T oo lb ar.
    pub toolbar:      Vec<ToolbarItemConfig>,
    /// S ho w t oo lb ar.
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
