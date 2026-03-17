# `shell/platform.rs`

## `pub enum Platform`
*Line 4 · enum*

Target platform for AppShell rendering and token routing.
P la tf or m enum.

---

## `pub fn as_str(&self) -> &'static str`
*Line 18 · fn*

Returns the platform identifier string that matches `Variants.*` in Slint.

---

## `pub fn is_mobile(&self) -> bool`
*Line 29 · fn*

Returns `true` if the platform is a mobile target (portrait touch-first).

---

## `pub fn is_small(&self) -> bool`
*Line 35 · fn*

Returns `true` if the platform is the small tier (1280×800 touchscreen + gamepad).
Small tier: gamepad-safe 56px touch targets, bottom/rail nav, max 5 nav items.

---

## `pub fn is_desktop(&self) -> bool`
*Line 41 · fn*

Returns `true` if the platform is a desktop target (mouse+keyboard or gamepad fullscreen).
Desktop tier includes SteamLinux — fullscreen but with desktop nav limits.

---

