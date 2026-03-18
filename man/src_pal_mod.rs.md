# src/pal/mod.rs

## `pub(crate) fn is_dark_mode() -> bool { false }`

*Line 23 · fn*

Fallback dark-mode check for non-Windows platforms (always returns false).

---

## `pub fn apply_backdrop(window: &slint::Window, style: BgStyle)`

*Line 27 · fn*

Apply OS-level backdrop to a Slint window.
No-op when `style == BgStyle::Solid` or on non-Windows platforms.

---

