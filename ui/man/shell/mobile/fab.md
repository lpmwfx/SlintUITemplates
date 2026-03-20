# `shell/mobile/fab.slint`

## `export component FAB inherits Rectangle`
*Line 8 · component*

F ab component.

---

## `in property <string> icon;`
*Line 10 · property*

Input property "icon".

---

## `in property <string> label: "";       // non-empty → extended FAB`
*Line 12 · property*

Input property "label".

---

## `in property <string> icon-font: "Segoe Fluent Icons";`
*Line 14 · property*

Input property "icon-font".

---

## `callback clicked();`
*Line 16 · callback*

Callback fired for c li ck ed.

---

## `private property <bool> extended: root.label != "";`
*Line 19 · property*

Private property "extended" used internally.

---

## `private property <float>  darken-amount: Sizes.ratio-xs;`
*Line 21 · property*

Private property "darken-amount" used internally.

---

## `private property <float>  shadow-alpha: Sizes.alpha-30;`
*Line 23 · property*

Private property "shadow-alpha" used internally.

---

## `private property <length> shadow-blur: Sizes.comp-6;`
*Line 25 · property*

Private property "shadow-blur" used internally.

---

## `private property <length> shadow-y: Sizes.border-w-3;`
*Line 27 · property*

Private property "shadow-y" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
