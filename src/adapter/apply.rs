use std::path::Path;
use slint::ComponentHandle;
use crate::Theme;
use crate::grid;

/// Grid row name — typed discriminator for row ratio assignment.
enum RowName_adp {
    Top,
    Main,
    Bottom,
}

impl RowName_adp {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            s if s.eq_ignore_ascii_case("top")    => Some(RowName_adp::Top),
            s if s.eq_ignore_ascii_case("main")   => Some(RowName_adp::Main),
            s if s.eq_ignore_ascii_case("bottom") => Some(RowName_adp::Bottom),
            _ => None,
        }
    }
}

impl super::AppAdapter {
    /// Apply settings to the Slint UI (zoom, theme, icons, font).
    pub fn apply_settings(&self, settings: &crate::settings::AppSettings) {
        crate::settings::apply::apply(&self.ui, settings);
    }

    /// Sync UI theme with OS dark mode preference.
    pub fn apply_theme(&self) {
        self.ui.global::<Theme>().set_dark(crate::pal::is_dark_mode());
    }

    /// Load grid layout from config file and apply row ratios.
    pub fn apply_grid(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let zones = grid::load_target(path)?;
        for row in &zones.rows {
            match RowName_adp::from_str(&row.name) {
                Some(RowName_adp::Top)    => self.ui.set_row_top_ratio(row.ratio as f32),
                Some(RowName_adp::Main)   => self.ui.set_row_main_ratio(row.ratio as f32),
                Some(RowName_adp::Bottom) => self.ui.set_row_bottom_ratio(row.ratio as f32),
                None => {}
            }
        }
        Ok(())
    }
}
