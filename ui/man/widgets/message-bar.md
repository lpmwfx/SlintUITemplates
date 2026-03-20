# `widgets/message-bar.slint`

## `export component MessageBar inherits Rectangle`
*Line 5 · component*

M es sa ge ba r component.

---

## `in property <string> message: "";`
*Line 7 · property*

Input property "message".

---

## `in property <string> variant: "info";   // info | warning | error | success`
*Line 9 · property*

Input property "variant".

---

## `in property <string> action-label: "";`
*Line 11 · property*

Input property "action-label".

---

## `in property <bool>   closable: true;`
*Line 13 · property*

Input property "closable".

---

## `callback action();`
*Line 15 · callback*

Callback fired for a ct io n.

---

## `callback closed();`
*Line 17 · callback*

Callback fired for c lo se d.

---

## `private property <string> v-error:   "error";`
*Line 20 · property*

Private property "v-error" used internally.

---

## `private property <string> v-warning: "warning";`
*Line 22 · property*

Private property "v-warning" used internally.

---

## `private property <string> v-success: "success";`
*Line 24 · property*

Private property "v-success" used internally.

---

## `private property <length> border-px: 1px;`
*Line 26 · property*

Private property "border-px" used internally.

---

## `private property <length> bar-w:     3px;`
*Line 28 · property*

Private property "bar-w" used internally.

---

## `private property <int>    btn-shrink: 0;`
*Line 30 · property*

Private property "btn-shrink" used internally.

---

## `private property <int>    text-grow:  1;`
*Line 32 · property*

Private property "text-grow" used internally.

---

## `private property <color> accent-color:`
*Line 34 · property*

Private property "accent-color" used internally.

---

## `private property <string> icon-glyph:`
*Line 40 · property*

Private property "icon-glyph" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
