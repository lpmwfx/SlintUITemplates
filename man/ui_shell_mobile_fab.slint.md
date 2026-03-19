# ui/shell/mobile/fab.slint

## `export component FAB inherits Rectangle {`

*Line 15 · component*

**undocumented**

---

## `in property <string> icon;`

*Line 24 · property*

**undocumented**

---

## `in property <string> label: "";       // non-empty → extended FAB`

*Line 33 · property*

**undocumented**

---

## `in property <string> icon-font: "Segoe Fluent Icons";`

*Line 42 · property*

**undocumented**

---

## `callback clicked();`

*Line 51 · callback*

**undocumented**

---

## `private property <bool> extended: root.label != "";`

*Line 61 · property*

**undocumented**

---

## `private property <float>  darken-amount: Sizes.ratio-xs;`

*Line 70 · property*

**undocumented**

---

## `private property <float>  shadow-alpha: Sizes.alpha-30;`

*Line 79 · property*

**undocumented**

---

## `private property <length> shadow-blur: Sizes.comp-6;`

*Line 88 · property*

**undocumented**

---

## `private property <length> shadow-y: Sizes.border-w-3;`

*Line 97 · property*

**undocumented**

---

