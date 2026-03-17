# `shell/macos/MacosNavItem.slint`

## `export component MacosNavItem inherits Rectangle`
*Line 10 · component*

macOS sidebar navigation row — icon glyph + label, touch-target height.

---

## `in property <NavItem> item;`
*Line 12 · property*

Navigation data for this item.

---

## `in property <bool>    active: false;`
*Line 14 · property*

Whether this item is the active destination.

---

## `callback navigate(string);`
*Line 17 · callback*

Fired when the user selects this item.

---

