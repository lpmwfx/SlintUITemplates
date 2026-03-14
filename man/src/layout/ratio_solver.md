# `src/layout/ratio_solver.rs`

## `pub struct Panel`
*Line 13 · struct*

A resizable panel with a normalized ratio and min/max constraints.

---

## `pub fn new(id: impl Into<String>, ratio: f32) -> Self`
*Line 20 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn with_constraint(mut self, min: f32, max: f32) -> Self`
*Line 24 · fn*

> ⚠ **undocumented** — add a `///` doc comment

---

## `pub fn drag(panels: &mut [Panel], handle_idx: usize, delta: f32) -> f32`
*Line 36 · fn*

Apply a drag delta to the handle between panels[handle_idx] and panels[handle_idx + 1].

`delta` is a signed ratio fraction: positive = handle moves right/down.
Both neighbors update; sum of all ratios stays at 1.0.

Returns the actual transfer applied (may be smaller than `delta` due to constraints).

---

## `pub fn normalize(panels: &mut [Panel])`
*Line 61 · fn*

Normalize panel ratios so they sum to exactly 1.0.
Call this after constructing panels to ensure invariant holds.

---

## `pub fn check_sum(panels: &[Panel]) -> bool`
*Line 71 · fn*

Verify that panel ratios sum to approximately 1.0.

---

