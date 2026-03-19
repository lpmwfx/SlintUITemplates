use rhai::Engine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::AppAdapter;

/// Register all AppAdapter API functions into the Rhai engine.
pub fn register_api(engine: &mut Engine, adapter: Rc<RefCell<AppAdapter>>) {
    register_core(engine, Rc::clone(&adapter)); // why shared? Rhai closures capture shared state — each registrar needs access to same adapter
    register_settings(engine, Rc::clone(&adapter)); // why shared? Rhai closures capture shared state — settings registrar needs adapter access
    super::dsl::register_dsl(engine, adapter);
}

/// Core view/theme/status API (4 functions).
fn register_core(engine: &mut Engine, adapter: Rc<RefCell<AppAdapter>>) {
    // why shared? each Rhai closure captures its own Rc clone for callback access
    let a = Rc::clone(&adapter);
    engine.register_fn("set_active_view", move |name: String| {
        a.borrow().set_active_view(&name);
    });

    let a = Rc::clone(&adapter); // why shared? Rhai closure captures adapter
    engine.register_fn("get_active_view", move || -> String {
        a.borrow().get_active_view()
    });

    let a = Rc::clone(&adapter); // why shared? Rhai closure captures adapter
    engine.register_fn("set_dark_mode", move |on: bool| {
        a.borrow().set_dark_mode(on);
    });

    let a = Rc::clone(&adapter); // why shared? Rhai closure captures adapter
    engine.register_fn("set_status", move |text: String| {
        a.borrow().set_status(&text);
    });
}

/// Settings API — zoom, icon, font (5 functions).
fn register_settings(engine: &mut Engine, adapter: Rc<RefCell<AppAdapter>>) {
    // why shared? each Rhai closure captures its own Rc clone for settings access
    let a = Rc::clone(&adapter);
    engine.register_fn("set_zoom", move |scale: f64| {
        let mut s = crate::settings::AppSettings::default();
        s.zoom.scale = scale as f32;
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter); // why shared? Rhai closure captures adapter
    engine.register_fn("set_icon_style", move |style: String| {
        let mut s = crate::settings::AppSettings::default();
        s.icons.style = crate::settings::IconStyle::from_str(&style);
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter); // why shared? Rhai closure captures adapter
    engine.register_fn("set_icon_color", move |color: String| {
        let mut s = crate::settings::AppSettings::default();
        s.icons.color = color;
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter); // why shared? Rhai closure captures adapter
    engine.register_fn("set_font", move |family: String| {
        let mut s = crate::settings::AppSettings::default();
        s.font.family = Some(family);
        a.borrow().apply_settings(&s);
    });

    let a = Rc::clone(&adapter); // why shared? Rhai closure captures adapter
    engine.register_fn("set_font_scale", move |scale: f64| {
        let mut s = crate::settings::AppSettings::default();
        s.font.font_scale = scale as f32;
        a.borrow().apply_settings(&s);
    });
}
