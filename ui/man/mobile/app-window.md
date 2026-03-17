# `mobile/app-window.slint`

## `export component AppWindow inherits Window`
*Line 13 · component*

A pp wi nd ow component.

---

## `private property <string> app-title: "SlintUITemplates \u{2014} Mobile";`
*Line 20 · property*

Private property "app-title" used internally.

---

## `private property <string> icon-home: "\u{1F3E0}";`
*Line 22 · property*

Private property "icon-home" used internally.

---

## `private property <string> icon-list: "\u{1F4CB}";`
*Line 24 · property*

Private property "icon-list" used internally.

---

## `private property <string> icon-settings: "\u{2699}";`
*Line 26 · property*

Private property "icon-settings" used internally.

---

## `private property <string> status-prefix: "View: ";`
*Line 28 · property*

Private property "status-prefix" used internally.

---

## `in-out property <string>    active-view: NavId.home;`
*Line 31 · property*

Two-way property "active-view".

---

## `in-out property <[NavItem]> nav-items: [`
*Line 33 · property*

Two-way property "nav-items".

---

## `callback navigate(string);`
*Line 40 · callback*

Callback fired for n av ig at e.

---

