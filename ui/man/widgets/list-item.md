# `widgets/list-item.slint`

## `export component ListItem inherits Rectangle`
*Line 5 · component*

Compact two-line list row with optional subtitle and selected state.

---

## `in property <string> title: "";`
*Line 7 · property*

Primary text shown for the row.

---

## `in property <string> subtitle: "";`
*Line 9 · property*

Secondary text shown below the title when non-empty.

---

## `in property <bool> selected: false;`
*Line 11 · property*

Highlights the row as the currently selected item.

---

## `callback clicked();`
*Line 13 · callback*

Fired when the row is clicked.

---

## `private property <length> inner-gap: Sizes.border-w-2;`
*Line 16 · property*

Private property "inner-gap" used internally.

---

