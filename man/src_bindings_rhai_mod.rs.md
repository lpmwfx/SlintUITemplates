# src/bindings/rhai/mod.rs

## `pub mod api;`

*Line 18 · mod*

Rhai-to-AppAdapter function bindings registered on the scripting engine.

# Limitations

**Thread-Safety**: The Slint UI framework is single-threaded (`!Send + !Sync`). Rhai's default
`Engine` has a `sync` feature that requires all closures and captured values to be `Send + Sync`.
This creates a fundamental incompatibility: the AppAdapter holds an `AppWindow` which is not
`Send`, yet Rhai's closures capture the adapter to register callbacks.

**Workaround**: This module disables Rhai's `sync` feature. Rhai engine instances remain
thread-local and must not be shared across threads. Use this engine only from the main UI thread.

**Implication**: Applications using Rhai scripting cannot spawn background threads that
interact with the UI. All Rhai scripts execute on the main UI thread.

---

## `pub mod dsl;`

*Line 20 · mod*

DSL API — nav, toolbar, window, bg style registration.

---

## `pub fn build_engine(adapter: Rc<RefCell<AppAdapter>>) -> Engine`

*Line 29 · fn*

Build a Rhai Engine with all AppAdapter API functions registered.

---

