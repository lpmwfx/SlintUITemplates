# `grid/mod.rs`

## `pub mod config;`
*Line 2 · mod*

Grid configuration loading and deserialization from TOML target files.

---

## `pub mod zone;`
*Line 4 · mod*

Zone model representing the runtime layout grid with rows and columns.

---

## `pub fn load_target(path: &Path) -> Result<ZoneModel, Box<dyn std::error::Error>>`
*Line 12 · fn*

Loads a target TOML file and converts its grid config into a `ZoneModel`.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
