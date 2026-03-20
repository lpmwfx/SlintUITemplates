# `widgets/DatePickerCalendar.slint`

## `export component DatePickerCalendar inherits Rectangle`
*Line 8 · component*

D at ep ic ke rc al en da r component.

---

## `in property <int> month-offset: Sizes.zero;`
*Line 10 · property*

Input property "month-offset".

---

## `in property <int> initial-year: 2026;`
*Line 12 · property*

Input property "initial-year".

---

## `callback prev-month();`
*Line 14 · callback*

Callback fired for p re v m on th.

---

## `callback next-month();`
*Line 16 · callback*

Callback fired for n ex t m on th.

---

## `callback day-selected(int);`
*Line 18 · callback*

Callback fired for d ay s el ec te d.

---

## `private property <int> months-per-year:  Sizes.twelve;`
*Line 22 · property*

Private property "months-per-year" used internally.

---

## `private property <int> zeller-shift:     Sizes.five;`
*Line 24 · property*

Private property "zeller-shift" used internally.

---

## `private property <int> zeller-factor:    13;`
*Line 26 · property*

Private property "zeller-factor" used internally.

---

## `private property <int> zeller-div:       Sizes.five;`
*Line 28 · property*

Private property "zeller-div" used internally.

---

## `private property <int> days-cols:        Sizes.seven;`
*Line 30 · property*

Private property "days-cols" used internally.

---

## `private property <int> leap-cycle:       Sizes.four;`
*Line 32 · property*

Private property "leap-cycle" used internally.

---

## `private property <int> century-cycle:    Sizes.hundred;`
*Line 34 · property*

Private property "century-cycle" used internally.

---

## `private property <int> grand-cycle:      400;`
*Line 36 · property*

Private property "grand-cycle" used internally.

---

## `private property <int> feb-normal:       28;`
*Line 38 · property*

Private property "feb-normal" used internally.

---

## `private property <int> feb-leap:         29;`
*Line 40 · property*

Private property "feb-leap" used internally.

---

## `private property <int> short-month-days: Sizes.thirty;`
*Line 42 · property*

Private property "short-month-days" used internally.

---

## `private property <int> long-month-days:  31;`
*Line 44 · property*

Private property "long-month-days" used internally.

---

## `private property <length> cell-size:     Spacing.control-md + Spacing.xs;`
*Line 46 · property*

Private property "cell-size" used internally.

---

## `private property <string> nav-prev:      "‹";`
*Line 48 · property*

Private property "nav-prev" used internally.

---

## `private property <string> nav-next:      "›";`
*Line 50 · property*

Private property "nav-next" used internally.

---

## `private property <int> display-month: Math.mod(root.month-offset, root.months-per-year) + Sizes.one;`
*Line 54 · property*

Private property "display-month" used internally.

---

## `private property <int> display-year:  root.initial-year + Math.floor(root.month-offset / root.months-per-year);`
*Line 56 · property*

Private property "display-year" used internally.

---

## `private property <[string]> month-names: [`
*Line 59 · property*

Private property "month-names" used internally.

---

## `private property <int> zy: root.display-month <= Sizes.two ? root.display-year - Sizes.one : root.display-year;`
*Line 66 · property*

Private property "zy" used internally.

---

## `private property <int> zm: root.display-month <= Sizes.two ? root.display-month + root.months-per-year : root.display-month;`
*Line 68 · property*

Private property "zm" used internally.

---

## `private property <int> zeller-h: Math.mod(Sizes.one + (root.zeller-factor * (root.zm + Sizes.one) / root.zeller-div) + root.zy + root.zy / root.leap-cycle - root.zy / root.century-cycle + root.zy / root.grand-cycle, root.days-cols);`
*Line 70 · property*

Private property "zeller-h" used internally.

---

## `private property <int> first-weekday: Math.mod(root.zeller-h + root.zeller-shift, root.days-cols);`
*Line 72 · property*

Private property "first-weekday" used internally.

---

## `private property <int> days-in-month:`
*Line 75 · property*

Private property "days-in-month" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
