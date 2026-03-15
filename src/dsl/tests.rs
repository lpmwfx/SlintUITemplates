use super::*;

fn three_nav() -> Vec<Nav> {
    vec![
        Nav::new("home",     "Home",     "home"),
        Nav::new("list",     "List",     "list"),
        Nav::new("settings", "Settings", "settings"),
    ]
}

#[test]
fn valid_dsl_builds() {
    let dsl = AppDsl::builder("My App").nav(three_nav()).build().unwrap();
    assert_eq!(dsl.nav.len(), 3);
    assert_eq!(dsl.nav[0].id, "home");
    assert_eq!(dsl.nav[0].icon_code, "\u{E80F}");
}

#[test]
fn empty_nav_is_error() {
    let errs = AppDsl::builder("App").build().unwrap_err();
    assert!(errs.contains(&DslError::NoNavItems));
}

#[test]
fn too_many_nav_desktop_is_error() {
    let nav = (0..8).map(|i| Nav::new(format!("id{i}"), format!("L{i}"), "home")).collect();
    let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
    assert!(errs.contains(&DslError::TooManyNavItems { max: 7, got: 8 }));
}

#[test]
fn too_many_nav_android_max_5() {
    let nav = (0..6).map(|i| Nav::new(format!("id{i}"), format!("L{i}"), "home")).collect();
    let errs = AppDsl::builder("App")
        .platform(Platform::Android).nav(nav).build().unwrap_err();
    assert!(errs.contains(&DslError::TooManyNavItems { max: 5, got: 6 }));
}

#[test]
fn unknown_icon_is_error() {
    let nav = vec![Nav::new("home", "Home", "not-an-icon")];
    let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
    assert!(matches!(&errs[0], DslError::UnknownIcon { name, .. } if name == "not-an-icon"));
}

#[test]
fn missing_nav_id_is_error() {
    let nav = vec![Nav::new("", "Home", "home")];
    let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
    assert!(errs.contains(&DslError::NavItemMissingId(0)));
}

#[test]
fn toolbar_resolves_icons() {
    let dsl = AppDsl::builder("App")
        .nav(three_nav())
        .toolbar(vec![Toolbar::new("save", "save", "Save file")])
        .build().unwrap();
    assert!(dsl.show_toolbar);
    assert_eq!(dsl.toolbar[0].icon_code, "\u{E74E}");
}

#[test]
fn toolbar_unknown_icon_is_error() {
    let errs = AppDsl::builder("App")
        .nav(three_nav())
        .toolbar(vec![Toolbar::new("x", "not-real", "Tip")])
        .build().unwrap_err();
    assert!(matches!(&errs[0], DslError::UnknownIcon { context: "toolbar", .. }));
}

#[test]
fn toolbar_missing_id_is_error() {
    let errs = AppDsl::builder("App")
        .nav(three_nav())
        .toolbar(vec![Toolbar::new("", "save", "Save")])
        .build().unwrap_err();
    assert!(errs.contains(&DslError::ToolbarItemMissingId(0)));
}

#[test]
fn window_size_stored() {
    let dsl = AppDsl::builder("App").nav(three_nav()).window_size(1100, 720).build().unwrap();
    assert_eq!(dsl.window_size, Some((1100, 720)));
}

#[test]
fn status_default_is_ready() {
    let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
    assert_eq!(dsl.status, "Ready");
}

#[test]
fn no_toolbar_hides_it() {
    let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
    assert!(!dsl.show_toolbar);
}

#[test]
fn views_matching_nav_is_ok() {
    let dsl = AppDsl::builder("App")
        .nav(three_nav())
        .views(vec!["home", "list", "settings"])
        .build();
    assert!(dsl.is_ok());
}

#[test]
fn nav_without_view_is_error() {
    let errs = AppDsl::builder("App")
        .nav(three_nav())
        .views(vec!["home", "list"])  // missing "settings"
        .build().unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, DslError::NavWithoutView(id) if id == "settings")));
}

#[test]
fn view_without_nav_is_error() {
    let errs = AppDsl::builder("App")
        .nav(three_nav())
        .views(vec!["home", "list", "settings", "extra"])
        .build().unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, DslError::ViewWithoutNav(id) if id == "extra")));
}

#[test]
fn bg_style_default_is_solid() {
    let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
    assert_eq!(dsl.bg_style, BgStyle::Solid);
}

#[test]
fn bg_style_mica_stored() {
    let dsl = AppDsl::builder("App")
        .nav(three_nav())
        .bg_style(BgStyle::Mica)
        .build().unwrap();
    assert_eq!(dsl.bg_style, BgStyle::Mica);
}

#[test]
fn bg_style_acrylic_stored() {
    let dsl = AppDsl::builder("App")
        .nav(three_nav())
        .bg_style(BgStyle::Acrylic)
        .build().unwrap();
    assert_eq!(dsl.bg_style, BgStyle::Acrylic);
}

#[test]
fn views_optional_no_check_when_omitted() {
    // No .views() call — nav↔view check skipped
    let dsl = AppDsl::builder("App").nav(three_nav()).build();
    assert!(dsl.is_ok());
}

#[test]
fn multiple_errors_collected() {
    let nav = vec![
        Nav::new("", "Home", "home"),      // missing id
        Nav::new("x", "X",   "bad-icon"),  // bad icon
    ];
    let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
    assert_eq!(errs.len(), 2);
}

// ── Platform tier tests ───────────────────────────────────────────────────────

#[test]
fn platform_steam_deck_as_str() {
    assert_eq!(Platform::SteamDeck.as_str(), "steam-deck");
}

#[test]
fn platform_steam_linux_as_str() {
    assert_eq!(Platform::SteamLinux.as_str(), "steam-linux");
}

#[test]
fn platform_steam_deck_is_small() {
    assert!(Platform::SteamDeck.is_small());
    assert!(!Platform::SteamDeck.is_mobile());
    assert!(!Platform::SteamDeck.is_desktop());
}

#[test]
fn platform_steam_linux_is_desktop() {
    assert!(Platform::SteamLinux.is_desktop());
    assert!(!Platform::SteamLinux.is_small());
    assert!(!Platform::SteamLinux.is_mobile());
}

#[test]
fn dsl_steam_deck_nav_limit_five() {
    // SteamDeck = small tier, same 5-item cap as mobile
    let nav = (0..6).map(|i| Nav::new(format!("id{i}"), format!("L{i}"), "home")).collect();
    let errs = AppDsl::builder("App")
        .platform(Platform::SteamDeck).nav(nav).build().unwrap_err();
    assert!(errs.contains(&DslError::TooManyNavItems { max: 5, got: 6 }));
}

#[test]
fn dsl_steam_linux_nav_limit_seven() {
    // SteamLinux = desktop tier, allows up to 7 nav items
    let nav = (0..7).map(|i| Nav::new(format!("id{i}"), format!("L{i}"), "home")).collect();
    let build_result = AppDsl::builder("App")
        .platform(Platform::SteamLinux).nav(nav).build();
    assert!(build_result.is_ok());
}
