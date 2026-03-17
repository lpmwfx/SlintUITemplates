# Changelog

## [Unreleased] — 0.1.0

### Phase 22 — Workspace Split
- Flat workspace: root = library crate, `tools/` = slintui + docgen + scanners (publish = false)
- Feature flags: `rhai` (optional Rhai scripting), `markdown` (optional docs module)
- `tiny-skia` moved to dev-dependencies (unused by library)
- Scanner build-deps removed from root — now in `tools/scanners`
- CI updated for workspace builds + no-default-features check
- Publish workflow targets `-p slint-ui-templates` specifically

### Phase 12 — Publish
- README with architecture diagram, quick-start, AppShell declaration example
- docs/architecture.md — mother-child pattern, token system, AppShell flow
- docs/tokens.md — all token namespaces and override patterns
- docs/platform-rules.md — placement rules per platform
- CI pipeline (GitHub Actions — build + test on push/PR)
- src/lib.rs — rustdoc from README

### Phase 11 — Android Material Shell
- `ui/tokens/material/` — Material Design 3 token globals (colors, spacing, radius, type, elevation)
- `ui/shell/mobile/shell.slint` — MobileShell: TopAppBar + content + BottomNavBar + FAB
- `ui/shell/mobile/top-app-bar.slint` — CenterAligned TopAppBar
- `ui/shell/mobile/bottom-nav-bar.slint` — NavigationBar with active indicator pill animation
- `ui/shell/mobile/fab.slint` — FloatingActionButton (simple + extended)
- `examples/mobile/main.rs` — `cargo run --example mobile`
- Platform enum: `is_mobile()` / `is_desktop()` helpers

### Phase 10 — Bi-directional Layout Engine
- `src/layout/ratio_solver.rs` — `drag(panels, handle_idx, delta)` zero-sum transfer
- `src/layout/dsl_v2.rs` — named-slot DSL: `"sidebar(0.2):content:inspector(0.25)"`
- `src/layout/constraints.rs` — min/max ratio constraints
- `build_v2()` / `drag()` / `normalize()` in public API
- DragHandle: added `drag-end(dx, dy)` callback
- page-layout.slint: live 3-panel bi-directional drag demo

### Phase 9 — WinUI AppShell
- `ui/shell/app-shell.slint` — AppShell (single entry point, @children content)
- `ui/shell/desktop/shell.slint` — DesktopShell: MenuBar + Toolbar + SideNav + Content + StatusBar
- `ui/shell/desktop/menu-bar.slint` — Fluent 2 styled menu bar
- `ui/shell/desktop/tool-bar.slint` — optional icon toolbar
- `ui/shell/desktop/side-nav.slint` — NavigationView (collapsible, animated width)
- `ui/shell/desktop/content-frame.slint` — @children content area
- `ui/shell/desktop/status-bar.slint` — Fluent 2 status bar + progress
- `src/shell/mod.rs` — ShellConfig, NavItemConfig, MenuConfig, ToolbarItemConfig
- `src/shell/platform.rs` — Platform enum (Windows, Android)
- page-app-shell.slint: uses real AppShell + real MobileShell

### Phase 8 — Fluent Foundation
- `ui/tokens/fluent/` — exact Fluent 2 / WinUI3 token globals (7 files + mod)
- `ui/tokens/fluent/icons.slint` — FluentIcons global (~60 Segoe Fluent codepoints)
- Colors updated to exact Fluent 2 light/dark values
- Spacing: added control-sm/md/lg height tokens
- theme.slint: exports all Fluent + Material globals
- All emoji replaced with FluentIcons codepoints; font-family: Segoe Fluent Icons
- page-icons.slint: Fluent icon catalog (7th viewer page)
- rules/winui/: placement, fluent2-tokens, fluent-icons rule files

### Phase 7 — Framework Viewer
- `ui/viewer/` — FrameworkViewer window: sidebar + topbar, 6 pages
- All 11 widgets showcased with variants and states
- `examples/viewer/main.rs` — `cargo run --example viewer`
- Latent bug fixes: toast/dialog visible clash, missing Colors.shadow

### Phase 6 — Settings System
- `src/settings/` — AppSettings (zoom, theme, icons, font) + TOML round-trip
- `src/settings/apply.rs` — apply_settings() pushes to Slint globals
- `ui/state/settings.slint` — Settings global
- `ui/tokens/scale.slint` — icon sizes reactive to zoom
- `ui/tokens/typography.slint` — font sizes reactive to zoom + font-scale
- Rhai API: set_zoom(), set_icon_style(), set_icon_color(), set_font(), set_font_scale()

### Phase 5 — Rhai Scripting
- `src/bindings/rhai/` — Rhai engine with full UI API
- `examples/rhai/` — demo script driving UI state

### Phase 4 — Rust Adapter
- `src/adapter/mod.rs` — AppAdapter struct
- Test suite (20 tests)
- `examples/desktop/main.rs`

### Phase 3 — Modules
- Avatar, SelectField, ProgressBar, DragHandle, PanelContainer, PanelSlot

### Phase 2 — Views
- Desktop mother with SideBar, NavBar, ContentArea, StatusBar
- Mobile mother with tab bar

### Phase 1 — Foundation
- Token system, AppWindow, grid engine, layout DSL
