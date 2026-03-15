# `desktop/app-window.slint`

## `export component AppWindow inherits Window`
*Line 19 · component*

A pp wi nd ow component.

---

## `in property <string> window-title: "SlintUI Templates";`
*Line 21 · property*

Input property "window-title".

---

## `in-out property <string>    active-view: NavId.home;`
*Line 34 · property*

Two-way property "active-view".

---

## `in-out property <bool>      sidebar-open: true;`
*Line 36 · property*

Two-way property "sidebar-open".

---

## `in-out property <bool>      sidebar-collapsed: false;`
*Line 38 · property*

Two-way property "sidebar-collapsed".

---

## `in-out property <[NavItem]> nav-items: [`
*Line 40 · property*

Two-way property "nav-items".

---

## `in property <bool>               show-toolbar:  false;`
*Line 48 · property*

Input property "show-toolbar".

---

## `in property <[ShellToolbarItem]> toolbar-items: [];`
*Line 50 · property*

Input property "toolbar-items".

---

## `in-out property <string> status-text: "";`
*Line 54 · property*

Two-way property "status-text".

---

## `in property <string> status-prefix: "View: ";`
*Line 56 · property*

Input property "status-prefix".

---

## `in property <string> right-panel-label: "right";`
*Line 58 · property*

Input property "right-panel-label".

---

## `in property <string> menu-file-label: "File";`
*Line 62 · property*

Input property "menu-file-label".

---

## `in property <string> menu-exit-label: "Exit";`
*Line 64 · property*

Input property "menu-exit-label".

---

## `in property <string> menu-view-label: "View";`
*Line 66 · property*

Input property "menu-view-label".

---

## `in property <string> menu-home-label: "Home";`
*Line 68 · property*

Input property "menu-home-label".

---

## `in property <string> menu-list-label: "List";`
*Line 70 · property*

Input property "menu-list-label".

---

## `in property <string> menu-settings-label: "Settings";`
*Line 72 · property*

Input property "menu-settings-label".

---

## `in property <string> menu-help-label: "Help";`
*Line 74 · property*

Input property "menu-help-label".

---

## `in property <string> menu-about-label: "About";`
*Line 76 · property*

Input property "menu-about-label".

---

## `callback menu-action(string);`
*Line 80 · callback*

Callback fired for m en u a ct io n.

---

## `callback toolbar-clicked(string);`
*Line 82 · callback*

Callback fired for t oo lb ar c li ck ed.

---

## `in-out property <float> row-top-ratio: Sizes.fill;`
*Line 86 · property*

Two-way property "row-top-ratio".

---

## `in-out property <float> row-main-ratio: Sizes.ten;`
*Line 88 · property*

Two-way property "row-main-ratio".

---

## `in-out property <float> row-bottom-ratio: Sizes.fill;`
*Line 90 · property*

Two-way property "row-bottom-ratio".

---

## `in-out property <length> col-left-width:  Spacing.xl * Sizes.four + Spacing.md;   // 208px`
*Line 94 · property*

Two-way property "col-left-width".

---

## `in-out property <length> col-right-width: Spacing.xl * Sizes.five;                 // 240px`
*Line 96 · property*

Two-way property "col-right-width".

---

## `callback navigate(string);`
*Line 103 · callback*

Callback fired for n av ig at e.

---

