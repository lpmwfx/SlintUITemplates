pub mod config;
pub mod zone;

pub use config::TargetConfig;
pub use zone::ZoneModel;

use std::path::Path;

pub fn load_target(path: &Path) -> Result<ZoneModel, Box<dyn std::error::Error>> {
    let config = TargetConfig::load(path)?;
    Ok(ZoneModel::from_config(&config.grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_desktop_target() {
        let zones = load_target(Path::new("config/desktop.toml")).unwrap();
        assert_eq!(zones.rows.len(), 3);
        assert_eq!(zones.total_row_ratio(), 12);
    }
}
