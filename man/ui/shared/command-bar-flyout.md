# `ui/shared/command-bar-flyout.slint`

## `export component CommandBarFlyout inherits Rectangle`
*Line 8 · component*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <[CbfAction]> primary: [];`
*Line 9 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <[CbfAction]> secondary: [];`
*Line 10 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `in property <length>      window-height;`
*Line 11 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `callback action(string);`
*Line 12 · callback*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> pop-x:        0px;`
*Line 14 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> pop-y:        0px;`
*Line 15 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> border-px:    1px;`
*Line 16 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <bool>   has-secondary: root.secondary.length > 0;`
*Line 17 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> sec-add:      root.has-secondary ? Spacing.xl : 0px;`
*Line 18 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> primary-w:    root.primary.length * Spacing.xl + root.sec-add + Spacing.sm * 2;`
*Line 19 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> primary-h:    Spacing.xl + Spacing.sm * 2;`
*Line 20 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> secondary-w:  200px;`
*Line 21 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

## `private property <length> secondary-h:  root.secondary.length * Spacing.xl + Spacing.sm * 2;`
*Line 22 · property*

> ⚠ **undocumented** — add a `///` doc comment

---

