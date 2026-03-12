# Platform Placement Rules

Placement rules are enforced per-platform. Each platform has exactly one correct
position for every UI element. The framework enforces these — you declare what,
the framework decides where.

## Windows 11 / Fluent 2

```
┌─────────────────────────────────────────────┐
│  MenuBar          top, full width, 30px      │
├─────────────────────────────────────────────┤
│  Toolbar          optional, below menubar    │
├──────────────┬──────────────────────────────┤
│              │                              │
│  SideNav     │  Content Area               │
│  48–280px    │  (flex, fills remaining)    │
│  collapsed   │                              │
│  ↕ expanded  │                              │
│              │                              │
├──────────────┴──────────────────────────────┤
│  StatusBar        bottom, full width, 24px   │
└─────────────────────────────────────────────┘
```

**Rules:**
- MenuBar: TOP, always visible (desktop apps must have a menu)
- Toolbar: below menubar ONLY, never left or bottom
- SideNav: LEFT side, never right
- Content: fills the main area, no chrome
- StatusBar: BOTTOM, read-only from user perspective
- FAB: NOT on desktop (mobile only)
- BottomNavBar: NOT on desktop (mobile only)

See: `rules/winui/placement.md`

## Android / Material 3

```
┌──────────────────────────┐
│  TopAppBar    64dp        │
│  (CenterAligned title)   │
├──────────────────────────┤
│                          │
│  Content Area            │
│  (fills remaining)       │
│                  [FAB]   │
│             56dp ↗       │
├──────────────────────────┤
│  BottomNavBar  80dp      │
│  3–5 destinations        │
└──────────────────────────┘
```

**Rules:**
- TopAppBar: TOP, always present
- BottomNavBar: BOTTOM, 3–5 items (never more)
- FAB: content area, bottom-right corner, above bottom nav
- MenuBar: NOT on Android (no visible menu bar)
- SideNav: use NavigationDrawer (not in current scope)

## Cross-platform rules

| Element | Windows | Android | Note |
|---------|---------|---------|------|
| Primary nav | SideNav (left) | BottomNavBar (bottom) | Platform-canonical |
| Secondary nav | Toolbar submenu | NavigationDrawer | Not yet implemented |
| Primary action | Button in content | FAB | Platform-canonical |
| App title | Window title bar | TopAppBar | |
| Search | Toolbar icon | TopAppBar icon | |
| Settings | File/Edit menu or nav item | Nav item | |
