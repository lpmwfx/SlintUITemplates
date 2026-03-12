---
name: Fluent 2 Token Reference
description: Token naming and values matching WinUI3 / Fluent 2 design system
type: rule
platform: windows
---

# Fluent 2 Token Reference

## Token files

All Fluent 2 tokens live in `ui/tokens/fluent/`.
Import via `ui/tokens/theme.slint` — never import fluent/ files directly.

```
ui/tokens/fluent/
├── colors.slint      FluentColors global
├── spacing.slint     FluentSpacing global
├── radius.slint      FluentRadius global
├── elevation.slint   FluentElevation global
├── motion.slint      FluentMotion global
├── typography.slint  FluentType global
├── icons.slint       FluentIcons global
└── mod.slint         re-exports all
```

## Color Tokens

### Light mode
| Token | Value | Usage |
|-------|-------|-------|
| bg-primary | #FFFFFF | Window background |
| bg-surface | #F3F3F3 | Cards, panels, secondary surfaces |
| bg-elevated | #FFFFFF | Overlays, elevated cards |
| accent | #0078D4 | Primary interactive color |
| accent-hover | #106EBE | Hover state |
| accent-pressed | #005A9E | Pressed state |
| text-primary | #1C1C1C | Body text |
| text-secondary | #616161 | Supporting text, labels |
| text-disabled | #A0A0A0 | Disabled state |
| border | #E0E0E0 | Default borders |
| border-strong | #C0C0C0 | Emphasized borders |

### Dark mode
| Token | Value | Usage |
|-------|-------|-------|
| bg-primary | #202020 | Window background |
| bg-surface | #2C2C2C | Cards, panels |
| bg-elevated | #383838 | Overlays |
| accent | #60CDFF | Primary interactive (light blue) |
| accent-hover | #4CC2F1 | Hover |
| accent-pressed | #38B5E6 | Pressed |
| text-primary | #FFFFFF | Body text |
| text-secondary | #9D9D9D | Supporting text |
| text-disabled | #6B6B6B | Disabled |
| border | #3D3D3D | Default borders |
| border-strong | #575757 | Emphasized borders |

## Spacing Tokens (FluentSpacing)

4px base grid — all values multiples of 4.

| Token | Value | Usage |
|-------|-------|-------|
| xs | 4px | Tight spacing, icon gaps |
| sm | 8px | Small gaps between related elements |
| md | 12px | Standard component padding |
| lg | 16px | Section padding |
| xl | 20px | Large spacing |
| xxl | 24px | Extra large |
| xxxl | 32px | Section separators |
| control-sm | 24px | Compact controls |
| control-md | 32px | Standard buttons, inputs |
| control-lg | 40px | Large/nav items |

## Radius Tokens (FluentRadius)

| Token | Value | Usage |
|-------|-------|-------|
| none | 0px | Flush edges |
| sm | 2px | Checkboxes, small chips |
| md | 4px | Buttons, inputs, cards |
| lg | 8px | Panels, dialogs |
| xl | 12px | Navigation pane, large surfaces |
| circle | 9999px | Avatars, pills |

## Motion Tokens (FluentMotion)

| Token | Duration | Usage |
|-------|----------|-------|
| fast | 83ms | Micro interactions (hover, press) |
| normal | 167ms | Standard transitions |
| slow | 250ms | Larger transitions (panels, pages) |
| slower | 383ms | Deliberate / emphasis |

## Typography Tokens (FluentType)

| Token | Size | Weight | Usage |
|-------|------|--------|-------|
| caption | 12px | 400 | Labels, hints |
| body | 14px | 400 | Body text |
| body-strong | 14px | 600 | Emphasized body |
| subtitle | 20px | 600 | Section titles |
| title | 28px | 600 | Page titles |
| title-large | 40px | 600 | Hero titles |
| display | 68px | 600 | Display / hero |

## Rules

- NEVER use literal color values in component files — always via Colors or FluentColors
- NEVER use literal px sizes for spacing — always via Spacing or FluentSpacing
- NEVER use literal border-radius values — always via FluentRadius (or Radius when added)
- The `Colors` global (ui/tokens/colors.slint) is the component-facing API — Fluent globals are the source values
