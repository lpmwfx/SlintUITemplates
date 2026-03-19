# ui/widgets/slider.slint

## `export component Slider inherits Rectangle {`

*Line 13 · component*

**undocumented**

---

## `in property <float> value: Sizes.no-fill;`

*Line 22 · property*

**undocumented**

---

## `in property <float>  min: Sizes.no-fill;`

*Line 31 · property*

**undocumented**

---

## `in property <float>  max: Sizes.fill;`

*Line 40 · property*

**undocumented**

---

## `in property <float>  step: Sizes.no-fill;`

*Line 49 · property*

**undocumented**

---

## `in property <string> label: "";`

*Line 58 · property*

**undocumented**

---

## `in property <bool>   show-value: false;`

*Line 67 · property*

**undocumented**

---

## `in property <bool>   enabled: true;`

*Line 76 · property*

**undocumented**

---

## `callback changed(float);`

*Line 85 · callback*

**undocumented**

---

## `private property <float>  fill-ratio: (root.value - root.min) / (root.max - root.min);`

*Line 95 · property*

**undocumented**

---

## `private property <length> track-h:    Spacing.xs;`

*Line 104 · property*

**undocumented**

---

## `private property <length> thumb-size: Spacing.md;`

*Line 113 · property*

**undocumented**

---

