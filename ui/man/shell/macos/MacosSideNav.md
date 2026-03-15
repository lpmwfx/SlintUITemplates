# `shell/macos/MacosSideNav.slint`

## `export component MacosSideNav inherits Rectangle`
*Line 11 · component*

macOS left sidebar — stacked nav items inside a fixed-width surface panel.

---

## `in property <[NavItem]> items;`
*Line 13 · property*

Items to render in the sidebar.

---

## `in property <string>    active-item;`
*Line 15 · property*

Id of the currently active item.

---

## `callback navigate(string);`
*Line 18 · callback*

Fired when a nav item is selected.

---

