# `widgets/text-area.slint`

## `export component TextArea inherits Rectangle`
*Line 6 · component*

T ex ta re a component.

---

## `in property <string> label: "";`
*Line 8 · property*

Input property "label".

---

## `in property <string> value: "";`
*Line 10 · property*

Input property "value".

---

## `in property <string> placeholder: "";`
*Line 12 · property*

Input property "placeholder".

---

## `in property <string> error: "";`
*Line 14 · property*

Input property "error".

---

## `in property <int>    rows: Sizes.four;`
*Line 16 · property*

Input property "rows".

---

## `in property <bool>   enabled: true;`
*Line 18 · property*

Input property "enabled".

---

## `callback changed(string);`
*Line 20 · callback*

Callback fired for c ha ng ed.

---

## `private property <length> area-height: root.rows * (Type.body-size + Spacing.xs) + Spacing.sm * Sizes.two;`
*Line 23 · property*

Private property "area-height" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
