# `widgets/command-bar-flyout.slint`

## `export component CommandBarFlyout inherits Rectangle`
*Line 8 · component*

C om ma nd ba rf ly ou t component.

---

## `in property <[CbfAction]> primary: [];`
*Line 10 · property*

Input property "primary".

---

## `in property <[CbfAction]> secondary: [];`
*Line 12 · property*

Input property "secondary".

---

## `in property <length>      window-height;`
*Line 14 · property*

Input property "window-height".

---

## `callback action(string);`
*Line 16 · callback*

Callback fired for a ct io n.

---

## `private property <length> pop-x:        0px;`
*Line 19 · property*

Private property "pop-x" used internally.

---

## `private property <length> pop-y:        0px;`
*Line 21 · property*

Private property "pop-y" used internally.

---

## `private property <length> border-px:    1px;`
*Line 23 · property*

Private property "border-px" used internally.

---

## `private property <bool>   has-secondary: root.secondary.length > 0;`
*Line 25 · property*

Private property "has-secondary" used internally.

---

## `private property <length> sec-add:      root.has-secondary ? Spacing.xl : 0px;`
*Line 27 · property*

Private property "sec-add" used internally.

---

## `private property <length> primary-w:    root.primary.length * Spacing.xl + root.sec-add + Spacing.sm * 2;`
*Line 29 · property*

Private property "primary-w" used internally.

---

## `private property <length> primary-h:    Spacing.xl + Spacing.sm * 2;`
*Line 31 · property*

Private property "primary-h" used internally.

---

## `private property <length> secondary-w:  200px;`
*Line 33 · property*

Private property "secondary-w" used internally.

---

## `private property <length> secondary-h:  root.secondary.length * Spacing.xl + Spacing.sm * 2;`
*Line 35 · property*

Private property "secondary-h" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
