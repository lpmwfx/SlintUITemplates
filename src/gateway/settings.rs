use std::path::Path;
use crate::settings::AppSettings;

/// Deserializes settings from a TOML file at the given path.
pub fn from_file(path: &Path) -> Result<AppSettings, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&text)?)
}

/// Load from file, falling back to defaults if the file is absent or unparseable.
pub fn from_file_or_default(path: &Path) -> AppSettings {
    from_file(path).unwrap_or_default()
}

/// Serializes settings to a TOML file, creating parent directories if needed.
pub fn to_file(settings: &AppSettings, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, toml::to_string_pretty(settings)?)?;
    Ok(())
}
