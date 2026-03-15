# `widgets/date-picker.slint`

## `export component DatePicker inherits Rectangle`
*Line 7 · component*

D at ep ic ke r component.

---

## `in property <string> value: "";`
*Line 9 · property*

Input property "value".

---

## `in property <string> label: "";`
*Line 11 · property*

Input property "label".

---

## `in property <string> placeholder: "Select date";`
*Line 13 · property*

Input property "placeholder".

---

## `in property <string> error: "";`
*Line 15 · property*

Input property "error".

---

## `in property <bool>   enabled: true;`
*Line 17 · property*

Input property "enabled".

---

## `in property <string> icon-font: "Segoe Fluent Icons";`
*Line 19 · property*

Input property "icon-font".

---

## `callback changed(string);`
*Line 21 · callback*

Callback fired for c ha ng ed.

---

## `private property <int>    initial-year:    2026;`
*Line 25 · property*

Private property "initial-year" used internally.

---

## `private property <int>    month-offset:    Sizes.zero;`
*Line 27 · property*

Private property "month-offset" used internally.

---

## `private property <int>    months-per-year: Sizes.twelve;`
*Line 29 · property*

Private property "months-per-year" used internally.

---

## `private property <string> date-sep:  "-";`
*Line 33 · property*

Private property "date-sep" used internally.

---

## `private property <string> zero-pad:  "0";`
*Line 35 · property*

Private property "zero-pad" used internally.

---

## `private property <int>    display-month: Math.mod(root.month-offset, root.months-per-year) + Sizes.one;`
*Line 37 · property*

Private property "display-month" used internally.

---

## `private property <int>    display-year:  root.initial-year + Math.floor(root.month-offset / root.months-per-year);`
*Line 39 · property*

Private property "display-year" used internally.

---

## `private property <string> ym-prefix:`
*Line 44 · property*

Private property "ym-prefix" used internally.

---

## `private property <length> cell-size:  Spacing.control-md + Spacing.xs;`
*Line 49 · property*

Private property "cell-size" used internally.

---

## `private property <length> popup-w:    cell-size * Sizes.seven + Spacing.sm * Sizes.two + Spacing.xs * Sizes.six;`
*Line 51 · property*

Private property "popup-w" used internally.

---

## `private property <length> popup-h:    Spacing.xl + Spacing.xl + cell-size * Sizes.six + Spacing.sm * Sizes.five + Spacing.md * Sizes.two;`
*Line 53 · property*

Private property "popup-h" used internally.

---

