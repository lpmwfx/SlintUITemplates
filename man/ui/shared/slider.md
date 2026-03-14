# `ui/shared/slider.slint`

## `export component Slider inherits Rectangle`
*Line 3 · component*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <float> value: 0.0;`
*Line 4 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <float>  min: 0.0;`
*Line 5 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <float>  max: 1.0;`
*Line 6 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <float>  step: 0.0;`
*Line 7 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <string> label: "";`
*Line 8 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <bool>   show-value: false;`
*Line 9 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <bool>   enabled: true;`
*Line 10 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback changed(float);`
*Line 11 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <float>  fill-ratio: (root.value - root.min) / (root.max - root.min);`
*Line 13 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> track-h:    Spacing.xs;           // 4px`
*Line 14 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> thumb-size: Spacing.md;           // 16px`
*Line 15 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

