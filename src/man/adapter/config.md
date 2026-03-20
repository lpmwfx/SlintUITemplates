# `adapter/config.rs`

## `pub fn load_view_configs(&self, dir: &Path) -> Result<(), crate::view_config::ViewConfigError>`
*Line 7 · fn*

Load all `*.rhai` files from `dir` and register as per-view configs.
Auto-applied by the navigate handler on every nav event.

---

## `pub fn set_window_size(&self, width: u32, height: u32)`
*Line 14 · fn*

Resize the window (logical pixels).

---

## `pub fn set_bg_style_str(&mut self, style: &str)`
*Line 19 · fn*

Set the OS-level backdrop style ("mica" | "acrylic" | anything else → solid).

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
