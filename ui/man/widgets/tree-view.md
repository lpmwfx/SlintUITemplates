# `widgets/tree-view.slint`

## `export struct TreeNode`
*Line 6 · struct*

T re en od e struct.

---

## `export component TreeView inherits Rectangle`
*Line 16 · component*

T re ev ie w component.

---

## `in property <[TreeNode]> nodes: [];`
*Line 18 · property*

Input property "nodes".

---

## `in property <string> selected: "";`
*Line 20 · property*

Input property "selected".

---

## `callback selected-changed(string);`
*Line 22 · callback*

Callback fired for s el ec te d c ha ng ed.

---

## `callback toggle-expanded(string);`
*Line 24 · callback*

Callback fired for t og gl e e xp an de d.

---

## `private property <length> bar-radius: Sizes.border-w-2;`
*Line 27 · property*

Private property "bar-radius" used internally.

---

## `private property <float>  bar-opacity: Sizes.percent-60;`
*Line 29 · property*

Private property "bar-opacity" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
