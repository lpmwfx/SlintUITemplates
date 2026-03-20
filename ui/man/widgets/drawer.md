# `widgets/drawer.slint`

## `export component Drawer inherits Rectangle`
*Line 5 · component*

D ra we r component.

---

## `in property <bool>   open: false;`
*Line 7 · property*

Input property "open".

---

## `in property <string> placement: "left";    // left | right`
*Line 9 · property*

Input property "placement".

---

## `in property <length> drawer-width: 280px;`
*Line 11 · property*

Input property "drawer-width".

---

## `in property <string> title: "";`
*Line 13 · property*

Input property "title".

---

## `callback close-requested();`
*Line 15 · callback*

Callback fired for c lo se r eq ue st ed.

---

## `private property <string> place-right: "right";`
*Line 18 · property*

Private property "place-right" used internally.

---

## `private property <length> border-px:   1px;`
*Line 20 · property*

Private property "border-px" used internally.

---

## `private property <length> zero-len:    0px;`
*Line 22 · property*

Private property "zero-len" used internally.

---

## `private property <length> no-gap:      0px;`
*Line 24 · property*

Private property "no-gap" used internally.

---

## `private property <duration> anim-dur:  200ms;`
*Line 26 · property*

Private property "anim-dur" used internally.

---

## `private property <length> panel-x:`
*Line 28 · property*

Private property "panel-x" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
