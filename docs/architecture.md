# Architecture

## Overview

SlintUITemplates follows the **mother-child pattern**: a strict hierarchy where
state flows down and events flow up. There is one and only one state owner per window.

## 3-Level Hierarchy

```
Level 1 — Mother (ui/mother/ or AppWindow)
  Inherits Window. Owns ALL state and layout dimensions.
  Sets in-out properties; writes to globals (Colors, Settings).

Level 2 — Shell / Views (ui/shell/, ui/views/)
  Receives state via `in property`. Emits changes via `callback`.
  Never owns state. Never writes globals.

Level 3 — Modules (ui/widgets/, ui/modules/)
  Atomic building blocks. In property + callback only.
  Single responsibility. One component per file.
```

## AppShell Flow

```
Rust Host                    Slint (ui/shell/app-shell.slint)
────────                     ──────────────────────────────
ShellConfig ─────────────►  AppShell
  nav_model()  ─────────►     DesktopShell
  menu_model() ─────────►       ShellMenuBar
  toolbar_model() ──────►       ShellToolbar (optional)
                                ShellSideNav ──► navigate(id) ──► slot_changed(id) ──► Rust
                                ShellContentFrame (@children)
                                ShellStatusBar
```

## Token System

```
theme.slint (single import point)
├── Colors        — Fluent 2 colors, dark-mode reactive
├── Spacing       — 4px grid, control heights (32/40px)
├── Type          — Font sizes, reactive to Settings.zoom * font-scale
├── Scale         — Icon sizes, reactive to Settings.zoom
├── FluentColors  — Exact WinUI3 light/dark values (source layer)
├── FluentSpacing — WinUI3 spacing tokens
├── FluentRadius  — WinUI3 corner radii
├── FluentMotion  — WinUI3 animation durations
├── FluentType    — WinUI3 type scale
├── FluentIcons   — Segoe Fluent Icons codepoints (~60)
├── MaterialColors — M3 color roles
├── MaterialSpacing — M3 8dp grid
├── MaterialRadius  — M3 shape scale
├── MaterialType    — M3 type scale
└── MaterialElevation — M3 tint overlay levels
```

**Rule:** Components import ONLY `theme.slint`. Never import individual token files.

## Layout Engine

```
DSL string → PanelNode tree → Solver → [SolvedItem] → Slint model
"1:2/1:1"    HSplit           flat     normalized     PanelContainer
             VSplit           items    x/y/w/h 0..1

DSL v2: named slots with explicit ratios
"sidebar(0.2):content:inspector(0.25)"
  → sidebar=0.20, content=0.55, inspector=0.25
  → drag(panels, 0, delta) → zero-sum flex, both neighbors update
```

## Theme global + barrel alias

`Theme` is the unified color global. `Colors` is a barrel alias for `Theme` —
they are the same Slint singleton.

```
ui/tokens/theme.slint:
  export { Theme }            from "../state/Theme.slint";
  export { Theme as Colors }  from "../state/Theme.slint";
```

The 39+ component files that use `Colors.*` need zero changes. Rust uses `Theme`:

```rust
ui.global::<Theme>().set_dark(true);
ui.global::<Theme>().set_material("mica".into());
```

## Settings Pipeline

```
AppSettings (Rust/TOML)
  → AppAdapter::apply_settings()
    → Theme.dark          (light/dark mode)
    → Theme.accent-override
    → Settings.zoom
    → Settings.icon-style
    → Settings.icon-color
    → Settings.font-family
    → Settings.font-scale
  → Slint property bindings propagate automatically
  → All components re-render with new values
```


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
