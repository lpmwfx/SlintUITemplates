# ui/shell/mobile/fab.slint

## `export component FAB inherits Rectangle {`

*Line 9 · component*

**undocumented**

---

## `in property <string> icon;`

*Line 12 · property*

**undocumented**

---

## `in property <string> label: "";       // non-empty → extended FAB`

*Line 15 · property*

**undocumented**

---

## `in property <string> icon-font: "Segoe Fluent Icons";`

*Line 18 · property*

**undocumented**

---

## `callback clicked();`

*Line 21 · callback*

**undocumented**

---

## `private property <bool> extended: root.label != "";`

*Line 25 · property*

**undocumented**

---

## `private property <float>  darken-amount: Sizes.ratio-xs;`

*Line 28 · property*

**undocumented**

---

## `private property <float>  shadow-alpha: Sizes.alpha-30;`

*Line 31 · property*

**undocumented**

---

## `private property <length> shadow-blur: Sizes.comp-6;`

*Line 34 · property*

**undocumented**

---

## `private property <length> shadow-y: Sizes.border-w-3;`

*Line 37 · property*

**undocumented**

---

