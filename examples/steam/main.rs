// examples/steam — Steam Deck / small-tier shell demo.
// Platform: SteamDeck — compact nav, 56px gamepad-safe targets, Steam tokens.
// Run: cargo run --example steam

use slint_ui_templates::{AppAdapter, Platform, dsl::{AppDsl, Nav}};

/// Steam Deck native display width.
const STEAM_DECK_WIDTH:  u32 = 1280;
/// Steam Deck native display height.
const STEAM_DECK_HEIGHT: u32 = 800;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dsl = AppDsl::builder("Steam Demo")
        .platform(Platform::SteamDeck)
        .window_size(STEAM_DECK_WIDTH, STEAM_DECK_HEIGHT)
        .nav(vec![
            Nav::new("library", "Library", "folder"),
            Nav::new("browse",  "Browse",  "apps"),
            Nav::new("friends", "Friends", "people"),
            Nav::new("settings","Settings","settings"),
        ])
        .status("Steam Deck")
        .build()
        .map_err(|errs| errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))?;

    let mut adapter = AppAdapter::new()?;
    adapter.apply_dsl(&dsl);
    adapter.run()?;
    Ok(())
}
