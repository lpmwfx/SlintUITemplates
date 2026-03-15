/// Applies loaded settings to the live Slint UI globals.
pub mod apply;
/// Default implementations for all settings structs.
mod defaults;

use serde::{Deserialize, Serialize};

/// Top-level application settings loaded from a TOML configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A pp se tt in gs struct.
pub struct AppSettings {
    #[serde(default)]
    /// Global UI zoom multiplier applied to layout and token scaling.
    pub zoom: ZoomSettings,
    #[serde(default)]
    /// Theme mode and accent override settings.
    pub theme: ThemeSettings,
    #[serde(default)]
    /// Icon style and colour strategy settings.
    pub icons: IconSettings,
    #[serde(default)]
    /// Font family and font-only scaling settings.
    pub font: FontSettings,
}

/// UI zoom / DPI-scale configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Z oo ms et ti ng s struct.
pub struct ZoomSettings {
    /// UI scale factor where `1.0` keeps the default logical size.
    pub scale: f32,
}

/// Light, dark, or system-detected colour-scheme selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
/// T he me mo de enum.
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

/// Theme configuration: colour-scheme mode and optional accent override.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// T he me se tt in gs struct.
pub struct ThemeSettings {
    /// Requested theme mode or OS-following behaviour.
    pub mode: ThemeMode,
    /// Optional hex accent override, e.g. "#ff6b35"
    pub accent: Option<String>,
}

/// Filled or outlined icon variant for the Fluent icon set.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
/// I co ns ty le enum.
pub enum IconStyle {
    Filled,
    Outlined,
}

/// Icon appearance configuration: style variant and colour strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// I co ns et ti ng s struct.
pub struct IconSettings {
    /// Icon glyph family variant to use in the UI.
    pub style: IconStyle,
    /// "theme" | "accent" | "#rrggbb"
    pub color: String,
}

/// Font family and scale configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// F on ts et ti ng s struct.
pub struct FontSettings {
    /// None = platform default; Some = system font family name
    pub family: Option<String>,
    /// Independent font-only scale factor on top of zoom (default 1.0)
    pub font_scale: f32,
}

// --- Load / Save (delegated to gateway::settings) ---

impl AppSettings {
    /// Deserializes settings from a TOML file at the given path.
    pub fn from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        crate::gateway::settings::from_file(path)
    }

    /// Load from file, falling back to defaults if the file is absent or unparseable.
    pub fn from_file_or_default(path: &std::path::Path) -> Self {
        crate::gateway::settings::from_file_or_default(path)
    }

    /// Serializes settings to a TOML file, creating parent directories if needed.
    pub fn to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        crate::gateway::settings::to_file(self, path)
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
