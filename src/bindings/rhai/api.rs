use rhai::Engine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::adapter::AppAdapter;

/// Register all AppAdapter API functions into the Rhai engine.
pub fn register(engine: &mut Engine, adapter: Rc<RefCell<AppAdapter>>) {
    let a = Rc::clone(&adapter);
    engine.register_fn("set_active_view", move |name: String| {
        a.borrow().set_active_view(&name);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("get_active_view", move || -> String {
        a.borrow().get_active_view()
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_dark_mode", move |on: bool| {
        a.borrow().set_dark_mode(on);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_status", move |text: String| {
        a.borrow().set_status(&text);
    });

    // Settings API
    let a = Rc::clone(&adapter);
    engine.register_fn("set_zoom", move |scale: f64| {
        let mut s = crate::settings::AppSettings::default();
        s.zoom.scale = scale as f32;
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_icon_style", move |style: String| {
        let mut s = crate::settings::AppSettings::default();
        s.icons.style = if style == "outlined" {
            crate::settings::IconStyle::Outlined
        } else {
            crate::settings::IconStyle::Filled
        };
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_icon_color", move |color: String| {
        let mut s = crate::settings::AppSettings::default();
        s.icons.color = color;
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_font", move |family: String| {
        let mut s = crate::settings::AppSettings::default();
        s.font.family = Some(family);
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter);
    engine.register_fn("set_font_scale", move |scale: f64| {
        let mut s = crate::settings::AppSettings::default();
        s.font.font_scale = scale as f32;
        a.borrow().apply_settings(&s);
    });
}
