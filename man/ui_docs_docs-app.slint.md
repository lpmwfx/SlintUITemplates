# ui/docs/docs-app.slint

## `export component DocsApp inherits Window {`

*Line 25 · component*

**undocumented**

---

## `in property <string>             window-title:     "SlintUITemplates — Docs";`

*Line 34 · property*

**undocumented**

---

## `in property <string>             menu-file-label:  "File";`

*Line 43 · property*

**undocumented**

---

## `in property <string>             menu-theme-label: "Theme";`

*Line 52 · property*

**undocumented**

---

## `in-out property <string>         active-view: "getting-started";`

*Line 70 · property*

**undocumented**

---

## `in-out property <[NavItem]>      nav-items: [];`

*Line 79 · property*

**undocumented**

---

## `in-out property <[DocBlock]>     doc-blocks: [];`

*Line 88 · property*

**undocumented**

---

## `in-out property <string>         doc-title: "";`

*Line 97 · property*

**undocumented**

---

## `in-out property <string>         status-text: "Ready";`

*Line 106 · property*

**undocumented**

---

## `in property <[ShellToolbarItem]> toolbar-items: [];`

*Line 115 · property*

**undocumented**

---

## `in property <bool>               show-toolbar: false;`

*Line 124 · property*

**undocumented**

---

## `in-out property <bool>   sidebar-collapsed: false;`

*Line 134 · property*

**undocumented**

---

## `in-out property <length> col-left-width: Spacing.xl * 4 + Spacing.md;`

*Line 143 · property*

**undocumented**

---

## `private property <length> drag-base-width: Spacing.xl * 4 + Spacing.md;`

*Line 156 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 174 · callback*

**undocumented**

---

## `callback toolbar-clicked(string);`

*Line 183 · callback*

**undocumented**

---

## `callback request-bg-style(string);`

*Line 192 · callback*

**undocumented**

---

## `callback toggle-group(string);   // Rust-siden håndterer hidden-toggle på nav-items`

*Line 201 · callback*

**undocumented**

---

## `callback drag-sidebar(dx: float);`

*Line 210 · callback*

**undocumented**

---

## `callback drag-anchor();   // called by DragHandle on press+release to re-anchor drag-base-width`

*Line 219 · callback*

**undocumented**

---

