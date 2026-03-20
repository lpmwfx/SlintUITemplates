# `grid/config.rs`

## `pub struct TargetConfig`
*Line 6 · struct*

Top-level configuration for a deployment target (e.g. desktop, tablet).
T ar ge tc on fi g struct.

---

## `pub struct TargetInfo`
*Line 16 · struct*

Metadata describing a target's name, screen class, and interaction model.
T ar ge ti nf o struct.

---

## `pub struct GridConfig`
*Line 28 · struct*

Grid layout definition containing the ordered list of row configurations.
G ri dc on fi g struct.

---

## `pub struct RowConfig`
*Line 36 · struct*

Configuration for a single grid row, optionally fixed or subdivided into columns.
R ow co nf ig struct.

---

## `pub struct ColumnConfig`
*Line 51 · struct*

Configuration for a single column within a grid row.
C ol um nc on fi g struct.

---

## `pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>>`
*Line 60 · fn*

Reads and deserializes a target configuration from a TOML file (delegated to gateway).

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=1" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
