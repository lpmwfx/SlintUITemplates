# `layout/solver.rs`

## `pub struct SolvedItem`
*Line 14 · struct*

A positioned layout element with normalized coordinates ready for rendering.
S ol ve di te m struct.

---

## `pub enum ItemKind { Panel, HandleH, HandleV }  impl ItemKind`
*Line 34 · enum*

Discriminates between content panels and drag handles in a solved layout.
I te mk in d enum.

---

## `pub fn as_str(&self) -> &'static str`
*Line 38 · fn*

Return a stable string identifier for this item kind.

---

## `pub struct Solver`
*Line 48 · struct*

Walks a `PanelNode` tree and emits flat `SolvedItem` rectangles with normalized coordinates.

---

## `pub fn new(win_w: f32, win_h: f32) -> Self`
*Line 58 · fn*

Create a solver targeting the given window pixel dimensions.

---

## `pub fn solve(mut self, root: &PanelNode) -> Vec<SolvedItem>`
*Line 63 · fn*

Consume the solver, recursively visit the tree, and return all solved items.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
