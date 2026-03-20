# `viewer/page-app-shell.slint`

## `export component PageAppShell inherits VerticalLayout`
*Line 16 · component*

Viewer page demonstrating desktop and mobile shell composition with shared content slots.

---

## `private property <string>    layout-mode: Variants.windows;`
*Line 18 · property*

Private property "layout-mode" used internally.

---

## `private property <string> view-home: "home";`
*Line 21 · property*

Private property "view-home" used internally.

---

## `private property <string> view-list: "list";`
*Line 23 · property*

Private property "view-list" used internally.

---

## `private property <string> view-settings: "settings";`
*Line 25 · property*

Private property "view-settings" used internally.

---

## `private property <string>    active-windows: root.view-home;`
*Line 29 · property*

Private property "active-windows" used internally.

---

## `private property <string>    active-android: root.view-home;`
*Line 31 · property*

Private property "active-android" used internally.

---

## `private property <string>    active-view: root.layout-mode == Variants.windows ? root.active-windows : root.active-android;`
*Line 33 · property*

Private property "active-view" used internally.

---

## `private property <string> shell-label-win: "App Shell — Windows (Fluent 2)";`
*Line 37 · property*

Private property "shell-label-win" used internally.

---

## `private property <string> shell-label-android: "App Shell — Android (Material 3)";`
*Line 39 · property*

Private property "shell-label-android" used internally.

---

## `private property <string> btn-to-android: "→ Android";`
*Line 41 · property*

Private property "btn-to-android" used internally.

---

## `private property <string> btn-to-windows: "→ Windows";`
*Line 43 · property*

Private property "btn-to-windows" used internally.

---

## `private property <string> status-prefix: "Active: ";`
*Line 45 · property*

Private property "status-prefix" used internally.

---

## `private property <string> mobile-title: "App";`
*Line 47 · property*

Private property "mobile-title" used internally.

---

## `private property <length> phone-w: Sizes.comp-380;`
*Line 49 · property*

Private property "phone-w" used internally.

---

## `private property <length> phone-border: Sizes.border-w-2;`
*Line 51 · property*

Private property "phone-border" used internally.

---

## `private property <[NavItem]> nav-items: [`
*Line 54 · property*

Private property "nav-items" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
