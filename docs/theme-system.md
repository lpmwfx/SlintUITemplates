# Theme System

SlintUITemplates uses one unified `Theme` global with **6 palette combinations**:
solid, mica, and acrylic — each in dark and light mode.

## File structure

```
ui/state/
├── Theme.slint          controller — imports palettes, exposes token surface (~75 lines)
└── theme/
    ├── solid.slint      Solid global: WinUI3 Fluent 2 opaque colors
    ├── mica.slint        Mica global: LayerFillColor semi-transparent tints
    └── acrylic.slint    Acrylic global: blur-friendly minimal tints
```

Each palette file is **pure constants** (~33 lines). No logic. No dark/light switching.
`Theme.slint` imports all three and selects based on `material` + `dark` inputs.

## Inputs (written by AppWindow only)

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `material` | string | `"solid"` | `"solid"` \| `"mica"` \| `"acrylic"` |
| `dark` | bool | OS scheme | light/dark mode toggle |
| `dark-mode` | bool | ← alias for `dark` | backwards-compat alias |
| `accent-override` | color | `transparent` | alpha=0 → derive from palette |

`AppWindow` is the only writer. Components never write to Theme directly.

## Outputs — token surface

All components read these. Values resolve from the active palette automatically.

```
bg-primary     bg-surface     bg-elevated
text-primary   text-secondary text-disabled
accent         accent-hover   accent-text
error          success        warning
border         border-strong  border-focus
shadow
```

Aliases (same value, convenience names for mother components):
```
bg-panel      → bg-surface
handle        → border
handle-hover  → border-strong
danger        → error
text-muted    → text-secondary
```

## Palette values — official WinUI3 source

### Solid

| Token | Dark | Light |
|-------|------|-------|
| bg-primary | `#202020` | `#FFFFFF` |
| bg-surface | `#2C2C2C` | `#F3F3F3` |
| bg-elevated | `#383838` | `#FFFFFF` |
| text-primary | `#FFFFFF` | `#1C1C1C` |
| text-secondary | `#9D9D9D` | `#616161` |
| accent | `#60CDFF` | `#0078D4` |
| border | `#3D3D3D` | `#E0E0E0` |

### Mica — WinUI3 `LayerFillColorDefault`

`bg-primary = transparent` — OS DWM provides the Mica backdrop.
Surfaces are semi-opaque white tints composited on top.

| Token | Dark | Light | Source |
|-------|------|-------|--------|
| bg-primary | `transparent` | `transparent` | OS Mica |
| bg-surface | `#FFFFFF0F` (6%) | `#FFFFFFB2` (70%) | LayerFillColorDefault |
| bg-elevated | `#FFFFFF18` (9%) | `#FFFFFFCC` (80%) | LayerFillColorSecondary |
| text-primary | `#FFFFFFFF` (100%) | `#000000E4` (89%) | TextFillColorPrimary |
| text-secondary | `#FFFFFFC8` (78%) | `#0000009A` (60%) | TextFillColorSecondary |
| text-disabled | `#FFFFFF5C` (36%) | `#0000005C` (36%) | TextFillColorDisabled |
| border | `#FFFFFF1A` | `#0000001A` | SubtleFillColorSecondary |

> **Why opacity values?** WinUI3 text tokens are opacity-based (`TextFillColor*`),
> not pre-multiplied. They composite correctly against any Mica backdrop regardless
> of wallpaper color. Using `#1C1C1C` (pre-multiplied for white) on a dark Mica
> backdrop = invisible text.

> Slint uses **RRGGBBAA** hex format (not ARGB). `#000000E4` = 89% opacity black. ✓

### Acrylic — blur+tint backdrop

`bg-primary = transparent`. Surfaces use minimal tints so the blur shows through.

| Token | Dark | Light |
|-------|------|-------|
| bg-surface | `#FFFFFF08` (3%) | `#FFFFFF60` (38%) |
| bg-elevated | `#FFFFFF12` (7%) | `#FFFFFF99` (60%) |
| text-primary | same as Mica | same as Mica |

## How the barrel alias works

```slint
// ui/tokens/theme.slint
export { Theme }            from "../state/Theme.slint";
export { Theme as Colors }  from "../state/Theme.slint";
```

`Colors` and `Theme` are the same singleton. Components that use `Colors.bg-surface`
get the correct value for the active material automatically — no changes needed
when switching from solid to Mica.

## Rust API

```rust
use slint::ComponentHandle;
use slint_ui_templates::Theme;

// Set material (also triggers OS DWM backdrop via platform::apply_backdrop)
ui.global::<Theme>().set_material("mica".into());

// Toggle dark mode
ui.global::<Theme>().set_dark(true);

// Override accent color
let color = slint::Color::from_rgb_u8(0, 120, 212);
ui.global::<Theme>().set_accent_override(color);
```

Via `AppDsl` builder (preferred):
```rust
use slint_ui_templates::dsl::BgStyle;

let dsl = AppDsl::builder("My App")
    .nav(vec![...])
    .bg_style(BgStyle::Mica)
    .build()?;

adapter.apply_dsl(&dsl);   // sets Theme.material + OS DWM in one call
```

## Adding a new material

1. Create `ui/state/theme/mymaterial.slint` — `export global MyMaterial { ... }`
2. Import in `Theme.slint` and add a branch to each output property
3. Add `BgStyle::MyMaterial` to `src/dsl/mod.rs`
4. Handle in `src/platform/mod.rs` for OS-level backdrop (if needed)

No component files need changes — they all read `Colors.*` via the token surface.

## WinUI3 reference

- [Mica material](https://learn.microsoft.com/en-us/windows/apps/design/style/mica)
- [Layering and elevation](https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/layering)
- [XAML theme resources](https://learn.microsoft.com/en-us/windows/apps/develop/platform/xaml/xaml-theme-resources)
  — TextFillColorPrimary light = `#E4000000`, dark = `#FFFFFFFF`
