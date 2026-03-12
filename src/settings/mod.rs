pub mod apply;

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub zoom: ZoomSettings,
    #[serde(default)]
    pub theme: ThemeSettings,
    #[serde(default)]
    pub icons: IconSettings,
    #[serde(default)]
    pub font: FontSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomSettings {
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub mode: ThemeMode,
    /// Optional hex accent override, e.g. "#ff6b35"
    pub accent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IconStyle {
    Filled,
    Outlined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSettings {
    pub style: IconStyle,
    /// "theme" | "accent" | "#rrggbb"
    pub color: String,
}

impl IconSettings {
    pub fn style_str(&self) -> &'static str {
        match self.style {
            IconStyle::Filled   => "filled",
            IconStyle::Outlined => "outlined",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSettings {
    /// None = platform default; Some = system font family name
    pub family: Option<String>,
    /// Independent font-only scale factor on top of zoom (default 1.0)
    pub font_scale: f32,
}

// --- Defaults ---

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

// --- Load / Save ---

impl AppSettings {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    /// Load from file, falling back to defaults if the file is absent or unparseable.
    pub fn from_file_or_default(path: &Path) -> Self {
        Self::from_file(path).unwrap_or_default()
    }

    pub fn to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_defaults() {
        let s = AppSettings::default();
        assert_eq!(s.zoom.scale, 1.0);
        assert_eq!(s.theme.mode, ThemeMode::System);
        assert!(s.theme.accent.is_none());
        assert_eq!(s.icons.style, IconStyle::Filled);
        assert_eq!(s.icons.color, "theme");
        assert!(s.font.family.is_none());
        assert_eq!(s.font.font_scale, 1.0);
    }

    #[test]
    fn settings_toml_round_trip() {
        let original = AppSettings {
            zoom:  ZoomSettings { scale: 1.5 },
            theme: ThemeSettings { mode: ThemeMode::Dark, accent: Some("#ff6b35".into()) },
            icons: IconSettings { style: IconStyle::Outlined, color: "accent".into() },
            font:  FontSettings { family: Some("Segoe UI".into()), font_scale: 1.1 },
        };
        let toml_str = toml::to_string_pretty(&original).unwrap();
        let loaded: AppSettings = toml::from_str(&toml_str).unwrap();

        assert!((loaded.zoom.scale - 1.5).abs() < 0.001);
        assert_eq!(loaded.theme.mode, ThemeMode::Dark);
        assert_eq!(loaded.theme.accent, Some("#ff6b35".into()));
        assert_eq!(loaded.icons.style, IconStyle::Outlined);
        assert_eq!(loaded.icons.color, "accent");
        assert_eq!(loaded.font.family, Some("Segoe UI".into()));
        assert!((loaded.font.font_scale - 1.1).abs() < 0.001);
    }

    #[test]
    fn settings_file_round_trip() {
        let path = std::env::temp_dir().join("slint_ui_test_settings.toml");
        let original = AppSettings {
            zoom: ZoomSettings { scale: 0.8 },
            ..AppSettings::default()
        };
        original.to_file(&path).unwrap();
        let loaded = AppSettings::from_file(&path).unwrap();
        assert!((loaded.zoom.scale - 0.8).abs() < 0.001);
    }
}
