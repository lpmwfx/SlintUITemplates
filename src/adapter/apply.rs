use std::path::Path;
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

/// # Example
/// ```rust,no_run
/// # let app = slint_ui_templates::AppAdapter::new().unwrap();
/// use slint_ui_templates::settings::AppSettings;
/// let settings = AppSettings::default();
/// app.apply_settings(&settings);
/// app.apply_theme();
/// ```
impl super::AppAdapter_adp {
    /// Apply settings to the Slint UI — also caches zoom scale.
    pub fn apply_settings(&self, settings: &crate::settings::AppSettings) {
        *self.zoom.borrow_mut() = settings.zoom.scale;
        crate::settings::apply::apply(&self.ui, settings);
    }

    /// Sync UI theme with OS dark mode preference.
    pub fn apply_theme(&self) {
        self.set_dark_mode(crate::pal::is_dark_mode());
    }

    /// Load grid layout from config file, apply row ratios, and update cache.
    pub fn apply_grid(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let zones = grid::load_target(path)?;
        for row in &zones.rows {
            match RowName_adp::from_str(&row.name) {
                Some(RowName_adp::Top) => {
                    *self.row_top_ratio.borrow_mut() = row.ratio as f32;
                    self.ui.set_row_top_ratio(row.ratio as f32);
                }
                Some(RowName_adp::Main) => {
                    *self.row_main_ratio.borrow_mut() = row.ratio as f32;
                    self.ui.set_row_main_ratio(row.ratio as f32);
                }
                Some(RowName_adp::Bottom) => self.ui.set_row_bottom_ratio(row.ratio as f32),
                None => {}
            }
        }
        Ok(())
    }
}
