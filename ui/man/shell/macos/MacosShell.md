# `shell/macos/MacosShell.slint`

## `export component MacosShell inherits VerticalLayout`
*Line 12 · component*

macOS app shell — title bar + sidebar navigation + content area.

---

## `in property <string>    title;`
*Line 14 · property*

Window or app title shown in the title bar.

---

## `in property <[NavItem]> nav-items;`
*Line 16 · property*

Navigation items shown in the sidebar.

---

## `in property <string>    active-slot;`
*Line 18 · property*

Id of the currently active view / slot.

---

## `callback slot-changed(string);`
*Line 21 · callback*

Fired when the user selects a nav destination.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
