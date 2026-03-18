# tools/docgen/src/parser.rs

## `pub(crate) fn split_comment(s: &str) -> (&str, &str)`

*Line 4 · fn*

Split a line into code and trailing `//` comment.

---

## `pub(crate) fn parse_prop(line: &str) -> Option<Prop>`

*Line 10 · fn*

Parse a Slint property declaration line.

---

## `pub(crate) fn parse_cb(line: &str) -> Option<Cb>`

*Line 26 · fn*

Parse a Slint callback declaration line.

---

## `pub(crate) fn parse_file(src: &str, filename: &str) -> Vec<Item>`

*Line 42 · fn*

Parse all exported components and structs from a Slint source file.

---

