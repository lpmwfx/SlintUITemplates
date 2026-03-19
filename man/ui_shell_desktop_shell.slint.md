# ui/shell/desktop/shell.slint

## `export component DesktopShell inherits VerticalLayout {`

*Line 24 · component*

**undocumented**

---

## `in property <[NavItem]>          nav-items;`

*Line 33 · property*

**undocumented**

---

## `in property <[ShellToolbarItem]> toolbar-items;`

*Line 42 · property*

**undocumented**

---

## `in property <bool>               show-toolbar: false;`

*Line 51 · property*

**undocumented**

---

## `in property <string>             active-view: "";`

*Line 60 · property*

**undocumented**

---

## `in property <string>             status-text: Strings.app-status-ready;`

*Line 69 · property*

**undocumented**

---

## `in property <float>              progress: -1.0;`

*Line 78 · property*

**undocumented**

---

## `private property <length> nav-width: Sizes.panel-sm;`

*Line 89 · property*

**undocumented**

---

## `private property <bool>   nav-collapsed: nav-width <= Spacing.xl + Spacing.sm + Spacing.xs;`

*Line 98 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 108 · callback*

**undocumented**

---

## `callback toolbar-clicked(string);`

*Line 117 · callback*

**undocumented**

---

