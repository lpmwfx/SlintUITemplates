use super::{AppSettings, ZoomSettings, ThemeSettings, ThemeMode, IconSettings, IconStyle, FontSettings};

impl IconStyle {
    /// Parse from string — unknown values default to `Filled`.
    pub fn from_str(s: &str) -> Self {
        if s.eq_ignore_ascii_case("outlined") { IconStyle::Outlined } else { IconStyle::Filled }
    }
}

impl IconSettings {
    /// Returns the icon style as a lowercase string slice (`"filled"` or `"outlined"`).
    pub fn style_str(&self) -> &'static str {
        match self.style { IconStyle::Filled => "filled", IconStyle::Outlined => "outlined" }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            zoom:  ZoomSettings::default(),
            theme: ThemeSettings::default(),
            icons: IconSettings::default(),
            font:  FontSettings::default(),
        }
    }
}

impl Default for ZoomSettings {
    fn default() -> Self { Self { scale: 1.0 } }
}

impl Default for ThemeSettings {
    fn default() -> Self { Self { mode: ThemeMode::System, accent: None } }
}

impl Default for IconSettings {
    fn default() -> Self { Self { style: IconStyle::Filled, color: "theme".into() } }
}

impl Default for FontSettings {
    fn default() -> Self { Self { family: None, font_scale: 1.0 } }
}
