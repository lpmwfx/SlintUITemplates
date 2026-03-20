# `shell/desktop/shell.slint`

## `export component DesktopShell inherits VerticalLayout`
*Line 16 · component*

Fluent desktop shell composed of toolbar, side navigation, content frame, and status bar.

---

## `in property <[NavItem]>          nav-items;`
*Line 18 · property*

Navigation items rendered in the left side navigation.

---

## `in property <[ShellToolbarItem]> toolbar-items;`
*Line 20 · property*

Toolbar items rendered in the optional toolbar row.

---

## `in property <bool>               show-toolbar: false;`
*Line 22 · property*

Controls whether the toolbar row is shown.

---

## `in property <string>             active-view: "";`
*Line 24 · property*

Id of the currently active content view.

---

## `in property <string>             status-text: "Ready";`
*Line 26 · property*

Status text shown in the bottom status bar.

---

## `in property <float>              progress: -1.0;`
*Line 28 · property*

Optional progress value shown in the status bar. Negative hides it.

---

## `private property <length> nav-width: Sizes.panel-sm;`
*Line 32 · property*

Private property "nav-width" used internally.

---

## `private property <bool>   nav-collapsed: nav-width <= Spacing.xl + Spacing.sm + Spacing.xs;`
*Line 34 · property*

Private property "nav-collapsed" used internally.

---

## `callback navigate(string);`
*Line 37 · callback*

Fired when a navigation destination is selected.

---

## `callback toolbar-clicked(string);`
*Line 39 · callback*

Fired when a toolbar action is selected.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
