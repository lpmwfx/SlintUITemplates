# `docs/docs-app.slint`

## `export component DocsApp inherits Window`
*Line 18 · component*

D oc sa pp component.

---

## `in property <string>             window-title:     "SlintUITemplates — Docs";`
*Line 20 · property*

Input property "window-title".

---

## `in property <string>             menu-file-label:  "File";`
*Line 22 · property*

Input property "menu-file-label".

---

## `in property <string>             menu-theme-label: "Theme";`
*Line 24 · property*

Input property "menu-theme-label".

---

## `in-out property <string>         active-view: "getting-started";`
*Line 35 · property*

Two-way property "active-view".

---

## `in-out property <[NavItem]>      nav-items: [];`
*Line 37 · property*

Two-way property "nav-items".

---

## `in-out property <[DocBlock]>     doc-blocks: [];`
*Line 39 · property*

Two-way property "doc-blocks".

---

## `in-out property <string>         doc-title: "";`
*Line 41 · property*

Two-way property "doc-title".

---

## `in-out property <string>         status-text: "Ready";`
*Line 43 · property*

Two-way property "status-text".

---

## `in property <[ShellToolbarItem]> toolbar-items: [];`
*Line 45 · property*

Input property "toolbar-items".

---

## `in property <bool>               show-toolbar: false;`
*Line 47 · property*

Input property "show-toolbar".

---

## `in-out property <bool>   sidebar-collapsed: false;`
*Line 50 · property*

Two-way property "sidebar-collapsed".

---

## `in-out property <length> col-left-width: Spacing.xl * 4 + Spacing.md;`
*Line 52 · property*

Two-way property "col-left-width".

---

## `private property <length> drag-base-width: Spacing.xl * 4 + Spacing.md;`
*Line 58 · property*

Private property "drag-base-width" used internally.

---

## `callback navigate(string);`
*Line 69 · callback*

Callback fired for n av ig at e.

---

## `callback toolbar-clicked(string);`
*Line 71 · callback*

Callback fired for t oo lb ar c li ck ed.

---

## `callback request-bg-style(string);`
*Line 73 · callback*

Callback fired for r eq ue st b g s ty le.

---

## `callback toggle-group(string);   // Rust-siden håndterer hidden-toggle på nav-items`
*Line 75 · callback*

Callback fired for t og gl e g ro up.

---

## `callback drag-sidebar(dx: float);`
*Line 77 · callback*

Callback fired for d ra g s id eb ar.

---

## `callback drag-anchor();   // called by DragHandle on press+release to re-anchor drag-base-width`
*Line 79 · callback*

Callback fired for d ra g a nc ho r.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
