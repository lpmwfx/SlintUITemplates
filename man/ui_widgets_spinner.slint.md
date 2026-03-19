# ui/widgets/spinner.slint

## `export component Spinner inherits Rectangle {`

*Line 14 · component*

**undocumented**

---

## `in property <string> size:          Variants.md;`

*Line 23 · property*

**undocumented**

---

## `in property <color>  spinner-color: Colors.accent;`

*Line 32 · property*

**undocumented**

---

## `in property <bool>   active:        true;`

*Line 41 · property*

**undocumented**

---

## `private property <length> dim:`

*Line 51 · property*

**undocumented**

---

## `private property <float>  anim-phase: 0.0;`

*Line 64 · property*

**undocumented**

---

## `private property <float>  anim-speed: 0.05;`

*Line 73 · property*

**undocumented**

---

## `private property <float>  anim-next: Math.mod(root.anim-phase + root.anim-speed, Sizes.two);`

*Line 82 · property*

**undocumented**

---

## `private property <float>  pulse-min:  0.2;`

*Line 91 · property*

**undocumented**

---

## `private property <float>  track-alpha: 0.15;`

*Line 100 · property*

**undocumented**

---

## `private property <length> ring-width: Sizes.border-w-2;`

*Line 109 · property*

**undocumented**

---

## `private property <float>  pulse-val: Math.abs(root.anim-phase - Sizes.fill) * (Sizes.fill - root.pulse-min) + root.pulse-min;`

*Line 119 · property*

**undocumented**

---

