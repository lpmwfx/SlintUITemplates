# ui/desktop/app-window.slint

## `export component AppWindow inherits Window {`

*Line 21 · component*

**undocumented**

---

## `in property <string> window-title: "SlintUI Templates";`

*Line 24 · property*

**undocumented**

---

## `in property <string>    active-view: NavId.home;`

*Line 38 · property*

**undocumented**

---

## `in property <[NavItem]> nav-items: [`

*Line 43 · property*

**undocumented**

---

## `in property <bool>               show-toolbar:  false;`

*Line 52 · property*

**undocumented**

---

## `in property <[ShellToolbarItem]> toolbar-items: [];`

*Line 55 · property*

**undocumented**

---

## `in property <string> status-text: Strings.label-default;`

*Line 60 · property*

**undocumented**

---

## `in property <string> status-prefix: Strings.status-view-prefix;`

*Line 63 · property*

**undocumented**

---

## `in property <string> right-panel-label: "right";`

*Line 66 · property*

**undocumented**

---

## `in property <string> menu-file-label: "File";`

*Line 71 · property*

**undocumented**

---

## `in property <string> menu-exit-label: "Exit";`

*Line 74 · property*

**undocumented**

---

## `in property <string> menu-view-label: "View";`

*Line 77 · property*

**undocumented**

---

## `in property <string> menu-home-label: "Home";`

*Line 80 · property*

**undocumented**

---

## `in property <string> menu-list-label: "List";`

*Line 83 · property*

**undocumented**

---

## `in property <string> menu-settings-label: "Settings";`

*Line 86 · property*

**undocumented**

---

## `in property <string> menu-help-label: "Help";`

*Line 89 · property*

**undocumented**

---

## `in property <string> menu-about-label: "About";`

*Line 92 · property*

**undocumented**

---

## `callback menu-action(string);`

*Line 97 · callback*

**undocumented**

---

## `callback toolbar-clicked(string);`

*Line 100 · callback*

**undocumented**

---

## `in property <float> row-top-ratio: Sizes.fill;`

*Line 105 · property*

**undocumented**

---

## `in property <float> row-main-ratio: Sizes.ten;`

*Line 108 · property*

**undocumented**

---

## `in property <float> row-bottom-ratio: Sizes.fill;`

*Line 111 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 123 · callback*

**undocumented**

---

