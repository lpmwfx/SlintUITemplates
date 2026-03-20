---
name: WinUI3 Placement Rules
description: Canonical slot positions for Windows 11 / WinUI3 desktop layout
type: rule
platform: windows
---

# WinUI3 Slot Placement Rules

## Desktop Shell Layout (top → bottom, left → right)

```
┌─────────────────────────────────────────────┐
│  MenuBar         (top, full width, 30px)    │
├─────────────────────────────────────────────┤
│  Toolbar         (optional, below menubar)  │
├──────────┬──────────────────────────────────┤
│          │                                  │
│ SideNav  │  Content Area                    │
│  (left)  │  (flex, fills remaining space)   │
│          │                                  │
├──────────┴──────────────────────────────────┤
│  StatusBar       (bottom, full width, 24px) │
└─────────────────────────────────────────────┘
```

## Rules

### MenuBar
- Position: TOP, full window width
- Height: 30px (WinUI3 standard)
- Contains: application menus (File, Edit, View, Help, ...)
- NEVER place in sidebar, content area, or bottom
- VIOLATION: MenuBar anywhere other than top

### Toolbar (optional)
- Position: directly BELOW MenuBar
- Height: 40px (Fluent 2 control-lg)
- Contains: frequently-used icon buttons (New, Open, Save, ...)
- NEVER duplicate MenuBar items as toolbar text-labels
- Toolbar is OPTIONAL — only add when frequent actions justify it

### SideNav (NavigationView)
- Position: LEFT side, below toolbar/menubar, above statusbar
- Width: 48px collapsed, 280px expanded (Fluent NavigationView defaults)
- Contains: primary navigation destinations (top-level views)
- NEVER use SideNav for secondary actions (use Toolbar or ContextMenu)
- Max nav items: 7 (beyond 7, use hierarchy or overflow)

### Content Area
- Position: RIGHT of SideNav, fills remaining space
- Contains: the active view routed by SideNav
- Content area owns NO chrome — pure content only
- Gutters between content panels are bi-directional (Rust ratio-solver)

### StatusBar
- Position: BOTTOM, full window width
- Height: 24px (Fluent 2 standard)
- Contains: status text, optional progress indicator, connection state
- NEVER use StatusBar for navigation or primary actions
- StatusBar content is READ-ONLY from user perspective

## Slot order (z-order)
1. MenuBar (top chrome)
2. Toolbar (sub-chrome, optional)
3. SideNav + Content (main area, horizontal split)
4. StatusBar (bottom chrome)

## Violations (blocked by pre-commit hook)
- FAB on desktop — FAB is Mobile only (Material 3)
- BottomNavBar on desktop — Mobile only
- TopAppBar on desktop — Mobile only
- SideNav on the right — always left on Windows
- MenuBar below content — always top


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
