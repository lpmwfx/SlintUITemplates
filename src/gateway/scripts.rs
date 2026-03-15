use std::path::Path;

/// Load a Rhai script file from disk.
pub fn load_script(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
