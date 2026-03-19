# ui/widgets/command-bar-flyout.slint

## `export component CommandBarFlyout inherits Rectangle {`

*Line 16 · component*

**undocumented**

---

## `in property <[CbfAction]> primary: [];`

*Line 25 · property*

**undocumented**

---

## `in property <[CbfAction]> secondary: [];`

*Line 34 · property*

**undocumented**

---

## `in property <length>      window-height;`

*Line 43 · property*

**undocumented**

---

## `callback action(string);`

*Line 52 · callback*

**undocumented**

---

## `private property <length> pop-x:        Sizes.no-size;`

*Line 62 · property*

**undocumented**

---

## `private property <length> pop-y:        Sizes.no-size;`

*Line 71 · property*

**undocumented**

---

## `private property <length> border-px:    Sizes.border-w;`

*Line 80 · property*

**undocumented**

---

## `private property <bool>   has-secondary: root.secondary.length > 0;`

*Line 89 · property*

**undocumented**

---

## `private property <length> sec-add:      root.has-secondary ? Spacing.xl : Sizes.no-size;`

*Line 98 · property*

**undocumented**

---

## `private property <length> primary-w:    root.primary.length * Spacing.xl + root.sec-add + Spacing.sm * 2;`

*Line 107 · property*

**undocumented**

---

## `private property <length> primary-h:    Spacing.xl + Spacing.sm * 2;`

*Line 116 · property*

**undocumented**

---

## `private property <length> secondary-w:  Sizes.panel-sm;`

*Line 125 · property*

**undocumented**

---

## `private property <length> secondary-h:  root.secondary.length * Spacing.xl + Spacing.sm * 2;`

*Line 134 · property*

**undocumented**

---

