# ui/viewer/viewer.slint

## `export component FrameworkViewer inherits Window {`

*Line 21 · component*

**undocumented**

---

## `in property <string> active-view: ViewId.buttons;`

*Line 35 · property*

**undocumented**

---

## `in property <image> canvas-frame;`

*Line 44 · property*

**undocumented**

---

## `callback navigate(string);`

*Line 54 · callback*

**undocumented**

---

## `private property <length> win-w:     Sizes.win-width-lg;`

*Line 64 · property*

**undocumented**

---

## `private property <length> win-h:     Sizes.win-height-lg;`

*Line 73 · property*

**undocumented**

---

## `private property <float>  zoom-small:   Sizes.three-quarter;`

*Line 82 · property*

**undocumented**

---

## `private property <float>  zoom-normal:  Sizes.fill;`

*Line 91 · property*

**undocumented**

---

## `private property <float>  zoom-large:  1.25;`

*Line 100 · property*

**undocumented**

---

## `private property <float>  zoom-xlarge:  1.5;`

*Line 109 · property*

**undocumented**

---

## `private property <string> menu-zoom-sm: "Zoom 75%";`

*Line 118 · property*

**undocumented**

---

## `private property <string> menu-zoom-md: "Zoom 100%";`

*Line 127 · property*

**undocumented**

---

## `private property <string> menu-zoom-lg: "Zoom 125%";`

*Line 136 · property*

**undocumented**

---

## `private property <string> menu-zoom-xl: "Zoom 150%";`

*Line 145 · property*

**undocumented**

---

## `private property <int>    v-stretch: Sizes.one;`

*Line 154 · property*

**undocumented**

---

## `private property <int>    h-sidebar: Sizes.zero;`

*Line 163 · property*

**undocumented**

---

## `private property <int>    h-stretch: Sizes.one;`

*Line 172 · property*

**undocumented**

---

## `callback request-bg-style(string);`

*Line 184 · callback*

**undocumented**

---

## `callback new-file();`

*Line 193 · callback*

**undocumented**

---

## `callback new-window();`

*Line 202 · callback*

**undocumented**

---

## `callback open-file();`

*Line 211 · callback*

**undocumented**

---

## `callback save();`

*Line 220 · callback*

**undocumented**

---

## `callback save-as();`

*Line 229 · callback*

**undocumented**

---

## `callback quit();`

*Line 238 · callback*

**undocumented**

---

## `callback undo();`

*Line 247 · callback*

**undocumented**

---

## `callback redo();`

*Line 256 · callback*

**undocumented**

---

## `callback cut();`

*Line 265 · callback*

**undocumented**

---

## `callback copy();`

*Line 274 · callback*

**undocumented**

---

## `callback paste();`

*Line 283 · callback*

**undocumented**

---

## `callback find();`

*Line 292 · callback*

**undocumented**

---

## `callback select-all();`

*Line 301 · callback*

**undocumented**

---

## `callback open-docs();`

*Line 310 · callback*

**undocumented**

---

## `callback open-shortcuts();`

*Line 319 · callback*

**undocumented**

---

## `callback about();`

*Line 328 · callback*

**undocumented**

---

