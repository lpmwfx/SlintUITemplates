# `src/platform/mod.rs`

## `pub fn apply_backdrop(window: &slint::Window, style: BgStyle)`
*Line 17 · fn*

Apply OS-level backdrop to a Slint window.
No-op when `style == BgStyle::Solid` or on non-Windows platforms.

---

