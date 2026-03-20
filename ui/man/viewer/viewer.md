# `viewer/viewer.slint`

## `export component FrameworkViewer inherits Window`
*Line 14 · component*

F ra me wo rk vi ew er component.

---

## `in-out property <string> active-view: ViewId.buttons;`
*Line 21 · property*

Two-way property "active-view".

---

## `in property <image> canvas-frame;`
*Line 23 · property*

Input property "canvas-frame".

---

## `private property <length> win-w:     1100px;`
*Line 26 · property*

Private property "win-w" used internally.

---

## `private property <length> win-h:     720px;`
*Line 28 · property*

Private property "win-h" used internally.

---

## `private property <float>  zoom-small:   Sizes.three-quarter;`
*Line 30 · property*

Private property "zoom-small" used internally.

---

## `private property <float>  zoom-normal:  Sizes.fill;`
*Line 32 · property*

Private property "zoom-normal" used internally.

---

## `private property <float>  zoom-large:  1.25;`
*Line 34 · property*

Private property "zoom-large" used internally.

---

## `private property <float>  zoom-xlarge:  1.5;`
*Line 36 · property*

Private property "zoom-xlarge" used internally.

---

## `private property <string> menu-zoom-sm: "Zoom 75%";`
*Line 38 · property*

Private property "menu-zoom-sm" used internally.

---

## `private property <string> menu-zoom-md: "Zoom 100%";`
*Line 40 · property*

Private property "menu-zoom-md" used internally.

---

## `private property <string> menu-zoom-lg: "Zoom 125%";`
*Line 42 · property*

Private property "menu-zoom-lg" used internally.

---

## `private property <string> menu-zoom-xl: "Zoom 150%";`
*Line 44 · property*

Private property "menu-zoom-xl" used internally.

---

## `private property <int>    v-stretch: Sizes.one;`
*Line 46 · property*

Private property "v-stretch" used internally.

---

## `private property <int>    h-sidebar: Sizes.zero;`
*Line 48 · property*

Private property "h-sidebar" used internally.

---

## `private property <int>    h-stretch: Sizes.one;`
*Line 50 · property*

Private property "h-stretch" used internally.

---

## `callback request-bg-style(string);`
*Line 55 · callback*

Callback fired for r eq ue st b g s ty le.

---

## `callback new-file();`
*Line 57 · callback*

Callback fired for n ew f il e.

---

## `callback new-window();`
*Line 59 · callback*

Callback fired for n ew w in do w.

---

## `callback open-file();`
*Line 61 · callback*

Callback fired for o pe n f il e.

---

## `callback save();`
*Line 63 · callback*

Callback fired for s av e.

---

## `callback save-as();`
*Line 65 · callback*

Callback fired for s av e a s.

---

## `callback quit();`
*Line 67 · callback*

Callback fired for q ui t.

---

## `callback undo();`
*Line 69 · callback*

Callback fired for u nd o.

---

## `callback redo();`
*Line 71 · callback*

Callback fired for r ed o.

---

## `callback cut();`
*Line 73 · callback*

Callback fired for c ut.

---

## `callback copy();`
*Line 75 · callback*

Callback fired for c op y.

---

## `callback paste();`
*Line 77 · callback*

Callback fired for p as te.

---

## `callback find();`
*Line 79 · callback*

Callback fired for f in d.

---

## `callback select-all();`
*Line 81 · callback*

Callback fired for s el ec t a ll.

---

## `callback open-docs();`
*Line 83 · callback*

Callback fired for o pe n d oc s.

---

## `callback open-shortcuts();`
*Line 85 · callback*

Callback fired for o pe n s ho rt cu ts.

---

## `callback about();`
*Line 87 · callback*

Callback fired for a bo ut.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
