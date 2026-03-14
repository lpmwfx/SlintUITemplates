/// Slint VecModel builders for ShellConfig — kept separate to limit mother size.
use super::ShellConfig;
use crate::{NavItem, ShellToolbarItem};
use slint::VecModel;
use std::rc::Rc;

impl ShellConfig {
    /// Convert nav items to a Slint VecModel ready for set_nav_items().
    pub fn nav_model(&self) -> Rc<VecModel<NavItem>> {
        let items: Vec<NavItem> = self.nav.iter().map(|n| NavItem {
            id:        n.id.as_str().into(),
            label:     n.label.as_str().into(),
            icon:      n.icon.as_str().into(),
            is_header: false,
            hidden:    false,
        }).collect();
        // Rc: Slint model requires shared ownership via ModelRc
        Rc::new(VecModel::from(items))
    }

    /// Convert toolbar items to a Slint VecModel ready for set_toolbar_items().
    pub fn toolbar_model(&self) -> Rc<VecModel<ShellToolbarItem>> {
        let items: Vec<ShellToolbarItem> = self.toolbar.iter().map(|t| ShellToolbarItem {
            id:      t.id.as_str().into(),
            icon:    t.icon.as_str().into(),
            tooltip: t.tooltip.as_str().into(),
        }).collect();
        // Rc: Slint model requires shared ownership via ModelRc
        Rc::new(VecModel::from(items))
    }
}
