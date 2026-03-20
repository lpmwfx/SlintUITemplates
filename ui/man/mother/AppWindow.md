# `mother/AppWindow.slint`

## `export component AppWindow inherits Window`
*Line 20 · component*

A pp wi nd ow component.

---

## `private property <string> app-title: "SlintUITemplates";`
*Line 25 · property*

Private property "app-title" used internally.

---

## `in-out property <string> theme-mode: Variants.system;`
*Line 29 · property*

Two-way property "theme-mode".

---

## `in-out property <string> material: Variants.solid;  // "solid" | "mica" | "acrylic"`
*Line 31 · property*

Two-way property "material".

---

## `in-out property <string> platform: Variants.windows;`
*Line 33 · property*

Two-way property "platform".

---

## `in-out property <[PanelItem]> panels;`
*Line 51 · property*

Two-way property "panels".

---

## `callback panel-dragged(int, float, float);   // id, dx, dy in logical px`
*Line 55 · callback*

Callback fired for p an el d ra gg ed.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
