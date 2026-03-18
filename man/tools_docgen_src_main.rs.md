# tools/docgen/src/main.rs

## `pub(crate) enum PropMode { In, InOut, Out }`

*Line 11 · enum*

Slint property access qualifier.

---

## `pub(crate) struct Prop { pub(crate) mode: PropMode, pub(crate) ty: String, pub(crate) name: String, pub(crate) default: String, pub(crate) desc: String }`

*Line 26 · struct*

Parsed Slint property declaration.

---

## `pub(crate) struct Cb   { pub(crate) name: String, pub(crate) args: String, pub(crate) desc: String }`

*Line 28 · struct*

Parsed Slint callback declaration.

---

## `pub(crate) struct Field { pub(crate) name: String, pub(crate) ty: String }`

*Line 30 · struct*

Parsed Slint struct field.

---

## `pub(crate) enum Item`

*Line 33 · enum*

Parsed Slint component or struct item.

---

## `pub(crate) const COMMENT_PREFIX_LEN: usize = "//".len();`

*Line 39 · const*

Length of "//" comment prefix.

---

## `pub(crate) const SLINT_EXT: &str = "slint";`

*Line 41 · const*

File extension for Slint source files.

---

