use slint::{ComponentHandle, VecModel};
use std::rc::Rc;
use crate::{AppWindow, NavItem, ShellToolbarItem};
use super::{AppDsl, BgStyle};

/// Apply a validated `AppDsl` to an `AppWindow`.
/// All composition rules already satisfied — this is pure wiring, no validation.
pub fn apply(ui: &AppWindow, dsl: &AppDsl) {
    // Nav items — resolved codepoints
    let nav: Vec<NavItem> = dsl.nav.iter().map(|n| NavItem {
        id:        n.id.as_str().into(),
        label:     n.label.as_str().into(),
        icon:      n.icon_code.as_str().into(),
        is_header: false,
        hidden:    false,
    }).collect();
    // Rc: Slint model requires shared ownership via ModelRc
    ui.set_nav_items(Rc::new(VecModel::from(nav)).into());

    // Toolbar
    let toolbar: Vec<ShellToolbarItem> = dsl.toolbar.iter().map(|t| ShellToolbarItem {
        id:      t.id.as_str().into(),
        icon:    t.icon_code.as_str().into(),
        tooltip: t.tooltip.as_str().into(),
    }).collect();
    // Rc: Slint model requires shared ownership via ModelRc
    ui.set_toolbar_items(Rc::new(VecModel::from(toolbar)).into());
    ui.set_show_toolbar(dsl.show_toolbar);

    // Status
    ui.set_status_text(dsl.status.as_str().into());

    // Window size (preferred — Window respects these as initial size hints)
    if let Some((w, h)) = dsl.window_size {
        ui.window().set_size(slint::PhysicalSize::new(w, h));
    }

    // Tell Theme which material is active so bg-* tokens resolve correctly.
    // Platform code applies the actual DWM attrs separately.
    let material: &str = match dsl.bg_style {
        BgStyle::Mica    => "mica",
        BgStyle::Acrylic => "acrylic",
        BgStyle::Solid   => "solid",
    };
    ui.global::<crate::Theme>().set_material(material.into());
}
