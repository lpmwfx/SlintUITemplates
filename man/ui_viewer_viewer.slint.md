# ui/viewer/viewer.slint

## `export component FrameworkViewer inherits Window {`

*Line 15 · component*

**undocumented**

---

## `in property <string> active-view: ViewId.buttons;`

*Line 23 · property*

**undocumented**

---

## `in property <image> canvas-frame;`

*Line 26 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 30 · callback*

**undocumented**

---

## `private property <length> win-w:     1100px;`

*Line 34 · property*

**undocumented**

---

## `private property <length> win-h:     720px;`

*Line 37 · property*

**undocumented**

---

## `private property <float>  zoom-small:   Sizes.three-quarter;`

*Line 40 · property*

**undocumented**

---

## `private property <float>  zoom-normal:  Sizes.fill;`

*Line 43 · property*

**undocumented**

---

## `private property <float>  zoom-large:  1.25;`

*Line 46 · property*

**undocumented**

---

## `private property <float>  zoom-xlarge:  1.5;`

*Line 49 · property*

**undocumented**

---

## `private property <string> menu-zoom-sm: "Zoom 75%";`

*Line 52 · property*

**undocumented**

---

## `private property <string> menu-zoom-md: "Zoom 100%";`

*Line 55 · property*

**undocumented**

---

## `private property <string> menu-zoom-lg: "Zoom 125%";`

*Line 58 · property*

**undocumented**

---

## `private property <string> menu-zoom-xl: "Zoom 150%";`

*Line 61 · property*

**undocumented**

---

## `private property <int>    v-stretch: Sizes.one;`

*Line 64 · property*

**undocumented**

---

## `private property <int>    h-sidebar: Sizes.zero;`

*Line 67 · property*

**undocumented**

---

## `private property <int>    h-stretch: Sizes.one;`

*Line 70 · property*

**undocumented**

---

## `callback request-bg-style(string);`

*Line 76 · callback*

**undocumented**

---

## `callback new-file();`

*Line 79 · callback*

**undocumented**

---

## `callback new-window();`

*Line 82 · callback*

**undocumented**

---

## `callback open-file();`

*Line 85 · callback*

**undocumented**

---

## `callback save();`

*Line 88 · callback*

**undocumented**

---

## `callback save-as();`

*Line 91 · callback*

**undocumented**

---

## `callback quit();`

*Line 94 · callback*

**undocumented**

---

## `callback undo();`

*Line 97 · callback*

**undocumented**

---

## `callback redo();`

*Line 100 · callback*

**undocumented**

---

## `callback cut();`

*Line 103 · callback*

**undocumented**

---

## `callback copy();`

*Line 106 · callback*

**undocumented**

---

## `callback paste();`

*Line 109 · callback*

**undocumented**

---

## `callback find();`

*Line 112 · callback*

**undocumented**

---

## `callback select-all();`

*Line 115 · callback*

**undocumented**

---

## `callback open-docs();`

*Line 118 · callback*

**undocumented**

---

## `callback open-shortcuts();`

*Line 121 · callback*

**undocumented**

---

## `callback about();`

*Line 124 · callback*

**undocumented**

---

