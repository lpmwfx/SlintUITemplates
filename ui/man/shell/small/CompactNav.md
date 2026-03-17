# `shell/small/CompactNav.slint`

## `export component CompactNav inherits Rectangle`
*Line 12 · component*

CompactNav — horizontal bottom bar or vertical compact rail.

---

## `in property <string> mode: Variants.nav-bottom;`
*Line 14 · property*

Nav mode: `Variants.nav-bottom` (horizontal) or `Variants.nav-compact` (rail).

---

## `in property <[NavItem]> items;`
*Line 16 · property*

Navigation items to render.

---

## `in property <string> active;`
*Line 18 · property*

Id of the currently active item.

---

## `callback navigate(string);`
*Line 20 · callback*

Fired when a nav item is tapped.

---

## `private property <bool> is-rail: root.mode == Variants.nav-compact;`
*Line 23 · property*

Private property "is-rail" used internally.

---

