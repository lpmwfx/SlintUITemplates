# ui/widgets/date-picker.slint

## `export component DatePicker inherits Rectangle {`

*Line 14 · component*

**undocumented**

---

## `in property <string> value: "";`

*Line 23 · property*

**undocumented**

---

## `in property <string> label: "";`

*Line 32 · property*

**undocumented**

---

## `in property <string> placeholder: "Select date";`

*Line 41 · property*

**undocumented**

---

## `in property <string> error: "";`

*Line 50 · property*

**undocumented**

---

## `in property <bool>   enabled: true;`

*Line 59 · property*

**undocumented**

---

## `in property <string> icon-font: "Segoe Fluent Icons";`

*Line 68 · property*

**undocumented**

---

## `callback changed(string);`

*Line 77 · callback*

**undocumented**

---

## `private property <int>    initial-year:    2026;`

*Line 88 · property*

**undocumented**

---

## `private property <int>    month-offset:    Sizes.zero;`

*Line 97 · property*

**undocumented**

---

## `private property <int>    months-per-year: Sizes.twelve;`

*Line 106 · property*

**undocumented**

---

## `private property <string> date-sep:  "-";`

*Line 117 · property*

**undocumented**

---

## `private property <string> zero-pad:  "0";`

*Line 126 · property*

**undocumented**

---

## `private property <int>    display-month: Math.mod(root.month-offset, root.months-per-year) + Sizes.one;`

*Line 135 · property*

**undocumented**

---

## `private property <int>    display-year:  root.initial-year + Math.floor(root.month-offset / root.months-per-year);`

*Line 144 · property*

**undocumented**

---

## `private property <string> ym-prefix:`

*Line 156 · property*

**undocumented**

---

## `private property <length> cell-size:  Spacing.control-md + Spacing.xs;`

*Line 168 · property*

**undocumented**

---

## `private property <length> popup-w:    cell-size * Sizes.seven + Spacing.sm * Sizes.two + Spacing.xs * Sizes.six;`

*Line 177 · property*

**undocumented**

---

## `private property <length> popup-h:    Spacing.xl + Spacing.xl + cell-size * Sizes.six + Spacing.sm * Sizes.five + Spacing.md * Sizes.two;`

*Line 186 · property*

**undocumented**

---

