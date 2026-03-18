# ui/widgets/spinner.slint

## `export component Spinner inherits Rectangle {`

*Line 8 · component*

**undocumented**

---

## `in property <string> size:          Variants.md;`

*Line 11 · property*

**undocumented**

---

## `in property <color>  spinner-color: Colors.accent;`

*Line 14 · property*

**undocumented**

---

## `in property <bool>   active:        true;`

*Line 17 · property*

**undocumented**

---

## `private property <length> dim:`

*Line 21 · property*

**undocumented**

---

## `private property <float>  anim-phase: 0.0;`

*Line 28 · property*

**undocumented**

---

## `private property <float>  anim-speed: 0.05;`

*Line 31 · property*

**undocumented**

---

## `private property <float>  anim-next: Math.mod(root.anim-phase + root.anim-speed, Sizes.two);`

*Line 34 · property*

**undocumented**

---

## `private property <float>  pulse-min:  0.2;`

*Line 37 · property*

**undocumented**

---

## `private property <float>  track-alpha: 0.15;`

*Line 40 · property*

**undocumented**

---

## `private property <length> ring-width: Sizes.border-w-2;`

*Line 43 · property*

**undocumented**

---

## `private property <float>  pulse-val: Math.abs(root.anim-phase - Sizes.fill) * (Sizes.fill - root.pulse-min) + root.pulse-min;`

*Line 47 · property*

**undocumented**

---

