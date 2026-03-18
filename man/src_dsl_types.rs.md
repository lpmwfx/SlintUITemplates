# src/dsl/types.rs

## `pub enum BgStyle`

*Line 8 · enum*

Windows Composition API backdrop style.
Mica and Acrylic require Windows 11 (22H2+) — silently falls back to Solid
on older OS versions or non-Windows platforms.
B gs ty le enum.

---

## `pub fn from_str(s: &str) -> Self`

*Line 20 · fn*

Parse from string — unknown values default to `Solid`.

---

## `pub struct Nav`

*Line 34 · struct*

A navigation destination — icon name resolved to codepoint at build().
N av struct.

---

## `pub fn new( id:    impl Into<String>, label: impl Into<String>, icon:  impl Into<String>, ) -> Self`

*Line 45 · fn*

Create a navigation item with the given id, display label, and icon name.

---

## `pub struct Toolbar`

*Line 57 · struct*

A toolbar icon button — icon name resolved to codepoint at build().
T oo lb ar struct.

---

## `pub fn new( id:      impl Into<String>, icon:    impl Into<String>, tooltip: impl Into<String>, ) -> Self`

*Line 68 · fn*

Create a toolbar button with the given id, icon name, and tooltip text.

---

