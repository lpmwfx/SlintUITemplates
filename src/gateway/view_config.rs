use std::collections::HashMap;
use std::path::Path;
use crate::view_config::{ViewConfig, ViewConfigError};

/// Load and evaluate a single `.rhai` file via the gateway.
pub fn eval_file(path: &Path) -> Result<ViewConfig, ViewConfigError> {
    let script = std::fs::read_to_string(path)?;
    crate::view_config::eval_script(&script)
}

/// Scan `dir/*.rhai` and return a map of `{stem → ViewConfig}`.
/// Missing or empty directory is not an error — returns an empty map.
pub fn load_all(dir: &Path) -> Result<HashMap<String, ViewConfig>, ViewConfigError> {
    let mut map = HashMap::new();
    if !dir.exists() {
        return Ok(map);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("rhai") {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if !stem.is_empty() {
                map.insert(stem, eval_file(&path)?);
            }
        }
    }
    Ok(map)
}
