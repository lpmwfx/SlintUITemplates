# tools/docgen/src/renderer.rs

## `pub(crate) fn render(all: &[Item]) -> String`

*Line 31 · fn*

Render parsed items to a Markdown string.

---

## `pub(crate) fn update(existing: &str, generated: &str) -> String`

*Line 55 · fn*

Splice generated content between AUTOGEN markers in existing text.

---

## `pub(crate) fn scan(shared: &Path) -> Result<Vec<Item>, Err>`

*Line 64 · fn*

Scan a directory for `.slint` files and parse all items.

---

## `pub(crate) fn generate(all: &[Item], docs: &Path) -> Result<(usize, usize), Err>`

*Line 79 · fn*

Write rendered Markdown to `docs` path. Returns (component_count, struct_count).

---

