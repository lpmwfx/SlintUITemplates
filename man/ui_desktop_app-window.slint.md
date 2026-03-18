# ui/desktop/app-window.slint

## `export component AppWindow inherits Window {`

*Line 20 · component*

**undocumented**

---

## `in property <string> window-title: "SlintUI Templates";`

*Line 23 · property*

**undocumented**

---

## `in property <string>    active-view: NavId.home;`

*Line 37 · property*

**undocumented**

---

## `in property <[NavItem]> nav-items: [`

*Line 42 · property*

**undocumented**

---

## `in property <bool>               show-toolbar:  false;`

*Line 51 · property*

**undocumented**

---

## `in property <[ShellToolbarItem]> toolbar-items: [];`

*Line 54 · property*

**undocumented**

---

## `in property <string> status-text: "";`

*Line 59 · property*

**undocumented**

---

## `in property <string> status-prefix: "View: ";`

*Line 62 · property*

**undocumented**

---

## `in property <string> right-panel-label: "right";`

*Line 65 · property*

**undocumented**

---

## `in property <string> menu-file-label: "File";`

*Line 70 · property*

**undocumented**

---

## `in property <string> menu-exit-label: "Exit";`

*Line 73 · property*

**undocumented**

---

## `in property <string> menu-view-label: "View";`

*Line 76 · property*

**undocumented**

---

## `in property <string> menu-home-label: "Home";`

*Line 79 · property*

**undocumented**

---

## `in property <string> menu-list-label: "List";`

*Line 82 · property*

**undocumented**

---

## `in property <string> menu-settings-label: "Settings";`

*Line 85 · property*

**undocumented**

---

## `in property <string> menu-help-label: "Help";`

*Line 88 · property*

**undocumented**

---

## `in property <string> menu-about-label: "About";`

*Line 91 · property*

**undocumented**

---

## `callback menu-action(string);`

*Line 96 · callback*

**undocumented**

---

## `callback toolbar-clicked(string);`

*Line 99 · callback*

**undocumented**

---

## `in property <float> row-top-ratio: Sizes.fill;`

*Line 104 · property*

**undocumented**

---

## `in property <float> row-main-ratio: Sizes.ten;`

*Line 107 · property*

**undocumented**

---

## `in property <float> row-bottom-ratio: Sizes.fill;`

*Line 110 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 122 · callback*

**undocumented**

---

