# `bindings/rhai/mod.rs`

## `pub mod api;`
*Line 2 · mod*

Rhai-to-AppAdapter function bindings registered on the scripting engine.

---

## `pub mod dsl;`
*Line 4 · mod*

DSL API — nav, toolbar, window, bg style registration.

---

## `pub fn build_engine(adapter: Rc<RefCell<AppAdapter>>) -> Engine`
*Line 13 · fn*

Build a Rhai Engine with all AppAdapter API functions registered.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
