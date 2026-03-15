# `widgets/stepper.slint`

## `export struct StepItem`
*Line 6 · struct*

S te pi te m struct.

---

## `export component Stepper inherits Rectangle`
*Line 12 · component*

S te pp er component.

---

## `in property <[StepItem]> steps: [];`
*Line 14 · property*

Input property "steps".

---

## `in property <int>    active: Sizes.zero;`
*Line 16 · property*

Input property "active".

---

## `callback step-changed(int);`
*Line 18 · callback*

Callback fired for s te p c ha ng ed.

---

## `private property <length> circle-size:    Spacing.control-md - Spacing.xs;`
*Line 22 · property*

Private property "circle-size" used internally.

---

## `private property <length> connector-w:    Spacing.xl;`
*Line 24 · property*

Private property "connector-w" used internally.

---

## `private property <length> connector-h:    Sizes.border-w-2;`
*Line 26 · property*

Private property "connector-h" used internally.

---

