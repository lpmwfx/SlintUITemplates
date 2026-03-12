---
name: Slint Composition Rules
description: Correct Slint component composition — fill, layout, stretch, MenuBar, state
type: rule
platform: all
---

# Slint Composition Rules

These rules encode what Slint requires structurally. Violations produce silent layout
failures — components collapse to minimum size or state changes are ignored.
The scanner enforces these automatically so no reasoning is needed at usage time.

---

## Rule 1 — MenuBar lives in Window only

**Slint constraint:** `MenuBar` can only be a direct child of a `Window` component.
Placing it inside any other component (Rectangle, VerticalLayout, custom component)
is a compile error.

```
CORRECT:
  export component MyWindow inherits Window {
      MenuBar { Menu { title: "File"; ... } }
      ...
  }

VIOLATION:
  export component MyShell inherits VerticalLayout {
      MenuBar { ... }   // ← compile error
  }
```

**Scanner pattern:** `MenuBar` must not appear in a component that does not `inherits Window`.

---

## Rule 2 — Shell components never inside Flickable

**Why:** Shell components (`AppShell`, `MobileShell`, `DesktopShell`) are designed to
fill their parent. A `Flickable` with `viewport-height: preferred-height` collapses them
to minimum height instead of filling the available space.

```
VIOLATION:
  Flickable {
      viewport-height: content.preferred-height;
      AppShell { ... }       // collapses — no fill
      MobileShell { ... }    // collapses — no fill
  }

CORRECT:
  // Shell pages placed directly in layout — fill available space
  if active == "shell": AppShell { horizontal-stretch: 1; }
  // Scrollable pages in their own Flickable
  if active != "shell": Flickable { ... }
```

**Scanner pattern:** `AppShell`, `MobileShell`, `DesktopShell` must not appear as
direct children of `Flickable`.

---

## Rule 3 — vertical-stretch only works inside layouts

**Why:** `vertical-stretch` is a hint to `VerticalLayout` / `HorizontalLayout`.
Inside a `Rectangle`, it has no effect — the child gets its preferred height (often 0).

```
VIOLATION:
  Rectangle {
      MyView { vertical-stretch: 1; }   // ignored — Rectangle is not a layout
  }

CORRECT (option A — wrap in VerticalLayout):
  Rectangle {
      VerticalLayout {
          MyView { vertical-stretch: 1; }   // works
      }
  }

CORRECT (option B — use height binding):
  Rectangle {
      MyView { height: parent.height; }   // explicit fill
  }
```

**Scanner pattern:** `vertical-stretch` on a child whose immediate parent is `Rectangle`
(or any non-layout component).

---

## Rule 4 — @children in Rectangle needs VerticalLayout wrapper

**Why:** `@children` placed directly in a `Rectangle` are stacked at position (0,0) with
preferred size. To allow children to stretch, wrap in a layout.

```
VIOLATION:
  export component MyFrame inherits Rectangle {
      @children   // children stack at 0,0, no stretch
  }

CORRECT:
  export component MyFrame inherits Rectangle {
      VerticalLayout {
          @children   // children participate in layout, stretch works
      }
  }
```

**Scanner pattern:** `@children` as direct child of a component that `inherits Rectangle`
without an intermediate layout wrapper.

---

## Rule 5 — in-out property banned in non-Window components

**Mother-child axiom:** Only the Window (mother) owns state. Children receive via
`in property` and emit via `callback`. `in-out property` in a child means the child
owns state — violation of the axiom.

```
VIOLATION:
  export component MyChild inherits Rectangle {
      in-out property <string> active;   // state ownership — banned
  }

CORRECT:
  export component MyChild inherits Rectangle {
      in property <string> active;       // receives from mother
      callback changed(string);          // emits to mother
  }
```

**Exception:** `in-out property` is allowed in:
- Components that `inherits Window` (they ARE mother)
- Slint `global` files (globals are shared state, not child state)

**Scanner pattern:** `in-out property` in any component that does not `inherits Window`
and is not a `global`.

---

## Rule 6 — Conditional fill pages bypass shared Flickable

**Why:** A Flickable shared between a fill-page (AppShell) and scroll-pages (widget
demos) cannot correctly serve both. Fill-pages collapse inside it.

**Pattern:** When a viewer/router mixes fill-pages and scroll-pages, split them:

```
CORRECT:
  // Fill-page: placed directly in layout
  if active == "shell": AppShell { horizontal-stretch: 1; }

  // Scroll-pages: share one Flickable
  if active != "shell": Flickable {
      viewport-height: content.preferred-height;
      VerticalLayout {
          if active == "buttons": PageButtons {}
          if active == "inputs":  PageInputs  {}
      }
  }
```

**Scanner pattern:** A component known to be a fill-component (`AppShell`, `MobileShell`,
`PageAppShell`) inside a `Flickable` alongside scroll-content.

---

## Summary table

| Rule | Pattern to catch | Effect if violated |
|------|------------------|--------------------|
| 1 | `MenuBar` outside Window | Compile error |
| 2 | Shell inside Flickable | Collapses to 0/min height |
| 3 | `vertical-stretch` in Rectangle | Ignored — preferred height |
| 4 | `@children` in Rectangle without layout | Stack at 0,0 |
| 5 | `in-out` in non-Window component | State ownership leak |
| 6 | Fill-page in shared Flickable | Collapses to min height |
