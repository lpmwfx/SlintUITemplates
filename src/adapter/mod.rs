use std::path::Path;
use slint::ComponentHandle;
use crate::{AppWindow, Colors};
use crate::grid;
use crate::dsl::AppDsl;

pub struct AppAdapter {
    ui: AppWindow,
}

impl AppAdapter {
    pub fn new() -> Result<Self, slint::PlatformError> {
        let ui = AppWindow::new()?;
        Ok(Self { ui })
    }

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

    /// Apply a validated `AppDsl` — composition rules enforced before this call.
    pub fn apply_dsl(&self, dsl: &AppDsl) {
        crate::dsl::apply::apply(&self.ui, dsl);
    }

    pub fn run(self) -> Result<(), slint::PlatformError> {
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
}
