# `widgets/pagination.slint`

## `export component Pagination inherits Rectangle`
*Line 5 · component*

Pagination control with previous/next buttons and optional explicit page chips.

---

## `in property <int>   current: Sizes.one;`
*Line 7 · property*

Currently selected page number.

---

## `in property <int>   total: Sizes.one;`
*Line 9 · property*

Highest available page number.

---

## `in property <[int]> pages: [];`
*Line 11 · property*

Explicit page numbers to render as clickable chips. Empty falls back to `current / total`.

---

## `callback changed(int);`
*Line 13 · callback*

Fired when the user requests a different page.

---

## `private property <string> nav-prev: "‹";`
*Line 17 · property*

Private property "nav-prev" used internally.

---

## `private property <string> nav-next: "›";`
*Line 19 · property*

Private property "nav-next" used internally.

---

## `private property <string> page-sep: " / ";`
*Line 21 · property*

Private property "page-sep" used internally.

---

## `private property <float> disabled-alpha: 0.4;`
*Line 25 · property*

Private property "disabled-alpha" used internally.

---

