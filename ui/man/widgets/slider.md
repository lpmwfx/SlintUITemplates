# `widgets/slider.slint`

## `export component Slider inherits Rectangle`
*Line 6 · component*

S li de r component.

---

## `in property <float> value: Sizes.no-fill;`
*Line 8 · property*

Input property "value".

---

## `in property <float>  min: Sizes.no-fill;`
*Line 10 · property*

Input property "min".

---

## `in property <float>  max: Sizes.fill;`
*Line 12 · property*

Input property "max".

---

## `in property <float>  step: Sizes.no-fill;`
*Line 14 · property*

Input property "step".

---

## `in property <string> label: "";`
*Line 16 · property*

Input property "label".

---

## `in property <bool>   show-value: false;`
*Line 18 · property*

Input property "show-value".

---

## `in property <bool>   enabled: true;`
*Line 20 · property*

Input property "enabled".

---

## `callback changed(float);`
*Line 22 · callback*

Callback fired for c ha ng ed.

---

## `private property <float>  fill-ratio: (root.value - root.min) / (root.max - root.min);`
*Line 25 · property*

Private property "fill-ratio" used internally.

---

## `private property <length> track-h:    Spacing.xs;`
*Line 27 · property*

Private property "track-h" used internally.

---

## `private property <length> thumb-size: Spacing.md;`
*Line 29 · property*

Private property "thumb-size" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
