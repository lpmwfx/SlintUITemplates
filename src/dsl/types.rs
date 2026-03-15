// ── Background style ─────────────────────────────────────────────────────────

/// Windows Composition API backdrop style.
/// Mica and Acrylic require Windows 11 (22H2+) — silently falls back to Solid
/// on older OS versions or non-Windows platforms.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// B gs ty le enum.
pub enum BgStyle {
    /// Opaque window — Slint draws `Colors.bg-primary` as background.
    #[default]
    Solid,
    /// Mica — frosted desktop wallpaper backdrop (DWMSBT_MAINWINDOW).
    Mica,
    /// Acrylic — blurred ambient backdrop (DWMSBT_TRANSIENTWINDOW).
    Acrylic,
}

impl BgStyle {
    /// Parse from string — unknown values default to `Solid`.
    pub fn from_str(s: &str) -> Self {
        match s {
            s if s.eq_ignore_ascii_case("mica")    => BgStyle::Mica,
            s if s.eq_ignore_ascii_case("acrylic") => BgStyle::Acrylic,
            _ => BgStyle::Solid,
        }
    }
}

// ── Public input types ────────────────────────────────────────────────────────

/// A navigation destination — icon name resolved to codepoint at build().
#[derive(Debug, Clone)]
/// N av struct.
pub struct Nav {
    /// I d.
    pub id:    String,
    /// L ab el.
    pub label: String,
    /// I co n.
    pub icon:  String,
}

impl Nav {
    /// Create a navigation item with the given id, display label, and icon name.
    pub fn new(
        id:    impl Into<String>,
        label: impl Into<String>,
        icon:  impl Into<String>,
    ) -> Self {
        Self { id: id.into(), label: label.into(), icon: icon.into() }
    }
}

/// A toolbar icon button — icon name resolved to codepoint at build().
#[derive(Debug, Clone)]
/// T oo lb ar struct.
pub struct Toolbar {
    /// I d.
    pub id:      String,
    /// I co n.
    pub icon:    String,
    /// T oo lt ip.
    pub tooltip: String,
}

impl Toolbar {
    /// Create a toolbar button with the given id, icon name, and tooltip text.
    pub fn new(
        id:      impl Into<String>,
        icon:    impl Into<String>,
        tooltip: impl Into<String>,
    ) -> Self {
        Self { id: id.into(), icon: icon.into(), tooltip: tooltip.into() }
    }
}
