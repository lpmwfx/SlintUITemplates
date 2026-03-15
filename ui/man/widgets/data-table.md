# `widgets/data-table.slint`

## `export component DataTable inherits Rectangle`
*Line 10 · component*

D at at ab le component.

---

## `in property <[TableColumn]> columns: [];`
*Line 12 · property*

Input property "columns".

---

## `in property <[TableRow]>    rows: [];`
*Line 14 · property*

Input property "rows".

---

## `in property <string>    sort-key: "";`
*Line 16 · property*

Input property "sort-key".

---

## `in property <bool>      sort-asc: true;`
*Line 18 · property*

Input property "sort-asc".

---

## `in property <string>    selected: "";`
*Line 20 · property*

Input property "selected".

---

## `in property <string> sort-asc-indicator: "\u{25B2}";`
*Line 22 · property*

Input property "sort-asc-indicator".

---

## `in property <string> sort-desc-indicator: "\u{25BC}";`
*Line 24 · property*

Input property "sort-desc-indicator".

---

## `callback sort-changed(string, bool);`
*Line 26 · callback*

Callback fired for s or t c ha ng ed.

---

## `callback row-selected(string);`
*Line 28 · callback*

Callback fired for r ow s el ec te d.

---

