# `adapter/mod.rs`

## `pub struct AppAdapter_adp`
*Line 17 · struct*

Adapter layer between host app and Slint UI window.
Owns AppWindow handle, ViewConfig registry, menu actions, and all cached state.
A pp ad ap te r adp struct.

---

## `pub fn new() -> Result<Self, slint::PlatformError>`
*Line 41 · fn*

Create a new adapter, initializing the Slint window and wiring event dispatch.

---

## `pub fn apply_dsl(&mut self, dsl: &AppDsl)`
*Line 100 · fn*

Apply a validated `AppDsl` — composition rules already enforced.

---

## `pub fn run(self) -> Result<(), slint::PlatformError>`
*Line 107 · fn*

Show window and apply OS-level backdrop effects, then enter event loop.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
