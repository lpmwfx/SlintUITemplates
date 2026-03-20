# Settings System

`AppSettings` is a serializable struct that covers zoom, theme, icons, and font.
Load from TOML, modify, push to Slint globals, save on exit — no recompile needed.

## Default config file

`config/settings.toml`:

```toml
[zoom]
scale = 1.0           # 0.5–2.0 — multiplies icon AND font sizes together

[theme]
mode = "system"       # "system" | "light" | "dark"
# accent = "#0078d4"  # uncomment to override the default accent color

[icons]
style = "filled"      # "filled" | "outlined"
color = "theme"       # "theme" | "accent" | "#rrggbb"

[font]
# family = "Segoe UI" # omit for platform default
font_scale = 1.0      # independent font-only scale on top of zoom
```

## Rust API

```rust
use slint_ui_templates::settings::{AppSettings, ZoomSettings, ThemeSettings,
                                   ThemeMode, IconSettings, IconStyle, FontSettings};
use slint_ui_templates::adapter::AppAdapter;
use std::path::Path;

// Load from file (falls back to defaults if absent)
let settings = AppSettings::from_file_or_default(Path::new("config/settings.toml"));

// Or build programmatically
let settings = AppSettings {
    zoom:  ZoomSettings { scale: 1.25 },
    theme: ThemeSettings {
        mode:   ThemeMode::Dark,
        accent: Some("#FF6B35".into()),
    },
    icons: IconSettings {
        style: IconStyle::Outlined,
        color: "accent".into(),
    },
    font:  FontSettings {
        family:     Some("Segoe UI".into()),
        font_scale: 1.1,
    },
};

// Push to Slint globals
let adapter = AppAdapter::new()?;
adapter.apply_settings(&settings);

// Save modified settings
settings.to_file(Path::new("config/settings.toml"))?;
```

## Settings fields

### ZoomSettings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `scale` | f32 | 1.0 | Global zoom — multiplies icon + font sizes (0.5–2.0) |

### ThemeSettings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | ThemeMode | System | `System` \| `Light` \| `Dark` |
| `accent` | Option\<String\> | None | Hex color override, e.g. `"#FF6B35"` |

`ThemeMode::System` reads the OS dark/light preference reactively via `Palette.color-scheme`.

### IconSettings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `style` | IconStyle | Filled | `Filled` \| `Outlined` |
| `color` | String | `"theme"` | `"theme"` \| `"accent"` \| `"#rrggbb"` |

`"theme"` = `Colors.text-primary` (follows dark/light). `"accent"` = `Colors.accent`.

### FontSettings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `family` | Option\<String\> | None | System font name; None = Slint default |
| `font_scale` | f32 | 1.0 | Independent font-only scale on top of zoom |

Final font size = `base_size * zoom * font_scale`.

## Push pipeline

```
AppSettings (Rust)
  → apply_settings()
    → Theme.dark             (ThemeMode → bool)
    → Theme.accent-override  (hex string → slint::Color)
    → Settings.zoom
    → Settings.icon-style    ("filled" | "outlined")
    → Settings.icon-color    ("theme" | "accent" | "#rrggbb")
    → Settings.font-family
    → Settings.font-scale
```

All Slint property bindings react automatically — no manual refresh needed.

## Note on material / backdrop

The window backdrop (Mica/Acrylic) is **not** in `AppSettings` — it is part of the
DSL configuration (`AppDsl::bg_style`) and applied at startup by the platform layer.


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
