# src/view_config/eval.rs

## `pub fn eval_script(script: &str) -> Result<ViewConfig, ViewConfigError>`

*Line 33 · fn*

Evaluate a Rhai script string and return a `ViewConfig`.

---

## `pub fn eval_file(path: &Path) -> Result<ViewConfig, ViewConfigError>`

*Line 63 · fn*

Load and evaluate a single `.rhai` file (delegated to gateway).

---

## `pub fn load_all(dir: &Path) -> Result<HashMap<String, ViewConfig>, ViewConfigError>`

*Line 68 · fn*

Scan `dir/*.rhai` and return a map of `{stem → ViewConfig}` (delegated to gateway).

---

