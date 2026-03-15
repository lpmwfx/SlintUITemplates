use std::path::Path;
use crate::grid::config::TargetConfig;

/// Reads and deserializes a target configuration from a TOML file.
pub fn load(path: &Path) -> Result<TargetConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: TargetConfig = toml::from_str(&content)?;
    Ok(config)
}
