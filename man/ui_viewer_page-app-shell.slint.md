# ui/viewer/page-app-shell.slint

## `export component PageAppShell inherits VerticalLayout {`

*Line 17 · component*

**undocumented**

---

## `private property <string>    layout-mode: Variants.windows;`

*Line 20 · property*

**undocumented**

---

## `private property <string> view-home: "home";`

*Line 24 · property*

**undocumented**

---

## `private property <string> view-list: "list";`

*Line 27 · property*

**undocumented**

---

## `private property <string> view-settings: "settings";`

*Line 30 · property*

**undocumented**

---

## `private property <string>    active-windows: root.view-home;`

*Line 35 · property*

**undocumented**

---

## `private property <string>    active-android: root.view-home;`

*Line 38 · property*

**undocumented**

---

## `private property <string>    active-view: root.layout-mode == Variants.windows ? root.active-windows : root.active-android;`

*Line 41 · property*

**undocumented**

---

## `private property <string> shell-label-win: "App Shell — Windows (Fluent 2)";`

*Line 46 · property*

**undocumented**

---

## `private property <string> shell-label-android: "App Shell — Android (Material 3)";`

*Line 49 · property*

**undocumented**

---

## `private property <string> btn-to-android: "→ Android";`

*Line 52 · property*

**undocumented**

---

## `private property <string> btn-to-windows: "→ Windows";`

*Line 55 · property*

**undocumented**

---

## `private property <string> status-prefix: "Active: ";`

*Line 58 · property*

**undocumented**

---

## `private property <string> mobile-title: "App";`

*Line 61 · property*

**undocumented**

---

## `private property <length> phone-w: Sizes.comp-380;`

*Line 64 · property*

**undocumented**

---

## `private property <length> phone-border: Sizes.border-w-2;`

*Line 67 · property*

**undocumented**

---

## `private property <[NavItem]> nav-items: [`

*Line 71 · property*

**undocumented**

---

