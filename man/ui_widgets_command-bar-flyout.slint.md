# ui/widgets/command-bar-flyout.slint

## `export component CommandBarFlyout inherits Rectangle {`

*Line 9 · component*

**undocumented**

---

## `in property <[CbfAction]> primary: [];`

*Line 12 · property*

**undocumented**

---

## `in property <[CbfAction]> secondary: [];`

*Line 15 · property*

**undocumented**

---

## `in property <length>      window-height;`

*Line 18 · property*

**undocumented**

---

## `callback action(string);`

*Line 21 · callback*

**undocumented**

---

## `private property <length> pop-x:        0px;`

*Line 25 · property*

**undocumented**

---

## `private property <length> pop-y:        0px;`

*Line 28 · property*

**undocumented**

---

## `private property <length> border-px:    1px;`

*Line 31 · property*

**undocumented**

---

## `private property <bool>   has-secondary: root.secondary.length > 0;`

*Line 34 · property*

**undocumented**

---

## `private property <length> sec-add:      root.has-secondary ? Spacing.xl : 0px;`

*Line 37 · property*

**undocumented**

---

## `private property <length> primary-w:    root.primary.length * Spacing.xl + root.sec-add + Spacing.sm * 2;`

*Line 40 · property*

**undocumented**

---

## `private property <length> primary-h:    Spacing.xl + Spacing.sm * 2;`

*Line 43 · property*

**undocumented**

---

## `private property <length> secondary-w:  200px;`

*Line 46 · property*

**undocumented**

---

## `private property <length> secondary-h:  root.secondary.length * Spacing.xl + Spacing.sm * 2;`

*Line 49 · property*

**undocumented**

---

