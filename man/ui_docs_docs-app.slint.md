# ui/docs/docs-app.slint

## `export component DocsApp inherits Window {`

*Line 19 · component*

**undocumented**

---

## `in property <string>             window-title:     "SlintUITemplates — Docs";`

*Line 22 · property*

**undocumented**

---

## `in property <string>             menu-file-label:  "File";`

*Line 25 · property*

**undocumented**

---

## `in property <string>             menu-theme-label: "Theme";`

*Line 28 · property*

**undocumented**

---

## `in-out property <string>         active-view: "getting-started";`

*Line 40 · property*

**undocumented**

---

## `in-out property <[NavItem]>      nav-items: [];`

*Line 43 · property*

**undocumented**

---

## `in-out property <[DocBlock]>     doc-blocks: [];`

*Line 46 · property*

**undocumented**

---

## `in-out property <string>         doc-title: "";`

*Line 49 · property*

**undocumented**

---

## `in-out property <string>         status-text: "Ready";`

*Line 52 · property*

**undocumented**

---

## `in property <[ShellToolbarItem]> toolbar-items: [];`

*Line 55 · property*

**undocumented**

---

## `in property <bool>               show-toolbar: false;`

*Line 58 · property*

**undocumented**

---

## `in-out property <bool>   sidebar-collapsed: false;`

*Line 62 · property*

**undocumented**

---

## `in-out property <length> col-left-width: Spacing.xl * 4 + Spacing.md;`

*Line 65 · property*

**undocumented**

---

## `private property <length> drag-base-width: Spacing.xl * 4 + Spacing.md;`

*Line 72 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 84 · callback*

**undocumented**

---

## `callback toolbar-clicked(string);`

*Line 87 · callback*

**undocumented**

---

## `callback request-bg-style(string);`

*Line 90 · callback*

**undocumented**

---

## `callback toggle-group(string);   // Rust-siden håndterer hidden-toggle på nav-items`

*Line 93 · callback*

**undocumented**

---

## `callback drag-sidebar(dx: float);`

*Line 96 · callback*

**undocumented**

---

## `callback drag-anchor();   // called by DragHandle on press+release to re-anchor drag-base-width`

*Line 99 · callback*

**undocumented**

---

