# `widgets/toggle.slint`

## `export component Toggle inherits Rectangle`
*Line 5 · component*

Binary on/off switch with an optional text label.

---

## `in property <bool> checked: false;`
*Line 7 · property*

Current checked state rendered by the switch.

---

## `in property <string> label: "";`
*Line 9 · property*

Optional label shown to the right of the switch.

---

## `callback toggled(bool);`
*Line 11 · callback*

Fired with the next checked state when the user toggles the control.

---

## `private property <length> track-width:  Spacing.control-lg;  // 40px`
*Line 15 · property*

Private property "track-width" used internally.

---

## `private property <length> track-height: Sizes.icon-md;       // 20px`
*Line 17 · property*

Private property "track-height" used internally.

---

## `private property <length> knob-size:    Sizes.icon-sm;       // 16px`
*Line 19 · property*

Private property "knob-size" used internally.

---

## `private property <length> knob-radius:  Spacing.sm;          // 8px`
*Line 21 · property*

Private property "knob-radius" used internally.

---

## `private property <length> knob-inset:   Sizes.border-w-2;    // 2px`
*Line 23 · property*

Private property "knob-inset" used internally.

---

## `private property <duration> slide-dur:  150ms;`
*Line 25 · property*

Private property "slide-dur" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
