# `shell/desktop/side-nav.slint`

## `export component ShellSideNav inherits Rectangle`
*Line 10 · component*

Collapsible desktop side navigation listing shell destinations.

---

## `in property <[NavItem]> items;`
*Line 12 · property*

Navigation items rendered in the side rail.

---

## `in property <string>    active-item: "";`
*Line 14 · property*

Id of the currently active navigation item.

---

## `in property <bool>      collapsed: false;`
*Line 16 · property*

Renders the compact icon-only mode when `true`.

---

## `in property <string>    icon-font: "Segoe Fluent Icons";`
*Line 18 · property*

Font family used to render icon glyph strings.

---

## `callback navigate(string);`
*Line 20 · callback*

Fired when the user chooses a navigation item.

---

## `callback toggle-collapsed();`
*Line 22 · callback*

Fired when the collapse/expand affordance is clicked.

---

## `private property <float>  active-alpha: Sizes.alpha-15;`
*Line 25 · property*

Private property "active-alpha" used internally.

---

## `private property <length> indicator-w: Sizes.border-w-3;`
*Line 27 · property*

Private property "indicator-w" used internally.

---

## `private property <length> indicator-radius: Sizes.border-w-2;`
*Line 29 · property*

Private property "indicator-radius" used internally.

---

