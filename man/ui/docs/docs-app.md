# `ui/docs/docs-app.slint`

## `export component DocsApp inherits Window`
*Line 17 · component*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <string>             window-title:     "SlintUITemplates — Docs";`
*Line 18 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <string>             menu-file-label:  "File";`
*Line 19 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <string>             menu-theme-label: "Theme";`
*Line 20 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <string>         active-view: "getting-started";`
*Line 30 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <[NavItem]>      nav-items: [];`
*Line 31 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <[DocBlock]>     doc-blocks: [];`
*Line 32 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <string>         doc-title: "";`
*Line 33 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <string>         status-text: "Ready";`
*Line 34 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <[ShellToolbarItem]> toolbar-items: [];`
*Line 35 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <bool>               show-toolbar: false;`
*Line 36 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <bool>   sidebar-collapsed: false;`
*Line 38 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <length> col-left-width: Spacing.xl * 4 + Spacing.md;`
*Line 39 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> drag-base-width: Spacing.xl * 4 + Spacing.md;`
*Line 44 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback navigate(string);`
*Line 54 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback toolbar-clicked(string);`
*Line 55 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback request-bg-style(string);`
*Line 56 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback toggle-group(string);   // Rust-siden håndterer hidden-toggle på nav-items`
*Line 57 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback drag-sidebar(dx: float);`
*Line 58 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback drag-anchor();   // called by DragHandle on press+release to re-anchor drag-base-width`
*Line 59 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

