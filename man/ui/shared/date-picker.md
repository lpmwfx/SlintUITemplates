# `ui/shared/date-picker.slint`

## `export component DatePicker inherits Rectangle`
*Line 3 · component*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in-out property <string> value: "";`
*Line 4 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <string> label: "";`
*Line 5 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <string> placeholder: "Select date";`
*Line 6 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <string> error: "";`
*Line 7 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <bool>   enabled: true;`
*Line 8 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback changed(string);`
*Line 9 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <int> display-year:  2026;`
*Line 11 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <int> display-month: 1;`
*Line 12 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> cell-size:   Spacing.control-md + Spacing.xs;   // 36px`
*Line 15 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> popup-w:     cell-size * 7 + Spacing.sm * 2 + Spacing.xs * 6;`
*Line 16 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> popup-h:     Spacing.xl + Spacing.xl + cell-size * 6 + Spacing.sm * 5 + Spacing.md * 2;`
*Line 17 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <int> zy: root.display-month <= 2 ? root.display-year - 1 : root.display-year;`
*Line 20 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <int> zm: root.display-month <= 2 ? root.display-month + 12 : root.display-month;`
*Line 21 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <int> zeller-h: Math.mod(1 + (13 * (root.zm + 1) / 5) + root.zy + root.zy / 4 - root.zy / 100 + root.zy / 400, 7);`
*Line 23 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <int> first-weekday: Math.mod(root.zeller-h + 5, 7);`
*Line 25 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <int> days-in-month:`
*Line 27 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <[string]> month-names: [`
*Line 35 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <bool> picker-open: false;`
*Line 40 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

