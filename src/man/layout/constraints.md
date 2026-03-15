# `layout/constraints.rs`

## `pub struct Constraint`
*Line 14 · struct*

Ratio-space min/max constraint that bounds a panel's resizable range.
C on st ra in t struct.

---

## `pub fn new(min: f32, max: f32) -> Self`
*Line 23 · fn*

Create a constraint with the given min and max ratio bounds.

---

## `pub fn clamp(&self, ratio: f32) -> f32`
*Line 31 · fn*

Clamp a ratio value to the `[min, max]` range.

---

