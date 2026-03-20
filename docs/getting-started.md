# Getting Started

SlintUITemplates is a platform-native Rust + Slint UI shell library.
Declare **what** your app needs — nav items, menus, content slots — and the
framework delivers the correct platform-stereotyped layout automatically.

## Run the examples

```sh
git clone https://github.com/lpmwfx/SlintUITemplates
cd SlintUITemplates

cargo run --example viewer   # browse all widgets + themes
cargo run --example desktop  # full desktop shell (Windows Fluent 2)
```

## Add as a dependency

```toml
[dependencies]
slint-ui-templates = "0.1"
```

## Minimal app

```rust
use slint_ui_templates::{
    adapter::AppAdapter,
    dsl::{AppDsl, Nav, BgStyle},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dsl = AppDsl::builder("My App")
        .nav(vec![
            Nav::new("home",     "Home",     "home"),
            Nav::new("settings", "Settings", "settings"),
        ])
        .bg_style(BgStyle::Mica)   // optional — Windows 11 Mica backdrop
        .build()?;

    let mut adapter = AppAdapter::new()?;
    adapter.apply_dsl(&dsl);
    adapter.run()?;
    Ok(())
}
```

## Live preview (no recompile)

```sh
slint-viewer ui/desktop/app-window.slint --auto-reload
```

Edit any `.slint` file → preview updates instantly. No Rust compilation needed
for UI iteration. Useful during theme and layout work.

## Project layout

```
ui/
├── tokens/theme.slint   single import for all tokens (Colors, Spacing, Type, ...)
├── state/               global singletons (Theme, Settings)
├── desktop/             AppWindow + NavBar, SideBar, ContentArea, StatusBar
├── shell/               ShellToolbar, ShellMenuBar, mobile shell
├── shared/              reusable components (Button, Card, Toggle, ...)
├── modules/             atomic building blocks (DragHandle, PanelContainer)
└── viewer/              FrameworkViewer — widget catalog for visual testing

src/
├── adapter/             AppAdapter — main Rust entry point
├── dsl/                 AppDsl builder + validation
├── settings/            AppSettings (TOML + JSON)
├── bindings/rhai/       Rhai scripting API
├── platform/            Windows DWM backdrop (Mica / Acrylic)
└── layout/              panel DSL parser + solver

config/
├── settings.toml        default app settings
└── desktop.toml         default grid row ratios
```

## Next steps

- [Architecture](architecture.md) — mother-child pattern, token system
- [Theme System](theme-system.md) — Solid / Mica / Acrylic × dark / light
- [DSL Reference](dsl.md) — AppDsl builder, nav, toolbar, backdrop
- [Settings](settings.md) — zoom, theme, icons, font (TOML + Rust)
- [Components](components.md) — full component catalog


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
