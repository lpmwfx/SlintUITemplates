# ui/shell/desktop/shell.slint

## `export component DesktopShell inherits VerticalLayout {`

*Line 17 · component*

**undocumented**

---

## `in property <[NavItem]>          nav-items;`

*Line 20 · property*

**undocumented**

---

## `in property <[ShellToolbarItem]> toolbar-items;`

*Line 23 · property*

**undocumented**

---

## `in property <bool>               show-toolbar: false;`

*Line 26 · property*

**undocumented**

---

## `in property <string>             active-view: "";`

*Line 29 · property*

**undocumented**

---

## `in property <string>             status-text: "Ready";`

*Line 32 · property*

**undocumented**

---

## `in property <float>              progress: -1.0;`

*Line 35 · property*

**undocumented**

---

## `private property <length> nav-width: Sizes.panel-sm;`

*Line 40 · property*

**undocumented**

---

## `private property <bool>   nav-collapsed: nav-width <= Spacing.xl + Spacing.sm + Spacing.xs;`

*Line 43 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 47 · callback*

**undocumented**

---

## `callback toolbar-clicked(string);`

*Line 50 · callback*

**undocumented**

---

