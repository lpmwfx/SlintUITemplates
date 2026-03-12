use slint::{ComponentHandle, VecModel};
use std::rc::Rc;
use crate::{AppWindow, NavItem};
use super::AppDsl;

/// Apply a validated `AppDsl` to an `AppWindow`.
/// All composition rules are already satisfied — this is pure wiring.
pub fn apply(ui: &AppWindow, dsl: &AppDsl) {
    // Nav items — resolved codepoints, never raw names
    let nav: Vec<NavItem> = dsl.nav.iter().map(|n| NavItem {
        id:    n.id.clone().into(),
        label: n.label.clone().into(),
        icon:  n.icon_code.clone().into(),
    }).collect();
    ui.set_nav_items(Rc::new(VecModel::from(nav)).into());

    // Status
    ui.set_status_text(dsl.status.clone().into());
}
