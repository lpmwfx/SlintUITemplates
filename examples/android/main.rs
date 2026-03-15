// examples/android — Material 3 / Android shell demo.
// Platform: Android — bottom nav, 48px touch targets, Material tokens.
// Run: cargo run --example android

use slint_ui_templates::{AppAdapter, Platform, dsl::{AppDsl, Nav}};

/// Standard Android phone width (portrait).
const ANDROID_WIDTH: u32  = 390;
/// Standard Android phone height (portrait).
const ANDROID_HEIGHT: u32 = 844;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dsl = AppDsl::builder("Android Demo")
        .platform(Platform::Android)
        .window_size(ANDROID_WIDTH, ANDROID_HEIGHT)
        .nav(vec![
            Nav::new("home",     "Home",     "home"),
            Nav::new("search",   "Search",   "search"),
            Nav::new("library",  "Library",  "folder"),
            Nav::new("settings", "Settings", "settings"),
        ])
        .status("Material 3")
        .build()
        .map_err(|errs| errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))?;

    let mut adapter = AppAdapter::new()?;
    adapter.apply_dsl(&dsl);
    adapter.run()?;
    Ok(())
}
