# ui/widgets/date-picker.slint

## `export component DatePicker inherits Rectangle {`

*Line 8 · component*

**undocumented**

---

## `in property <string> value: "";`

*Line 11 · property*

**undocumented**

---

## `in property <string> label: "";`

*Line 14 · property*

**undocumented**

---

## `in property <string> placeholder: "Select date";`

*Line 17 · property*

**undocumented**

---

## `in property <string> error: "";`

*Line 20 · property*

**undocumented**

---

## `in property <bool>   enabled: true;`

*Line 23 · property*

**undocumented**

---

## `in property <string> icon-font: "Segoe Fluent Icons";`

*Line 26 · property*

**undocumented**

---

## `callback changed(string);`

*Line 29 · callback*

**undocumented**

---

## `private property <int>    initial-year:    2026;`

*Line 34 · property*

**undocumented**

---

## `private property <int>    month-offset:    Sizes.zero;`

*Line 37 · property*

**undocumented**

---

## `private property <int>    months-per-year: Sizes.twelve;`

*Line 40 · property*

**undocumented**

---

## `private property <string> date-sep:  "-";`

*Line 45 · property*

**undocumented**

---

## `private property <string> zero-pad:  "0";`

*Line 48 · property*

**undocumented**

---

## `private property <int>    display-month: Math.mod(root.month-offset, root.months-per-year) + Sizes.one;`

*Line 51 · property*

**undocumented**

---

## `private property <int>    display-year:  root.initial-year + Math.floor(root.month-offset / root.months-per-year);`

*Line 54 · property*

**undocumented**

---

## `private property <string> ym-prefix:`

*Line 60 · property*

**undocumented**

---

## `private property <length> cell-size:  Spacing.control-md + Spacing.xs;`

*Line 66 · property*

**undocumented**

---

## `private property <length> popup-w:    cell-size * Sizes.seven + Spacing.sm * Sizes.two + Spacing.xs * Sizes.six;`

*Line 69 · property*

**undocumented**

---

## `private property <length> popup-h:    Spacing.xl + Spacing.xl + cell-size * Sizes.six + Spacing.sm * Sizes.five + Spacing.md * Sizes.two;`

*Line 72 · property*

**undocumented**

---

