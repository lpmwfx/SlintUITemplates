# `shell/desktop/status-bar.slint`

## `export component ShellStatusBar inherits Rectangle`
*Line 7 · component*

Bottom status bar for the desktop shell with optional progress indicator.

---

## `in property <string>  status-text: "Ready";`
*Line 9 · property*

Primary status message shown on the left.

---

## `in property <float>   progress: -1.0;   // -1 = hidden, 0.0..1.0 = shown`
*Line 11 · property*

Progress value in the range `0.0..=1.0`; negative hides the indicator.

---

## `in property <string>  right-text: "";`
*Line 13 · property*

Optional secondary message aligned to the right.

---

## `private property <length> bar-radius: Sizes.border-w-2;`
*Line 16 · property*

Private property "bar-radius" used internally.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
