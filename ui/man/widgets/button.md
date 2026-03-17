# `widgets/button.slint`

## `export component Button inherits Rectangle`
*Line 6 · component*

Generic action button with Fluent-style variants and hover feedback.

---

## `in property <string> label: "Button";`
*Line 8 · property*

Text rendered in the button body.

---

## `in property <string> variant: Variants.primary;   // primary | secondary | ghost | danger`
*Line 10 · property*

Visual style key: `primary`, `secondary`, `ghost`, or `danger`.

---

## `in property <bool> enabled: true;`
*Line 12 · property*

Enables pointer interaction and active styling when `true`.

---

## `callback clicked();`
*Line 14 · callback*

Fired when the user activates the button.

---

