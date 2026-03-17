# `shell/small/shell.slint`

## `export component CompactShell inherits Rectangle`
*Line 14 · component*

CompactShell — fullscreen shell for Steam Deck, handheld, and small-screen targets.

---

## `in property <string> title;`
*Line 16 · property*

Page or app title shown in the top bar.

---

## `in property <[NavItem]> nav-items;`
*Line 18 · property*

Navigation items rendered in the bottom bar or side rail.

---

## `in property <string> active-view;`
*Line 20 · property*

Id of the currently active view / slot.

---

## `in property <bool> show-topbar: true;`
*Line 22 · property*

Whether the top bar is visible.

---

## `in property <bool> show-back: false;`
*Line 24 · property*

Whether the back button in the top bar is shown.

---

## `in property <string> nav-mode: Variants.nav-bottom;`
*Line 26 · property*

Nav chrome mode forwarded to CompactNav.

---

## `callback navigate(string);`
*Line 28 · callback*

Fired when a nav item is tapped.

---

## `callback back();`
*Line 30 · callback*

Fired when the top-bar back button is tapped.

---

