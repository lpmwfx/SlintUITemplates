use rhai::Engine;
use std::rc::Rc;
use std::cell::RefCell;
use crate::adapter::AppAdapter;
use crate::dsl::{AppDsl, Nav};

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

    // DSL API — nav items as array of "id:Label:icon" strings
    // Example: set_nav(["home:Home:home", "list:List:list", "settings:Settings:settings"]);
    let a = Rc::clone(&adapter);
    engine.register_fn("set_nav", move |items: rhai::Array| {
        let nav: Vec<Nav> = items.iter().filter_map(|v| {
            let s = v.clone().into_string().ok()?;
            let parts: Vec<&str> = s.splitn(3, ':').collect();
            match parts.as_slice() {
                [id, label, icon] => Some(Nav::new(*id, *label, *icon)),
                [id, label]       => Some(Nav::new(*id, *label, "list")),
                _                 => None,
            }
        }).collect();

        match AppDsl::builder("").nav(nav).build() {
            Ok(dsl)   => a.borrow().apply_dsl(&dsl),
            Err(errs) => {
                for e in &errs { eprintln!("[dsl] {e}"); }
            }
        }
    });
}
