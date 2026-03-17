// examples/macos — macOS HIG shell demo.
// Platform: MacOS — sidebar nav, SF Pro fonts, AppKit control sizing.
// Run: cargo run --example macos

use slint_ui_templates::{AppAdapter, Platform, dsl::{AppDsl, Nav}};

/// Standard MacBook / iMac display width.
const MACOS_WIDTH:  u32 = 1440;
/// Standard MacBook / iMac display height.
const MACOS_HEIGHT: u32 = 900;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dsl = AppDsl::builder("macOS Demo")
        .platform(Platform::MacOS)
        .window_size(MACOS_WIDTH, MACOS_HEIGHT)
        .nav(vec![
            Nav::new("home",     "Home",     "home"),
            Nav::new("browse",   "Browse",   "folder"),
            Nav::new("search",   "Search",   "search"),
            Nav::new("settings", "Settings", "settings"),
        ])
        .status("Apple HIG")
        .build()
        .map_err(|errs| errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))?;

    let mut adapter = AppAdapter::new()?;
    adapter.apply_dsl(&dsl);
    adapter.run()?;
    Ok(())
}
