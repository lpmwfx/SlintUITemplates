# `gateway/view_config.rs`

## `pub fn eval_file(path: &Path) -> Result<ViewConfig, ViewConfigError>`
*Line 6 · fn*

Load and evaluate a single `.rhai` file via the gateway.

---

## `pub fn load_all(dir: &Path) -> Result<HashMap<String, ViewConfig>, ViewConfigError>`
*Line 13 · fn*

Scan `dir/*.rhai` and return a map of `{stem → ViewConfig}`.
Missing or empty directory is not an error — returns an empty map.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
