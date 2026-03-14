use serde::Deserialize;
use std::path::Path;

/// Top-level configuration for a deployment target (e.g. desktop, tablet).
#[derive(Debug, Deserialize)]
pub struct TargetConfig {
    pub target: TargetInfo,
    pub grid: GridConfig,
}

/// Metadata describing a target's name, screen class, and interaction model.
#[derive(Debug, Deserialize)]
pub struct TargetInfo {
    pub name: String,
    pub screen: String,
    pub interaction: String,
}

/// Grid layout definition containing the ordered list of row configurations.
#[derive(Debug, Deserialize)]
pub struct GridConfig {
    pub rows: Vec<RowConfig>,
}

/// Configuration for a single grid row, optionally fixed or subdivided into columns.
#[derive(Debug, Deserialize)]
pub struct RowConfig {
    pub name: String,
    pub ratio: u32,
    pub fixed: Option<String>,
    #[serde(default)]
    pub columns: Vec<ColumnConfig>,
}

/// Configuration for a single column within a grid row.
#[derive(Debug, Deserialize)]
pub struct ColumnConfig {
    pub name: String,
    pub ratio: u32,
}

impl TargetConfig {
    /// Reads and deserializes a target configuration from a TOML file.
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: TargetConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_desktop_config() {
        let config = TargetConfig::load(Path::new("config/desktop.toml")).unwrap();

        assert_eq!(config.target.name, "desktop");
        assert_eq!(config.grid.rows.len(), 3);

        // top row: fixed nav-bar
        assert_eq!(config.grid.rows[0].name, "top");
        assert_eq!(config.grid.rows[0].ratio, 1);
        assert_eq!(config.grid.rows[0].fixed.as_deref(), Some("nav-bar"));
        assert!(config.grid.rows[0].columns.is_empty());

        // main row: 3 columns
        assert_eq!(config.grid.rows[1].name, "main");
        assert_eq!(config.grid.rows[1].columns.len(), 3);
        assert_eq!(config.grid.rows[1].columns[0].name, "left");
        assert_eq!(config.grid.rows[1].columns[0].ratio, 3);
        assert_eq!(config.grid.rows[1].columns[1].name, "center");
        assert_eq!(config.grid.rows[1].columns[1].ratio, 4);
        assert_eq!(config.grid.rows[1].columns[2].name, "right");
        assert_eq!(config.grid.rows[1].columns[2].ratio, 3);

        // bottom row: fixed status-bar
        assert_eq!(config.grid.rows[2].name, "bottom");
        assert_eq!(config.grid.rows[2].fixed.as_deref(), Some("status-bar"));
    }
}
