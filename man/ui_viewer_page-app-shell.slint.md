# ui/viewer/page-app-shell.slint

## `export component PageAppShell inherits VerticalLayout {`

*Line 23 · component*

**undocumented**

---

## `private property <string>    layout-mode: Variants.windows;`

*Line 32 · property*

**undocumented**

---

## `private property <string> view-home: "home";`

*Line 42 · property*

**undocumented**

---

## `private property <string> view-list: "list";`

*Line 51 · property*

**undocumented**

---

## `private property <string> view-settings: "settings";`

*Line 60 · property*

**undocumented**

---

## `private property <string>    active-windows: root.view-home;`

*Line 71 · property*

**undocumented**

---

## `private property <string>    active-android: root.view-home;`

*Line 80 · property*

**undocumented**

---

## `private property <string>    active-view: root.layout-mode == Variants.windows ? root.active-windows : root.active-android;`

*Line 89 · property*

**undocumented**

---

## `private property <string> shell-label-win: "App Shell — Windows (Fluent 2)";`

*Line 100 · property*

**undocumented**

---

## `private property <string> shell-label-android: "App Shell — Android (Material 3)";`

*Line 109 · property*

**undocumented**

---

## `private property <string> btn-to-android: "→ Android";`

*Line 118 · property*

**undocumented**

---

## `private property <string> btn-to-windows: "→ Windows";`

*Line 127 · property*

**undocumented**

---

## `private property <string> status-prefix: "Active: ";`

*Line 136 · property*

**undocumented**

---

## `private property <string> mobile-title: "App";`

*Line 145 · property*

**undocumented**

---

## `private property <length> phone-w: Sizes.comp-380;`

*Line 154 · property*

**undocumented**

---

## `private property <length> phone-border: Sizes.border-w-2;`

*Line 163 · property*

**undocumented**

---

## `private property <[NavItem]> nav-items: [`

*Line 173 · property*

**undocumented**

---

