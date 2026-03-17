//! Consumer integration tests — verify that consumers can discover and use the library.

use std::path::Path;

#[test]
fn components_slint_exists_in_ui_dir() {
    assert!(
        Path::new("ui/components.slint").exists(),
        "ui/components.slint must exist for consumer imports"
    );
}

#[test]
fn lib_slint_exists_in_ui_dir() {
    assert!(
        Path::new("ui/lib.slint").exists(),
        "ui/lib.slint must exist as the library entry point"
    );
}

#[test]
fn scaffold_generates_build_rs() {
    let dir = std::env::temp_dir().join("slintui_test_scaffold");
    // Clean up any previous run
    let _ = std::fs::remove_dir_all(&dir);

    let name = dir.file_name().map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    // Run scaffold in parent of target dir
    let parent = dir.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let original = std::env::current_dir().unwrap_or_default();
    std::env::set_current_dir(&parent).ok();
    let result = slint_ui_templates::gateway::scaffold::scaffold(&name);
    std::env::set_current_dir(&original).ok();

    assert!(result.is_ok(), "scaffold failed: {:?}", result.err());

    // Verify build.rs was generated and contains DEP_ env var
    let build_rs = dir.join("build.rs");
    assert!(build_rs.exists(), "build.rs not generated");
    let content = std::fs::read_to_string(&build_rs).unwrap_or_default();
    assert!(
        content.contains("DEP_SLINT_UI_TEMPLATES_SLINT_INCLUDE_PATH"),
        "build.rs must reference DEP_ env var"
    );

    // Verify ui/app.slint was generated
    let app_slint = dir.join("ui").join("app.slint");
    assert!(app_slint.exists(), "ui/app.slint not generated");

    // Clean up
    let _ = std::fs::remove_dir_all(&dir);
}
