# `adapter/state.rs`

## `pub fn set_active_view(&self, name: &str)`
*Line 7 · fn*

Switch the active view by name — updates cache and UI.

---

## `pub fn get_active_view(&self) -> String`
*Line 13 · fn*

Get the currently active view name from the adapter cache.

---

## `pub fn set_dark_mode(&self, on: bool)`
*Line 18 · fn*

Set dark mode on/off — updates cache and UI Theme global.

---

## `pub fn get_dark_mode(&self) -> bool`
*Line 24 · fn*

Check if dark mode is currently active (reads from cache).

---

## `pub fn set_status(&self, text: &str)`
*Line 29 · fn*

Set the status bar text — updates cache and UI.

---

## `pub fn get_status(&self) -> String`
*Line 35 · fn*

Get the current status bar text from the adapter cache.

---

## `pub fn get_zoom(&self) -> f32`
*Line 40 · fn*

Get the current UI zoom scale from the adapter cache.

---

## `pub fn get_row_top_ratio(&self) -> f32`
*Line 45 · fn*

Get the top row ratio from the adapter cache.

---

## `pub fn get_row_main_ratio(&self) -> f32`
*Line 50 · fn*

Get the main row ratio from the adapter cache.

---

## `pub fn set_platform(&self, platform: Platform)`
*Line 56 · fn*

Set the active platform — updates cache and Theme.platform token router.
Call once at startup before `run()` (or when switching platforms).

---

## `pub fn get_platform(&self) -> Platform`
*Line 62 · fn*

Get the current platform from the adapter cache.

---

