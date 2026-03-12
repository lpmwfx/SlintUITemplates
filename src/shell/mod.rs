pub mod platform;
pub use platform::Platform;

use crate::{NavItem, ShellToolbarItem};
use slint::VecModel;
use std::rc::Rc;

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
    pub fn new(platform: Platform, title: impl Into<String>) -> Self {
        Self {
            platform,
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_nav(mut self, items: Vec<NavItemConfig>) -> Self {
        self.nav = items;
        self
    }

    pub fn with_toolbar(mut self, items: Vec<ToolbarItemConfig>) -> Self {
        self.show_toolbar = !items.is_empty();
        self.toolbar = items;
        self
    }

    /// Convert nav items to Slint VecModel.
    pub fn nav_model(&self) -> Rc<VecModel<NavItem>> {
        let items: Vec<NavItem> = self.nav.iter().map(|n| NavItem {
            id:    n.id.clone().into(),
            label: n.label.clone().into(),
            icon:  n.icon.clone().into(),
        }).collect();
        Rc::new(VecModel::from(items))
    }

    /// Convert toolbar items to Slint VecModel.
    pub fn toolbar_model(&self) -> Rc<VecModel<ShellToolbarItem>> {
        let items: Vec<ShellToolbarItem> = self.toolbar.iter().map(|t| ShellToolbarItem {
            id:      t.id.clone().into(),
            icon:    t.icon.clone().into(),
            tooltip: t.tooltip.clone().into(),
        }).collect();
        Rc::new(VecModel::from(items))
    }
}
