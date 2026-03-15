use super::*;
use std::path::Path;

fn init() {
    i_slint_backend_testing::init_no_event_loop();
}

#[test]
fn active_view_round_trip() {
    init();
    let adapter = AppAdapter_adp::new().unwrap();
    adapter.set_active_view("settings");
    assert_eq!(adapter.get_active_view(), "settings");
}

#[test]
fn row_ratio_round_trip() {
    init();
    let adapter = AppAdapter_adp::new().unwrap();
    adapter.apply_grid(Path::new("config/desktop.toml")).unwrap();
    assert_eq!(adapter.get_row_top_ratio(), 1.0);
}

#[test]
fn apply_settings_pushes_zoom() {
    init();
    let adapter = AppAdapter_adp::new().unwrap();
    let mut s = crate::settings::AppSettings::default();
    s.zoom.scale = 1.5;
    adapter.apply_settings(&s);
    assert!((adapter.get_zoom() - 1.5).abs() < 0.001);
}

#[test]
fn apply_settings_accent_override() {
    use slint::ComponentHandle;
    init();
    let adapter = AppAdapter_adp::new().unwrap();
    let mut s = crate::settings::AppSettings::default();
    s.theme.accent = Some("#ff6b35".into());
    s.theme.mode = crate::settings::ThemeMode::Dark;
    adapter.apply_settings(&s);
    let color = adapter.ui.global::<crate::Theme>().get_accent_override();
    assert_eq!(color.red(), 255);
    assert_eq!(color.green(), 107);
    assert_eq!(color.blue(), 53);
}

#[test]
fn apply_grid_loads_three_rows() {
    init();
    let adapter = AppAdapter_adp::new().unwrap();
    adapter.apply_grid(Path::new("config/desktop.toml")).unwrap();
    assert_eq!(adapter.get_row_main_ratio(), 10.0);
}

#[test]
fn apply_dsl_sets_nav_and_status() {
    use crate::dsl::{AppDsl, Nav};
    init();
    let mut adapter = AppAdapter_adp::new().unwrap();
    let dsl = AppDsl::builder("Test App")
        .nav(vec![
            Nav::new("home", "Home", "home"),
            Nav::new("list", "List", "list"),
        ])
        .status("Connected")
        .build()
        .unwrap();
    adapter.apply_dsl(&dsl);
    assert_eq!(adapter.get_status(), "Connected");
}

#[test]
fn menu_action_dispatch() {
    use std::sync::{Arc, Mutex};
    init();
    let adapter = AppAdapter_adp::new().unwrap();
    // REASON: Closure inside on_menu_action needs shared mutable access across thread boundary
    let called = Arc::new(Mutex::new(false));
    let c = Arc::clone(&called);
    adapter.on_menu_action("file.new", move || { *c.lock().unwrap() = true; });
    adapter.ui.invoke_menu_action("file.new".into());
    assert!(*called.lock().unwrap());
}

#[test]
fn view_config_auto_applied_on_navigate() {
    use crate::view_config::ViewConfig;
    init();
    let adapter = AppAdapter_adp::new().unwrap();
    let mut cfg = ViewConfig::default();
    cfg.status = Some("Test Status".into());
    adapter.view_configs.borrow_mut().insert("home".into(), cfg);
    adapter.ui.invoke_navigate("home".into());
    assert_eq!(adapter.get_status(), "Test Status");
}
