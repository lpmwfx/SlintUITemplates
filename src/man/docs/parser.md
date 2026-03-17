# `docs/parser.rs`

## `pub fn parse(input: &str) -> Vec<DocBlock>`
*Line 19 · fn*

Parse a CommonMark string into a flat list of [`DocBlock`] items.

Supported block types:
- `"h1"` / `"h2"` / `"h3"` — headings
- `"p"` — paragraph
- `"code"` — fenced code block
- `"li"` / `"li2"` — list item (level 0 / level 1+)
- `"th"` / `"tr"` — table header / data row
- `"hr"` — horizontal rule
- `"bq"` — blockquote
P ar se.

---

