# ui/widgets/DatePickerCalendar.slint

## `export component DatePickerCalendar inherits Rectangle {`

*Line 9 · component*

**undocumented**

---

## `in property <int> month-offset: Sizes.zero;`

*Line 12 · property*

**undocumented**

---

## `in property <int> initial-year: 2026;`

*Line 15 · property*

**undocumented**

---

## `callback prev-month();`

*Line 18 · callback*

**undocumented**

---

## `callback next-month();`

*Line 21 · callback*

**undocumented**

---

## `callback day-selected(int);`

*Line 24 · callback*

**undocumented**

---

## `private property <int> months-per-year:  Sizes.twelve;`

*Line 29 · property*

**undocumented**

---

## `private property <int> zeller-shift:     Sizes.five;`

*Line 32 · property*

**undocumented**

---

## `private property <int> zeller-factor:    13;`

*Line 35 · property*

**undocumented**

---

## `private property <int> zeller-div:       Sizes.five;`

*Line 38 · property*

**undocumented**

---

## `private property <int> days-cols:        Sizes.seven;`

*Line 41 · property*

**undocumented**

---

## `private property <int> leap-cycle:       Sizes.four;`

*Line 44 · property*

**undocumented**

---

## `private property <int> century-cycle:    Sizes.hundred;`

*Line 47 · property*

**undocumented**

---

## `private property <int> grand-cycle:      400;`

*Line 50 · property*

**undocumented**

---

## `private property <int> feb-normal:       28;`

*Line 53 · property*

**undocumented**

---

## `private property <int> feb-leap:         29;`

*Line 56 · property*

**undocumented**

---

## `private property <int> short-month-days: Sizes.thirty;`

*Line 59 · property*

**undocumented**

---

## `private property <int> long-month-days:  31;`

*Line 62 · property*

**undocumented**

---

## `private property <length> cell-size:     Spacing.control-md + Spacing.xs;`

*Line 65 · property*

**undocumented**

---

## `private property <string> nav-prev:      "‹";`

*Line 68 · property*

**undocumented**

---

## `private property <string> nav-next:      "›";`

*Line 71 · property*

**undocumented**

---

## `private property <int> display-month: Math.mod(root.month-offset, root.months-per-year) + Sizes.one;`

*Line 76 · property*

**undocumented**

---

## `private property <int> display-year:  root.initial-year + Math.floor(root.month-offset / root.months-per-year);`

*Line 79 · property*

**undocumented**

---

## `private property <[string]> month-names: [`

*Line 83 · property*

**undocumented**

---

## `private property <int> zy: root.display-month <= Sizes.two ? root.display-year - Sizes.one : root.display-year;`

*Line 91 · property*

**undocumented**

---

## `private property <int> zm: root.display-month <= Sizes.two ? root.display-month + root.months-per-year : root.display-month;`

*Line 94 · property*

**undocumented**

---

## `private property <int> zeller-h: Math.mod(Sizes.one + (root.zeller-factor * (root.zm + Sizes.one) / root.zeller-div) + root.zy + root.zy / root.leap-cycle - root.zy / root.century-cycle + root.zy / root.grand-cycle, root.days-cols);`

*Line 97 · property*

**undocumented**

---

## `private property <int> first-weekday: Math.mod(root.zeller-h + root.zeller-shift, root.days-cols);`

*Line 100 · property*

**undocumented**

---

## `private property <int> days-in-month:`

*Line 104 · property*

**undocumented**

---

