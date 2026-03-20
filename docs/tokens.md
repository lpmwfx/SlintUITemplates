# Token Reference

## How to use tokens

```slint
import { Colors, Spacing, Type, FluentIcons } from "../tokens/theme.slint";

component MyComponent inherits Rectangle {
    background: Colors.bg-surface;

    Text {
        text: FluentIcons.settings;
        font-family: "Segoe Fluent Icons";
        font-size: Type.body-size;
        color: Colors.text-primary;
    }
}
```

**Rule:** Always import from `theme.slint`. Never hardcode px, colors, or font names.

---

## Colors

`Colors` is an alias for `Theme` — see [theme-system.md](theme-system.md) for the
full 6-palette table (solid/mica/acrylic × dark/light). Below: solid mode values.

| Token | Solid light | Solid dark | Usage |
|-------|-------------|------------|-------|
| `Colors.bg-primary` | `#FFFFFF` | `#202020` | Window background |
| `Colors.bg-surface` | `#F3F3F3` | `#2C2C2C` | Cards, panels — LayerFillColorDefault |
| `Colors.bg-elevated` | `#FFFFFF` | `#383838` | Elevated surfaces |
| `Colors.text-primary` | `#1C1C1C` | `#FFFFFF` | Body text |
| `Colors.text-secondary` | `#616161` | `#9D9D9D` | Supporting text |
| `Colors.text-disabled` | `#A0A0A0` | `#6B6B6B` | Disabled states |
| `Colors.accent` | `#0078D4` | `#60CDFF` | Interactive, selection |
| `Colors.accent-hover` | `#106EBE` | `#4CC2F1` | Hover state |
| `Colors.border` | `#E0E0E0` | `#3D3D3D` | Default borders |
| `Colors.border-strong` | `#C0C0C0` | `#575757` | Emphasized borders |
| `Colors.error` | `#C42B1C` | `#C42B1C` | Error states |

On Mica/Acrylic: `bg-primary` = `transparent` (OS provides backdrop),
surfaces use WinUI3 `LayerFillColorDefault` tints, text uses opacity-based
`TextFillColor*` tokens. See [theme-system.md](theme-system.md).

## Spacing

| Token | Value | Usage |
|-------|-------|-------|
| `Spacing.xs` | 4px | Tight gaps |
| `Spacing.sm` | 8px | Related elements |
| `Spacing.md` | 16px | Standard padding |
| `Spacing.lg` | 24px | Section padding |
| `Spacing.xl` | 48px | Container heights |
| `Spacing.control-sm` | 24px | Compact controls |
| `Spacing.control-md` | 32px | Standard buttons/inputs |
| `Spacing.control-lg` | 40px | Large controls, nav items |

## Typography (zoom-reactive)

| Token | Base | Usage |
|-------|------|-------|
| `Type.caption-size` | 12px | Labels, hints |
| `Type.body-size` | 14px | Body text |
| `Type.subtitle-size` | 16px | Section subtitles |
| `Type.heading-size` | 20px | Section titles |
| `Type.title-size` | 28px | Page titles |

Sizes scale with `Settings.zoom * Settings.font-scale` at runtime.

## FluentIcons (Segoe Fluent Icons font)

Set `font-family: "Segoe Fluent Icons"` when rendering icon strings.

```
home, back, forward, menu, settings, nav-collapse, nav-expand
add, remove, delete, edit, save, copy, paste, cut, undo, redo
refresh, close, search, filter, sort, share, download, upload, print
maximize, minimize, restore
folder, folder-open, file, file-new, cloud, database
play, pause, stop, volume, mute, camera, image
list, grid, layout, apps, panel-left, panel-right, fullscreen
person, people, chat, mail, phone, notification, help, info, warning
checkmark, dismiss, device, keyboard, wifi, bluetooth
```

## Fluent 2 tokens (source layer)

`FluentColors`, `FluentSpacing`, `FluentRadius`, `FluentMotion`, `FluentType`, `FluentIcons`

These are the raw WinUI3 values. The `Colors` / `Spacing` globals are the component-facing API.
Import both from `theme.slint` if you need the exact Fluent 2 values directly.

## Material 3 tokens

`MaterialColors`, `MaterialSpacing`, `MaterialRadius`, `MaterialType`, `MaterialElevation`

Used by `MobileShell` and its sub-components. Import from `theme.slint`.

---

## How to override tokens

For theme customization (e.g., custom accent color):

```rust
// Rust — push accent override at startup
adapter.apply_settings(&AppSettings {
    theme: ThemeSettings {
        accent: Some("#FF6B35".into()),
        ..Default::default()
    },
    ..Default::default()
});
```

For font/zoom:
```rust
adapter.apply_settings(&AppSettings {
    zoom: ZoomSettings { scale: 1.5 },
    font: FontSettings { family: Some("Segoe UI".into()), font_scale: 1.1 },
    ..Default::default()
});
```


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
