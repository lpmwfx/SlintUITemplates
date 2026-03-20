# `shell/app-shell.slint`

## `export component AppShell inherits Rectangle`
*Line 16 · component*

Cross-platform shell facade used by the viewer and host applications.

On desktop it wraps `DesktopShell`; mobile variants are re-exported separately.

---

## `in property <[NavItem]>          nav-items;`
*Line 18 · property*

Navigation items rendered in the shell navigation surface.

---

## `in property <string>             active-slot;`
*Line 20 · property*

Id of the currently active content slot.

---

## `in property <bool>               show-toolbar:  false;`
*Line 22 · property*

Controls whether the desktop toolbar row is visible.

---

## `in property <[ShellToolbarItem]> toolbar-items;`
*Line 24 · property*

Toolbar buttons rendered below the desktop menu bar.

---

## `in property <string>             status-text:   "Ready";`
*Line 26 · property*

Status text shown in the desktop status bar.

---

## `in property <float>              progress:      -1.0;`
*Line 28 · property*

Optional progress value for the desktop status bar. Negative hides it.

---

## `callback slot-changed(string);`
*Line 31 · callback*

Fired when navigation changes the active slot id.

---

## `callback toolbar-clicked(string);`
*Line 33 · callback*

Fired when a toolbar button is activated.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
