# ui/widgets/slider.slint

## `export component Slider inherits Rectangle {`

*Line 7 · component*

**undocumented**

---

## `in property <float> value: Sizes.no-fill;`

*Line 10 · property*

**undocumented**

---

## `in property <float>  min: Sizes.no-fill;`

*Line 13 · property*

**undocumented**

---

## `in property <float>  max: Sizes.fill;`

*Line 16 · property*

**undocumented**

---

## `in property <float>  step: Sizes.no-fill;`

*Line 19 · property*

**undocumented**

---

## `in property <string> label: "";`

*Line 22 · property*

**undocumented**

---

## `in property <bool>   show-value: false;`

*Line 25 · property*

**undocumented**

---

## `in property <bool>   enabled: true;`

*Line 28 · property*

**undocumented**

---

## `callback changed(float);`

*Line 31 · callback*

**undocumented**

---

## `private property <float>  fill-ratio: (root.value - root.min) / (root.max - root.min);`

*Line 35 · property*

**undocumented**

---

## `private property <length> track-h:    Spacing.xs;`

*Line 38 · property*

**undocumented**

---

## `private property <length> thumb-size: Spacing.md;`

*Line 41 · property*

**undocumented**

---

