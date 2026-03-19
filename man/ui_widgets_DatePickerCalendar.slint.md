# ui/widgets/DatePickerCalendar.slint

## `export component DatePickerCalendar inherits Rectangle {`

*Line 15 · component*

**undocumented**

---

## `in property <int> month-offset: Sizes.zero;`

*Line 24 · property*

**undocumented**

---

## `in property <int> initial-year: 2026;`

*Line 33 · property*

**undocumented**

---

## `callback prev-month();`

*Line 42 · callback*

**undocumented**

---

## `callback next-month();`

*Line 51 · callback*

**undocumented**

---

## `callback day-selected(int);`

*Line 60 · callback*

**undocumented**

---

## `private property <int> months-per-year:  Sizes.twelve;`

*Line 71 · property*

**undocumented**

---

## `private property <int> zeller-shift:     Sizes.five;`

*Line 80 · property*

**undocumented**

---

## `private property <int> zeller-factor:    13;`

*Line 89 · property*

**undocumented**

---

## `private property <int> zeller-div:       Sizes.five;`

*Line 98 · property*

**undocumented**

---

## `private property <int> days-cols:        Sizes.seven;`

*Line 107 · property*

**undocumented**

---

## `private property <int> leap-cycle:       Sizes.four;`

*Line 116 · property*

**undocumented**

---

## `private property <int> century-cycle:    Sizes.hundred;`

*Line 125 · property*

**undocumented**

---

## `private property <int> grand-cycle:      400;`

*Line 134 · property*

**undocumented**

---

## `private property <int> feb-normal:       28;`

*Line 143 · property*

**undocumented**

---

## `private property <int> feb-leap:         29;`

*Line 152 · property*

**undocumented**

---

## `private property <int> short-month-days: Sizes.thirty;`

*Line 161 · property*

**undocumented**

---

## `private property <int> long-month-days:  31;`

*Line 170 · property*

**undocumented**

---

## `private property <length> cell-size:     Spacing.control-md + Spacing.xs;`

*Line 179 · property*

**undocumented**

---

## `private property <string> nav-prev:      "‹";`

*Line 188 · property*

**undocumented**

---

## `private property <string> nav-next:      "›";`

*Line 197 · property*

**undocumented**

---

## `private property <int> display-month: Math.mod(root.month-offset, root.months-per-year) + Sizes.one;`

*Line 208 · property*

**undocumented**

---

## `private property <int> display-year:  root.initial-year + Math.floor(root.month-offset / root.months-per-year);`

*Line 217 · property*

**undocumented**

---

## `private property <[string]> month-names: [`

*Line 227 · property*

**undocumented**

---

## `private property <int> zy: root.display-month <= Sizes.two ? root.display-year - Sizes.one : root.display-year;`

*Line 241 · property*

**undocumented**

---

## `private property <int> zm: root.display-month <= Sizes.two ? root.display-month + root.months-per-year : root.display-month;`

*Line 250 · property*

**undocumented**

---

## `private property <int> zeller-h: Math.mod(Sizes.one + (root.zeller-factor * (root.zm + Sizes.one) / root.zeller-div) + root.zy + root.zy / root.leap-cycle - root.zy / root.century-cycle + root.zy / root.grand-cycle, root.days-cols);`

*Line 259 · property*

**undocumented**

---

## `private property <int> first-weekday: Math.mod(root.zeller-h + root.zeller-shift, root.days-cols);`

*Line 268 · property*

**undocumented**

---

## `private property <int> days-in-month:`

*Line 278 · property*

**undocumented**

---

