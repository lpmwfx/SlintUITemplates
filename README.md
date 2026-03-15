# SlintUITemplates

A platform-native Slint UI shell library. Declare **what** you want — AppShell,
nav items, menus, content slots — and the framework delivers the correct
platform-stereotyped UI without any layout decisions from the caller.

**One declaration → one correct visual result per platform.**

```slint
AppShell {
    nav-items: [
        { id: "home",     label: "Home",     icon: FluentIcons.home },
        { id: "settings", label: "Settings", icon: FluentIcons.settings },
    ];
    menus: [
        { id: "file", label: "File" },
        { id: "edit", label: "Edit" },
    ];
    active-slot <=> root.active-view;
    slot-changed(id) => { root.active-view = id; }

    if root.active-view == "home":     MyHomeView     {}
    if root.active-view == "settings": MySettingsView {}
}
```

On Windows: **MenuBar → optional Toolbar → SideNav + Content → StatusBar** (Fluent 2)
On Android: **TopAppBar + Content + BottomNavBar + FAB** (Material 3) — Phase 11

---

## Quick Start

```sh
git clone https://github.com/lpmwfx/SlintUITemplates
cd SlintUITemplates
cargo run --example viewer   # browse all widgets
cargo run --example desktop  # full desktop shell
cargo run --example mobile   # Material 3 mobile demo
```

Or as a dependency:

```toml
[dependencies]
slint-ui-templates = "0.1"
```

---

## Architecture

```text
AppShell (Windows — Fluent 2)
├── ShellMenuBar      Fluent 2 application menu
├── ShellToolbar      Optional icon toolbar
├── ShellSideNav      NavigationView (collapsible, 48–280px)
├── ShellContentFrame @children content area
└── ShellStatusBar    Bottom status + progress

MobileShell (Android — Material 3)
├── TopAppBar         CenterAligned, nav icon, actions
├── Content + FAB     @children + FloatingActionButton
└── BottomNavBar      3–5 destinations, active indicator pill
```

### Mother-child pattern

```text
Level 1 — Mother:  AppWindow (inherits Window) — owns ALL state
Level 2 — Shell:   AppShell / MobileShell      — in property + callback
Level 3 — Modules: Button, Card, TextInput ... — atomic building blocks
```

**Axiom:** AppWindow is the ONLY state owner. Children receive via `in property`
and emit via `callback`. No `in-out`. No state in leaf components.

### Token system

```text
ui/tokens/theme.slint        Single import — never import token files directly
├── Colors                   Fluent 2 light/dark (bg, text, accent, border, shadow)
├── Spacing                  4px grid + control heights
├── Type                     Font sizes — reactive to Settings.zoom * font-scale
├── Scale                    Icon sizes — reactive to Settings.zoom
├── FluentColors/Spacing/... Exact WinUI3 values (source of truth)
├── FluentIcons              ~60 Segoe Fluent Icon codepoints
└── MaterialColors/...       Material Design 3 color roles + scales
```

### Bi-directional layout engine

```rust,no_run
use slint_ui_templates::{build_v2, drag};

// Named-slot DSL v2
let mut panels = build_v2("sidebar(0.2):content:inspector(0.25)");
let delta = 0.05f32;

// Drag: both neighbors update, sum stays 1.0
drag(&mut panels, 0, delta);   // sidebar ↔ content, inspector unchanged
drag(&mut panels, 1, delta);   // content ↔ inspector, sidebar unchanged
```

---

## Platforms

| Platform   | Design System     | Icons              | Status    |
|------------|-------------------|--------------------|-----------|
| Windows 11 | Fluent 2 / WinUI3 | Segoe Fluent Icons | Active    |
| Android    | Material Design 3 | Material Symbols   | Phase 11  |
| macOS      | Aqua / SwiftUI    | SF Symbols         | Future    |
| Linux      | libadwaita/Breeze | GNOME/Breeze icons | Future    |

---

## Settings System

```rust,no_run
use slint_ui_templates::settings::{AppSettings, ZoomSettings, ThemeSettings, ThemeMode,
                                   IconSettings, IconStyle, FontSettings};
use slint_ui_templates::AppAdapter;

let adapter = AppAdapter::new().unwrap();
let settings = AppSettings {
    zoom:  ZoomSettings { scale: 1.25 },
    theme: ThemeSettings { mode: ThemeMode::Dark, ..Default::default() },
    icons: IconSettings  { style: IconStyle::Outlined, ..Default::default() },
    font:  FontSettings  { family: Some("Segoe UI".into()), ..Default::default() },
};
adapter.apply_settings(&settings);
```

Runtime Rhai scripting also supported:
```js
set_zoom(1.25);
set_icon_style("outlined");
set_font("Segoe UI");
```

---

## Components

| Component   | Key Props                        | Callback          |
|-------------|----------------------------------|-------------------|
| Button      | `label`, `variant`               | `clicked()`       |
| Toggle      | `checked`, `label`               | `toggled(bool)`   |
| TextInput   | `text`, `placeholder`, `error`   | `changed(string)` |
| SelectField | `options`, `selected`            | `changed(string)` |
| Avatar      | `initials`, `size`               | —                 |
| Badge       | `text`, `variant`                | —                 |
| Card        | `title`, `subtitle`              | —                 |
| ProgressBar | `value` (0.0–1.0), `show-label`  | —                 |
| Toast       | `message`, `variant`, `visible`  | `closed()`        |
| Dialog      | `title`, `message`, `visible`    | `confirmed()`, `cancelled()` |
| DragHandle  | `vertical`                       | `dragged(dx,dy)`, `drag-end(dx,dy)` |

---

## License

MIT — see [LICENSE](LICENSE)
