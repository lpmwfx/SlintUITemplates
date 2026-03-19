# src/adapter/mod.rs

## `pub struct AppAdapter_adp`

*Line 30 · struct*

Adapter layer between host app and Slint UI window.
Owns AppWindow handle, ViewConfig registry, menu actions, and all cached state.

# Example
```rust,no_run
use slint_ui_templates::AppAdapter;
use slint_ui_templates::dsl::{AppDsl, Nav};

let mut app = AppAdapter::new().unwrap();
let dsl = AppDsl::builder("My App")
    .nav(vec![Nav::new("home", "Home", "home")])
    .build().unwrap();
app.apply_dsl(&dsl);
app.run().unwrap();
```
A pp ad ap te r adp struct.

---

## `pub fn new() -> Result<Self, slint::PlatformError>`

*Line 54 · fn*

Create a new adapter, initializing the Slint window and wiring event dispatch.

---

## `pub fn apply_dsl(&mut self, dsl: &AppDsl)`

*Line 118 · fn*

Apply a validated `AppDsl` — composition rules already enforced.

---

## `pub fn run(self) -> Result<(), slint::PlatformError>`

*Line 125 · fn*

Show window and apply OS-level backdrop effects, then enter event loop.

---

