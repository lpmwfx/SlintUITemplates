use serde::Deserialize;

/// Top-level configuration for a deployment target (e.g. desktop, tablet).
#[derive(Debug, Deserialize)]
/// T ar ge tc on fi g struct.
pub struct TargetConfig {
    /// Metadata describing the target device class and interaction model.
    pub target: TargetInfo,
    /// Grid rows that define the target's runtime shell layout.
    pub grid: GridConfig,
}

/// Metadata describing a target's name, screen class, and interaction model.
#[derive(Debug, Deserialize)]
/// T ar ge ti nf o struct.
pub struct TargetInfo {
    /// Stable target name used to select the configuration file.
    pub name: String,
    /// Screen-class label such as desktop, tablet, or mobile.
    pub screen: String,
    /// Primary interaction mode such as mouse or touch.
    pub interaction: String,
}

/// Grid layout definition containing the ordered list of row configurations.
#[derive(Debug, Deserialize)]
/// G ri dc on fi g struct.
pub struct GridConfig {
    /// Ordered top-to-bottom rows in the resolved application grid.
    pub rows: Vec<RowConfig>,
}

/// Configuration for a single grid row, optionally fixed or subdivided into columns.
#[derive(Debug, Deserialize)]
/// R ow co nf ig struct.
pub struct RowConfig {
    /// Stable row identifier used by the runtime zone model.
    pub name: String,
    /// Relative weight used when distributing row height.
    pub ratio: u32,
    /// Optional fixed module name rendered across the whole row.
    pub fixed: Option<String>,
    #[serde(default)]
    /// Optional column layout when the row is split horizontally.
    pub columns: Vec<ColumnConfig>,
}

/// Configuration for a single column within a grid row.
#[derive(Debug, Deserialize)]
/// C ol um nc on fi g struct.
pub struct ColumnConfig {
    /// Stable column identifier within its parent row.
    pub name: String,
    /// Relative weight used when distributing row width.
    pub ratio: u32,
}

impl TargetConfig {
    /// Reads and deserializes a target configuration from a TOML file (delegated to gateway).
    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        crate::gateway::grid::load(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

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
