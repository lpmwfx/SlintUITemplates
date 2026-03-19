// ── Validation errors ─────────────────────────────────────────────────────────
// NOTE(mother-child): No module-level statics — the &'static str in UnknownIcon
// is a const context string, not mutable global state.

/// Rule violations detected at `build()` time — never silent visual failures.
#[derive(Debug, PartialEq)]
/// D sl er ro r enum.
pub enum DslError {
    /// Nav must have at least one item.
    NoNavItems,
    /// Nav exceeds WinUI3 maximum of 7 items (Android: 5).
    TooManyNavItems { max: usize, got: usize },
    /// Nav item at index N has an empty id.
    NavItemMissingId(usize),
    /// Nav item at index N has an empty label.
    NavItemMissingLabel(usize),
    /// Toolbar item at index N has an empty id.
    ToolbarItemMissingId(usize),
    /// Icon name is not in the FluentIcons registry.
    UnknownIcon { context: &'static str, index: usize, name: String },
    /// Nav item id has no matching ViewSlot registered.
    NavWithoutView(String),
    /// ViewSlot id has no matching nav item.
    ViewWithoutNav(String),
}

impl std::fmt::Display for DslError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DslError::NoNavItems =>
                write!(f, "nav: at least one item required"),
            DslError::TooManyNavItems { max, got } =>
                write!(f, "nav: {got} items exceeds platform maximum of {max}"),
            DslError::NavItemMissingId(i) =>
                write!(f, "nav[{i}]: id must not be empty"),
            DslError::NavItemMissingLabel(i) =>
                write!(f, "nav[{i}]: label must not be empty"),
            DslError::ToolbarItemMissingId(i) =>
                write!(f, "toolbar[{i}]: id must not be empty"),
            DslError::UnknownIcon { context, index, name } =>
                write!(f, "{context}[{index}]: unknown icon \"{name}\" — see dsl::icons"),
            DslError::NavWithoutView(id) =>
                write!(f, "nav \"{id}\" has no registered ViewSlot — add .views([\"{id}\", ...])"),
            DslError::ViewWithoutNav(id) =>
                write!(f, "ViewSlot \"{id}\" has no nav item — remove from .views() or add to .nav()"),
        }
    }
}
