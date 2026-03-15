# `src/view_config/mod.rs`

## `pub struct ViewConfig`
*Line 34 · struct*

Per-view shell chrome configuration.
`None` / empty = "don't override the DSL value".

---

## `pub fn eval_script(script: &str) -> Result<ViewConfig, Box<dyn Error>>`
*Line 44 · fn*

Evaluate a Rhai script string and return a `ViewConfig`.

---

## `pub fn eval_file(path: &Path) -> Result<ViewConfig, Box<dyn Error>>`
*Line 79 · fn*

Load and evaluate a single `.rhai` file.

---

## `pub fn load_all(dir: &Path) -> Result<HashMap<String, ViewConfig>, Box<dyn Error>>`
*Line 86 · fn*

Scan `dir/*.rhai` and return a map of `{stem → ViewConfig}`.
Missing or empty directory is not an error — returns an empty map.

---

## `pub fn apply(ui: &AppWindow, cfg: &ViewConfig)`
*Line 112 · fn*

Apply non-None / non-empty `ViewConfig` fields to the `AppWindow`.
Fields that are `None` or empty are left unchanged.

---

