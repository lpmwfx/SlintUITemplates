# `state/Theme.slint`

## `in-out property <string> material: "solid";  // "solid" | "mica" | "acrylic"`
*Line 25 · property*

Two-way property "material".

---

## `in-out property <bool>   dark: Palette.color-scheme == ColorScheme.dark;`
*Line 27 · property*

Two-way property "dark".

---

## `in-out property <bool>   dark-mode <=> dark;  // backwards-compat alias`
*Line 29 · property*

Two-way property "dark-mode".

---

## `in-out property <color>  accent-override: transparent;  // alpha=0 → derive from palette`
*Line 31 · property*

Two-way property "accent-override".

---

## `out property <color> bg-primary:`
*Line 34 · property*

Output property "bg-primary".

---

## `out property <color> bg-surface:`
*Line 40 · property*

Output property "bg-surface".

---

## `out property <color> bg-elevated:`
*Line 46 · property*

Output property "bg-elevated".

---

## `out property <color> text-primary:`
*Line 52 · property*

Output property "text-primary".

---

## `out property <color> text-secondary:`
*Line 58 · property*

Output property "text-secondary".

---

## `out property <color> text-disabled:`
*Line 64 · property*

Output property "text-disabled".

---

## `out property <color> accent:`
*Line 70 · property*

Output property "accent".

---

## `out property <color> accent-hover:`
*Line 77 · property*

Output property "accent-hover".

---

## `out property <color> accent-text:`
*Line 83 · property*

Output property "accent-text".

---

## `out property <color> error:   #C42B1C;`
*Line 89 · property*

Output property "error".

---

## `out property <color> success: #0F7B0F;`
*Line 91 · property*

Output property "success".

---

## `out property <color> warning: #9D5D00;`
*Line 93 · property*

Output property "warning".

---

## `out property <color> border:`
*Line 96 · property*

Output property "border".

---

## `out property <color> border-strong:`
*Line 102 · property*

Output property "border-strong".

---

## `out property <color> border-focus: accent;`
*Line 108 · property*

Output property "border-focus".

---

## `out property <color> shadow:`
*Line 111 · property*

Output property "shadow".

---

## `out property <color> overlay: #00000040;`
*Line 118 · property*

Output property "overlay".

---

## `out property <color> bg-panel:     bg-surface;`
*Line 122 · property*

Output property "bg-panel".

---

## `out property <color> handle:       border;`
*Line 124 · property*

Output property "handle".

---

## `out property <color> handle-hover: border-strong;`
*Line 126 · property*

Output property "handle-hover".

---

## `out property <color> danger:       error;`
*Line 128 · property*

Output property "danger".

---

## `out property <color> text-muted:   text-secondary;`
*Line 130 · property*

Output property "text-muted".

---

## `in-out property <string> platform: Variants.windows;`
*Line 137 · property*

Two-way property "platform".

---

## `out property <length> radius-sm:`
*Line 149 · property*

Small border radius — checkboxes, chips.

---

## `out property <length> radius-md:`
*Line 152 · property*

Standard border radius — buttons, inputs.

---

## `out property <length> radius-lg:`
*Line 155 · property*

Large border radius — cards, panels.

---

## `out property <length> radius-xl:`
*Line 158 · property*

Extra-large border radius — dialogs, large surfaces.

---

## `out property <length> radius-circle:`
*Line 161 · property*

Full pill / circle radius.

---

## `out property <length> touch-sm:`
*Line 166 · property*

Small control height — compact items.

---

## `out property <length> touch-md:`
*Line 169 · property*

Standard control height — buttons, inputs.

---

## `out property <length> touch-lg:`
*Line 172 · property*

Large control height — primary targets.

---

## `out property <length> font-caption:`
*Line 177 · property*

Caption / label text size.

---

## `out property <length> font-body:`
*Line 180 · property*

Body text size.

---

## `out property <length> font-subtitle:`
*Line 183 · property*

Subtitle / section heading.

---

## `out property <length> font-heading:`
*Line 186 · property*

Heading.

---

## `out property <length> font-title:`
*Line 189 · property*

Title / display.

---

## `out property <int> font-weight-regular:`
*Line 194 · property*

Regular weight (400).

---

## `out property <int> font-weight-medium:`
*Line 197 · property*

Medium weight (500).

---

## `out property <int> font-weight-bold:`
*Line 200 · property*

Bold weight (700).

---

## `out property <length> space-xs:`
*Line 205 · property*

Extra-small spacing.

---

## `out property <length> space-sm:`
*Line 208 · property*

Small spacing.

---

## `out property <length> space-md:`
*Line 211 · property*

Medium spacing.

---

## `out property <length> space-lg:`
*Line 214 · property*

Large spacing.

---

## `out property <length> space-xl:`
*Line 217 · property*

Extra-large spacing.

---

## `out property <length> nav-rail-width:`
*Line 222 · property*

Side navigation rail width.

---

## `out property <length> nav-bar-height:`
*Line 225 · property*

Navigation bar height (bottom nav / toolbar).

---

## `out property <length> top-bar-height:`
*Line 228 · property*

Top bar / title bar height.

---

## `out property <length> touch-target:`
*Line 234 · property*

Minimum safe interactive target for this platform.
Use for min-width / min-height on buttons and nav items.

---

## `out property <string> nav-style:`
*Line 237 · property*

Recommended nav chrome pattern for this platform.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
