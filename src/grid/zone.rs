use crate::grid::config::{GridConfig, RowConfig};

/// Runtime grid model holding resolved rows and their proportional ratios.
#[derive(Debug)]
/// Z on em od el struct.
pub struct ZoneModel {
    /// R ow s.
    pub rows: Vec<RowZone>,
    total_row_ratio: u32,
}

/// A resolved grid row with its name, ratio weight, and content kind.
#[derive(Debug)]
/// R ow zo ne struct.
pub struct RowZone {
    /// N am e.
    pub name: String,
    /// R at io.
    pub ratio: u32,
    /// K in d.
    pub kind: RowKind,
}

/// Describes how a row's content is arranged: fixed element, column split, or empty.
#[derive(Debug)]
/// R ow ki nd enum.
pub enum RowKind {
    Fixed { element: String },
    Columns(Vec<ColumnZone>),
    Empty,
}

/// A resolved column within a row, with an optional module assignment.
#[derive(Debug)]
/// C ol um nz on e struct.
pub struct ColumnZone {
    /// N am e.
    pub name: String,
    /// R at io.
    pub ratio: u32,
    /// M od ul e.
    pub module: Option<String>,
}

impl ZoneModel {
    /// Builds a `ZoneModel` from a parsed `GridConfig`.
    pub fn from_config(config: &GridConfig) -> Self {
        let total_row_ratio = config.rows.iter().map(|r| r.ratio).sum();
        let rows = config.rows.iter().map(RowZone::from_config).collect();
        Self {
            rows,
            total_row_ratio,
        }
    }

    /// Returns the sum of all row ratio weights in the grid.
    pub fn total_row_ratio(&self) -> u32 {
        self.total_row_ratio
    }

    /// Looks up a row by name, returning a reference if found.
    pub fn row(&self, name: &str) -> Option<&RowZone> {
        self.rows.iter().find(|r| r.name == name)
    }

    /// Looks up a column by row and column name, returning a reference if found.
    pub fn column(&self, row_name: &str, col_name: &str) -> Option<&ColumnZone> {
        self.row(row_name).and_then(|r| match &r.kind {
            RowKind::Columns(cols) => cols.iter().find(|c| c.name == col_name),
            _ => None,
        })
    }

    /// Assigns a module name to a specific column within a row.
    pub fn set_module(&mut self, row_name: &str, col_name: &str, module: &str) {
        if let Some(row) = self.rows.iter_mut().find(|r| r.name == row_name) {
            row.set_column_module(col_name, module);
        }
    }
}

impl RowZone {
    /// Sets the module for a column by name within this row.
    fn set_column_module(&mut self, col_name: &str, module: &str) {
        if let RowKind::Columns(cols) = &mut self.kind {
            if let Some(col) = cols.iter_mut().find(|c| c.name == col_name) {
                // REASON: owned String required — config structs use String fields
                col.module = Some(module.to_string());
            }
        }
    }

    // REASON: clone required — building owned zone model from borrowed config
    fn from_config(config: &RowConfig) -> Self {
        let kind = if let Some(fixed) = &config.fixed {
            RowKind::Fixed {
                element: fixed.to_owned(),
            }
        } else if !config.columns.is_empty() {
            RowKind::Columns(
                config
                    .columns
                    .iter()
                    .map(|c| ColumnZone {
                        name: c.name.clone(),
                        ratio: c.ratio,
                        module: None,
                    })
                    .collect(),
            )
        } else {
            RowKind::Empty
        };

        Self {
            name: config.name.clone(),
            ratio: config.ratio,
            kind,
        }
    }

    /// Returns the sum of all column ratio weights in this row, or 0 if not column-based.
    pub fn total_column_ratio(&self) -> u32 {
        match &self.kind {
            RowKind::Columns(cols) => cols.iter().map(|c| c.ratio).sum(),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::config::TargetConfig;
    use std::path::Path;

    #[test]
    fn zone_model_from_desktop_config() {
        let config = TargetConfig::load(Path::new("config/desktop.toml")).unwrap();
        let zones = ZoneModel::from_config(&config.grid);

        assert_eq!(zones.rows.len(), 3);
        assert_eq!(zones.total_row_ratio(), 12); // 1 + 10 + 1

        // top is fixed
        let top = zones.row("top").unwrap();
        assert!(matches!(&top.kind, RowKind::Fixed { element } if element == "nav-bar"));

        // main has 3 columns
        let main = zones.row("main").unwrap();
        assert_eq!(main.total_column_ratio(), 10); // 3 + 4 + 3

        // bottom is fixed
        let bottom = zones.row("bottom").unwrap();
        assert!(matches!(&bottom.kind, RowKind::Fixed { element } if element == "status-bar"));
    }

    #[test]
    fn set_module_in_zone() {
        let config = TargetConfig::load(Path::new("config/desktop.toml")).unwrap();
        let mut zones = ZoneModel::from_config(&config.grid);

        zones.set_module("main", "left", "file-tree");
        zones.set_module("main", "center", "code-editor");

        let left = zones.column("main", "left").unwrap();
        assert_eq!(left.module.as_deref(), Some("file-tree"));

        let center = zones.column("main", "center").unwrap();
        assert_eq!(center.module.as_deref(), Some("code-editor"));

        // right is still empty
        let right = zones.column("main", "right").unwrap();
        assert!(right.module.is_none());
    }
}
