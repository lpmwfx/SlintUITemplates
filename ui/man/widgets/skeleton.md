# `widgets/skeleton.slint`

## `export component Skeleton inherits Rectangle`
*Line 6 · component*

S ke le to n component.

---

## `in property <string> variant: root.variant-rect;   // text | circle | rect`
*Line 8 · property*

Input property "variant".

---

## `in property <bool>   animate: true;`
*Line 10 · property*

Input property "animate".

---

## `private property <string> variant-rect: "rect";`
*Line 14 · property*

Private property "variant-rect" used internally.

---

## `private property <float>    shimmer-step:    0.01;`
*Line 18 · property*

Private property "shimmer-step" used internally.

---

## `private property <float>    shimmer-max:     1.33;`
*Line 20 · property*

Private property "shimmer-max" used internally.

---

## `private property <float>    shimmer-reset:  -0.33;`
*Line 22 · property*

Private property "shimmer-reset" used internally.

---

## `private property <float>    shimmer-alpha:   0.4;`
*Line 24 · property*

Private property "shimmer-alpha" used internally.

---

## `private property <duration> frame-interval:  16ms;`
*Line 26 · property*

Private property "frame-interval" used internally.

---

## `private property <length>   text-radius: Sizes.border-w-2;`
*Line 28 · property*

Private property "text-radius" used internally.

---

## `private property <float> shimmer-pos: Sizes.no-fill;`
*Line 38 · property*

Private property "shimmer-pos" used internally.

---

## `private property <float> shimmer-next:`
*Line 40 · property*

Private property "shimmer-next" used internally.

---

