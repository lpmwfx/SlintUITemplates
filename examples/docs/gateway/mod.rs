//! Gateway — disk IO for the docs example.

/// Read a markdown file from disk. Returns `None` if the file cannot be read.
pub fn read_md(path: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
