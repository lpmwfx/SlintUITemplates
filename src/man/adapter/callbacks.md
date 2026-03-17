# `adapter/callbacks.rs`

## `pub fn on_navigate(&self, f: impl Fn(slint::SharedString) + 'static)`
*Line 4 · fn*

Register a handler for the navigate callback (nav item clicked).
Replaces any previously registered handler.

---

## `pub fn on_toolbar_clicked(&self, f: impl Fn(slint::SharedString) + 'static)`
*Line 10 · fn*

Register a handler for toolbar button clicks.
Stored in adapter cache; dispatched via the handler wired in new().

---

## `pub fn on_menu_action(&self, id: impl Into<String>, f: impl Fn() + 'static)`
*Line 16 · fn*

Register a handler for a specific MenuBar action id.
Multiple calls accumulate — each id maps to one handler.

---

