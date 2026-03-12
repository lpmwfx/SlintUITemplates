use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use slint::ComponentHandle;
use crate::{AppWindow, Colors};
use crate::grid;
use crate::dsl::{AppDsl, BgStyle};

pub struct AppAdapter {
    ui:           AppWindow,
    menu_actions: Rc<RefCell<HashMap<String, Box<dyn Fn()>>>>,
    bg_style:     BgStyle,
    view_configs: Rc<RefCell<HashMap<String, crate::view_config::ViewConfig>>>,
    navigate_cb:  Rc<RefCell<Option<Box<dyn Fn(slint::SharedString)>>>>,
}

impl AppAdapter {
    pub fn new() -> Result<Self, slint::PlatformError> {
        let ui = AppWindow::new()?;
        let menu_actions: Rc<RefCell<HashMap<String, Box<dyn Fn()>>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let view_configs: Rc<RefCell<HashMap<String, crate::view_config::ViewConfig>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let navigate_cb: Rc<RefCell<Option<Box<dyn Fn(slint::SharedString)>>>> =
            Rc::new(RefCell::new(None));

        // Wire menu-action dispatch once — all on_menu_action() calls add to this table
        let actions = Rc::clone(&menu_actions);
        ui.on_menu_action(move |id| {
            if let Some(f) = actions.borrow().get(id.as_str()) {
                f();
            }
        });

        // Wire navigate once — auto-applies ViewConfig then calls user callback
        let vc      = Rc::clone(&view_configs);
        let nav_cb  = Rc::clone(&navigate_cb);
        let ui_nav  = ui.clone_strong();
        ui.on_navigate(move |id| {
            if let Some(cfg) = vc.borrow().get(id.as_str()) {
                crate::view_config::apply(&ui_nav, cfg);
            }
            if let Some(ref f) = *nav_cb.borrow() { f(id); }
        });

        Ok(Self { ui, menu_actions, bg_style: BgStyle::Solid, view_configs, navigate_cb })
    }

    // ── DSL ──────────────────────────────────────────────────────────────────

    /// Apply a validated `AppDsl` — composition rules already enforced.
    pub fn apply_dsl(&mut self, dsl: &AppDsl) {
        self.bg_style = dsl.bg_style;
        crate::dsl::apply::apply(&self.ui, dsl);
    }

    // ── View config ───────────────────────────────────────────────────────────

    /// Load all `*.rhai` files from `dir` and register as per-view configs.
    /// Auto-applied by the navigate handler on every nav event.
    pub fn load_view_configs(&self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let configs = crate::view_config::load_all(dir)?;
        *self.view_configs.borrow_mut() = configs;
        Ok(())
    }

    // ── Window ────────────────────────────────────────────────────────────────

    /// Resize the window (logical pixels).
    pub fn set_window_size(&self, width: u32, height: u32) {
        self.ui.window().set_size(slint::PhysicalSize::new(width, height));
    }

    /// Set the OS-level backdrop style ("mica" | "acrylic" | anything else → solid).
    pub fn set_bg_style_str(&mut self, style: &str) {
        self.bg_style = match style {
            "mica"    => BgStyle::Mica,
            "acrylic" => BgStyle::Acrylic,
            _         => BgStyle::Solid,
        };
    }

    // ── Callback wiring ───────────────────────────────────────────────────────

    /// Register a handler for the navigate callback (nav item clicked).
    /// Replaces any previously registered handler.
    pub fn on_navigate(&self, f: impl Fn(slint::SharedString) + 'static) {
        *self.navigate_cb.borrow_mut() = Some(Box::new(f));
    }

    /// Register a handler for toolbar button clicks.
    pub fn on_toolbar_clicked(&self, f: impl Fn(slint::SharedString) + 'static) {
        self.ui.on_toolbar_clicked(f);
    }

    /// Register a handler for a specific MenuBar action id.
    /// Multiple calls accumulate — each id maps to one handler.
    pub fn on_menu_action(&self, id: impl Into<String>, f: impl Fn() + 'static) {
        self.menu_actions.borrow_mut().insert(id.into(), Box::new(f));
    }

    // ── State setters ─────────────────────────────────────────────────────────

    pub fn apply_settings(&self, settings: &crate::settings::AppSettings) {
        crate::settings::apply::apply(&self.ui, settings);
    }

    pub fn apply_theme(&self) {
        self.ui.global::<Colors>().set_dark_mode(is_dark_mode());
    }

    pub fn apply_grid(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let zones = grid::load_target(path)?;
        for row in &zones.rows {
            match row.name.as_str() {
                "top"    => self.ui.set_row_top_ratio(row.ratio as f32),
                "main"   => self.ui.set_row_main_ratio(row.ratio as f32),
                "bottom" => self.ui.set_row_bottom_ratio(row.ratio as f32),
                _ => {}
            }
        }
        Ok(())
    }

    pub fn set_active_view(&self, name: &str) {
        self.ui.set_active_view(name.into());
    }

    pub fn get_active_view(&self) -> String {
        self.ui.get_active_view().to_string()
    }

    pub fn set_dark_mode(&self, on: bool) {
        self.ui.global::<Colors>().set_dark_mode(on);
    }

    pub fn get_dark_mode(&self) -> bool {
        self.ui.global::<Colors>().get_dark_mode()
    }

    pub fn set_status(&self, text: &str) {
        self.ui.set_status_text(text.into());
    }

    pub fn run(self) -> Result<(), slint::PlatformError> {
        // Show the window first so the HWND is live before applying DWM backdrop.
        self.ui.show()?;
        crate::platform::apply_backdrop(&self.ui.window(), self.bg_style);
        self.ui.run()
    }
}

pub(crate) fn is_dark_mode() -> bool {
    std::process::Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "/v",
            "AppsUseLightTheme",
        ])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.contains("0x0"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        i_slint_backend_testing::init_no_event_loop();
    }

    #[test]
    fn active_view_round_trip() {
        init();
        let adapter = AppAdapter::new().unwrap();
        adapter.ui.set_active_view("settings".into());
        assert_eq!(adapter.ui.get_active_view(), "settings");
    }

    #[test]
    fn row_ratio_round_trip() {
        init();
        let adapter = AppAdapter::new().unwrap();
        adapter.ui.set_row_top_ratio(2.0);
        assert_eq!(adapter.ui.get_row_top_ratio(), 2.0);
    }

    #[test]
    fn apply_settings_pushes_zoom() {
        use slint::ComponentHandle;
        init();
        let adapter = AppAdapter::new().unwrap();
        let mut s = crate::settings::AppSettings::default();
        s.zoom.scale = 1.5;
        adapter.apply_settings(&s);
        let zoom = adapter.ui.global::<crate::Settings>().get_zoom();
        assert!((zoom - 1.5).abs() < 0.001);
    }

    #[test]
    fn apply_settings_accent_override() {
        use slint::ComponentHandle;
        init();
        let adapter = AppAdapter::new().unwrap();
        let mut s = crate::settings::AppSettings::default();
        s.theme.accent = Some("#ff6b35".into());
        s.theme.mode = crate::settings::ThemeMode::Dark;
        adapter.apply_settings(&s);
        let color = adapter.ui.global::<crate::Colors>().get_accent();
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 107);
        assert_eq!(color.blue(), 53);
    }

    #[test]
    fn apply_grid_loads_three_rows() {
        init();
        let adapter = AppAdapter::new().unwrap();
        adapter.apply_grid(Path::new("config/desktop.toml")).unwrap();
        assert_eq!(adapter.ui.get_row_main_ratio(), 10.0);
    }

    #[test]
    fn apply_dsl_sets_nav_and_status() {
        use crate::dsl::{AppDsl, Nav};
        init();
        let mut adapter = AppAdapter::new().unwrap();
        let dsl = AppDsl::builder("Test App")
            .nav(vec![
                Nav::new("home", "Home", "home"),
                Nav::new("list", "List", "list"),
            ])
            .status("Connected")
            .build()
            .unwrap();
        adapter.apply_dsl(&dsl);
        assert_eq!(adapter.ui.get_status_text(), "Connected");
    }

    #[test]
    fn menu_action_dispatch() {
        use std::sync::{Arc, Mutex};
        init();
        let adapter = AppAdapter::new().unwrap();
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
        let adapter = AppAdapter::new().unwrap();
        let mut cfg = ViewConfig::default();
        cfg.status = Some("Test Status".into());
        adapter.view_configs.borrow_mut().insert("home".into(), cfg);
        adapter.ui.invoke_navigate("home".into());
        assert_eq!(adapter.ui.get_status_text(), "Test Status");
    }
}
