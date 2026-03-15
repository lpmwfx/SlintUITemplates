# `widgets/spinner.slint`

## `export component Spinner inherits Rectangle`
*Line 7 · component*

S pi nn er component.

---

## `in property <string> size:          Variants.md;`
*Line 9 · property*

Input property "size".

---

## `in property <color>  spinner-color: Colors.accent;`
*Line 11 · property*

Input property "spinner-color".

---

## `in property <bool>   active:        true;`
*Line 13 · property*

Input property "active".

---

## `private property <length> dim:`
*Line 16 · property*

Private property "dim" used internally.

---

## `private property <float>  anim-phase: 0.0;`
*Line 22 · property*

Private property "anim-phase" used internally.

---

## `private property <float>  anim-speed: 0.05;`
*Line 24 · property*

Private property "anim-speed" used internally.

---

## `private property <float>  anim-next: Math.mod(root.anim-phase + root.anim-speed, Sizes.two);`
*Line 26 · property*

Private property "anim-next" used internally.

---

## `private property <float>  pulse-min:  0.2;`
*Line 28 · property*

Private property "pulse-min" used internally.

---

## `private property <float>  track-alpha: 0.15;`
*Line 30 · property*

Private property "track-alpha" used internally.

---

## `private property <length> ring-width: Sizes.border-w-2;`
*Line 32 · property*

Private property "ring-width" used internally.

---

## `private property <float>  pulse-val: Math.abs(root.anim-phase - Sizes.fill) * (Sizes.fill - root.pulse-min) + root.pulse-min;`
*Line 35 · property*

Private property "pulse-val" used internally.

---

