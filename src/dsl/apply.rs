use slint::{ComponentHandle, VecModel};
use std::rc::Rc;
use crate::{AppWindow, NavItem, ShellToolbarItem};
use super::{AppDsl, BgStyle};

/// Apply a validated `AppDsl` to an `AppWindow`.
/// All composition rules already satisfied — this is pure wiring, no validation.
pub fn apply(ui: &AppWindow, dsl: &AppDsl) {
    // Nav items — resolved codepoints
    let nav: Vec<NavItem> = dsl.nav.iter().map(|n| NavItem {
        id:    n.id.clone().into(),
        label: n.label.clone().into(),
        icon:  n.icon_code.clone().into(),
    }).collect();
    ui.set_nav_items(Rc::new(VecModel::from(nav)).into());

    // Toolbar
    let toolbar: Vec<ShellToolbarItem> = dsl.toolbar.iter().map(|t| ShellToolbarItem {
        id:      t.id.clone().into(),
        icon:    t.icon_code.clone().into(),
        tooltip: t.tooltip.clone().into(),
    }).collect();
    ui.set_toolbar_items(Rc::new(VecModel::from(toolbar)).into());
    ui.set_show_toolbar(dsl.show_toolbar);

    // Status
    ui.set_status_text(dsl.status.clone().into());

    // Window size (preferred — Window respects these as initial size hints)
    if let Some((w, h)) = dsl.window_size {
        ui.window().set_size(slint::PhysicalSize::new(w, h));
    }

    // Mica/Acrylic: tell the Slint Window to use a transparent background
    // so the DWM backdrop shows through. Platform code applies DWM attrs separately.
    ui.set_mica_mode(dsl.bg_style != BgStyle::Solid);
}
