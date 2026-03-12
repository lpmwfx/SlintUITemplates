//! Composition DSL — fluent builder API for SlintUI shell configuration.
//!
//! Rules from `rules/slint/composition.md` are enforced **by construction**:
//! wrong configurations are `DslError`, not silent visual failures.
//!
//! # Example
//! ```rust,no_run
//! use slint_ui_templates::dsl::{AppDsl, Nav, Toolbar};
//! use slint_ui_templates::shell::Platform;
//!
//! let dsl = AppDsl::builder("My App")
//!     .platform(Platform::Windows)
//!     .window_size(1100, 720)
//!     .nav(vec![
//!         Nav::new("home",     "Home",     "home"),
//!         Nav::new("list",     "List",     "list"),
//!         Nav::new("settings", "Settings", "settings"),
//!     ])
//!     .status("Ready")
//!     .build()
//!     .expect("invalid DSL configuration");
//! ```

pub mod apply;
pub mod icons;

use icons::fluent_icon;
use crate::shell::Platform;

// ── Public input types ────────────────────────────────────────────────────────

/// A navigation destination — icon name resolved to codepoint at build().
#[derive(Debug, Clone)]
pub struct Nav {
    pub id:    String,
    pub label: String,
    pub icon:  String,
}

impl Nav {
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
pub struct Toolbar {
    pub id:      String,
    pub icon:    String,
    pub tooltip: String,
}

impl Toolbar {
    pub fn new(
        id:      impl Into<String>,
        icon:    impl Into<String>,
        tooltip: impl Into<String>,
    ) -> Self {
        Self { id: id.into(), icon: icon.into(), tooltip: tooltip.into() }
    }
}

// ── Validation errors ─────────────────────────────────────────────────────────

/// Rule violations detected at `build()` time — never silent visual failures.
#[derive(Debug, PartialEq)]
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

// ── Resolved internal types ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct ResolvedNav {
    pub id:        String,
    pub label:     String,
    pub icon_code: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedToolbar {
    pub id:        String,
    pub icon_code: String,
    pub tooltip:   String,
}

// ── Validated configuration ───────────────────────────────────────────────────

/// Validated, sealed shell configuration.
/// Can only be constructed via `AppDsl::builder().build()`.
#[derive(Debug)]
pub struct AppDsl {
    pub(crate) title:        String,
    pub(crate) platform:     Platform,
    pub(crate) nav:          Vec<ResolvedNav>,
    pub(crate) status:       String,
    pub(crate) toolbar:      Vec<ResolvedToolbar>,
    pub(crate) show_toolbar: bool,
    pub(crate) window_size:  Option<(u32, u32)>,
    /// Validated view ids — matches nav ids 1:1 when provided.
    pub(crate) views:        Vec<String>,
}

// ── Builder ───────────────────────────────────────────────────────────────────

pub struct AppDslBuilder {
    title:       String,
    platform:    Platform,
    nav:         Vec<Nav>,
    status:      String,
    toolbar:     Vec<Toolbar>,
    window_size: Option<(u32, u32)>,
    views:       Vec<String>,
}

impl AppDsl {
    pub fn builder(title: impl Into<String>) -> AppDslBuilder {
        AppDslBuilder {
            title:       title.into(),
            platform:    Platform::Windows,
            nav:         Vec::new(),
            status:      "Ready".into(),
            toolbar:     Vec::new(),
            window_size: None,
            views:       Vec::new(),
        }
    }
}

impl AppDslBuilder {
    pub fn platform(mut self, p: Platform) -> Self {
        self.platform = p;
        self
    }

    pub fn nav(mut self, items: Vec<Nav>) -> Self {
        self.nav = items;
        self
    }

    pub fn status(mut self, text: impl Into<String>) -> Self {
        self.status = text.into();
        self
    }

    pub fn toolbar(mut self, items: Vec<Toolbar>) -> Self {
        self.toolbar = items;
        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_size = Some((width, height));
        self
    }

    /// Register ViewSlot ids — validated to match nav ids 1:1.
    /// Optional: if omitted, nav↔view consistency is not checked.
    pub fn views(mut self, ids: Vec<&str>) -> Self {
        self.views = ids.into_iter().map(String::from).collect();
        self
    }

    /// Validate all composition rules. Returns sealed `AppDsl` or list of errors.
    pub fn build(self) -> Result<AppDsl, Vec<DslError>> {
        let mut errors: Vec<DslError> = Vec::new();

        let nav_max = if self.platform.is_mobile() { 5 } else { 7 };

        // Rule: 1–max nav items
        if self.nav.is_empty() {
            errors.push(DslError::NoNavItems);
        } else if self.nav.len() > nav_max {
            errors.push(DslError::TooManyNavItems { max: nav_max, got: self.nav.len() });
        }

        // Rule: each nav item needs id + label + known icon
        let mut resolved_nav: Vec<ResolvedNav> = Vec::new();
        for (i, item) in self.nav.iter().enumerate() {
            if item.id.is_empty()    { errors.push(DslError::NavItemMissingId(i)); }
            if item.label.is_empty() { errors.push(DslError::NavItemMissingLabel(i)); }
            match fluent_icon(&item.icon) {
                Some(code) => resolved_nav.push(ResolvedNav {
                    id:        item.id.clone(),
                    label:     item.label.clone(),
                    icon_code: code.into(),
                }),
                None => errors.push(DslError::UnknownIcon {
                    context: "nav",
                    index:   i,
                    name:    item.icon.clone(),
                }),
            }
        }

        // Rule: each toolbar item needs id + known icon
        let mut resolved_toolbar: Vec<ResolvedToolbar> = Vec::new();
        for (i, item) in self.toolbar.iter().enumerate() {
            if item.id.is_empty() { errors.push(DslError::ToolbarItemMissingId(i)); }
            match fluent_icon(&item.icon) {
                Some(code) => resolved_toolbar.push(ResolvedToolbar {
                    id:        item.id.clone(),
                    icon_code: code.into(),
                    tooltip:   item.tooltip.clone(),
                }),
                None => errors.push(DslError::UnknownIcon {
                    context: "toolbar",
                    index:   i,
                    name:    item.icon.clone(),
                }),
            }
        }

        // Rule: nav ids ↔ view ids must match 1:1 (only when views() is provided)
        if !self.views.is_empty() {
            let nav_ids: std::collections::HashSet<&str> =
                self.nav.iter().map(|n| n.id.as_str()).collect();
            let view_ids: std::collections::HashSet<&str> =
                self.views.iter().map(|s| s.as_str()).collect();

            for id in nav_ids.difference(&view_ids) {
                errors.push(DslError::NavWithoutView(id.to_string()));
            }
            for id in view_ids.difference(&nav_ids) {
                errors.push(DslError::ViewWithoutNav(id.to_string()));
            }
        }

        if errors.is_empty() {
            Ok(AppDsl {
                title:        self.title,
                platform:     self.platform,
                nav:          resolved_nav,
                status:       self.status,
                show_toolbar: !resolved_toolbar.is_empty(),
                toolbar:      resolved_toolbar,
                window_size:  self.window_size,
                views:        self.views,
            })
        } else {
            Err(errors)
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn three_nav() -> Vec<Nav> {
        vec![
            Nav::new("home",     "Home",     "home"),
            Nav::new("list",     "List",     "list"),
            Nav::new("settings", "Settings", "settings"),
        ]
    }

    #[test]
    fn valid_dsl_builds() {
        let dsl = AppDsl::builder("My App").nav(three_nav()).build().unwrap();
        assert_eq!(dsl.nav.len(), 3);
        assert_eq!(dsl.nav[0].id, "home");
        assert_eq!(dsl.nav[0].icon_code, "\u{E80F}");
    }

    #[test]
    fn empty_nav_is_error() {
        let errs = AppDsl::builder("App").build().unwrap_err();
        assert!(errs.contains(&DslError::NoNavItems));
    }

    #[test]
    fn too_many_nav_desktop_is_error() {
        let nav = (0..8).map(|i| Nav::new(format!("id{i}"), format!("L{i}"), "home")).collect();
        let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
        assert!(errs.contains(&DslError::TooManyNavItems { max: 7, got: 8 }));
    }

    #[test]
    fn too_many_nav_android_max_5() {
        let nav = (0..6).map(|i| Nav::new(format!("id{i}"), format!("L{i}"), "home")).collect();
        let errs = AppDsl::builder("App")
            .platform(Platform::Android).nav(nav).build().unwrap_err();
        assert!(errs.contains(&DslError::TooManyNavItems { max: 5, got: 6 }));
    }

    #[test]
    fn unknown_icon_is_error() {
        let nav = vec![Nav::new("home", "Home", "not-an-icon")];
        let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
        assert!(matches!(&errs[0], DslError::UnknownIcon { name, .. } if name == "not-an-icon"));
    }

    #[test]
    fn missing_nav_id_is_error() {
        let nav = vec![Nav::new("", "Home", "home")];
        let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
        assert!(errs.contains(&DslError::NavItemMissingId(0)));
    }

    #[test]
    fn toolbar_resolves_icons() {
        let dsl = AppDsl::builder("App")
            .nav(three_nav())
            .toolbar(vec![Toolbar::new("save", "save", "Save file")])
            .build().unwrap();
        assert!(dsl.show_toolbar);
        assert_eq!(dsl.toolbar[0].icon_code, "\u{E74E}");
    }

    #[test]
    fn toolbar_unknown_icon_is_error() {
        let errs = AppDsl::builder("App")
            .nav(three_nav())
            .toolbar(vec![Toolbar::new("x", "not-real", "Tip")])
            .build().unwrap_err();
        assert!(matches!(&errs[0], DslError::UnknownIcon { context: "toolbar", .. }));
    }

    #[test]
    fn toolbar_missing_id_is_error() {
        let errs = AppDsl::builder("App")
            .nav(three_nav())
            .toolbar(vec![Toolbar::new("", "save", "Save")])
            .build().unwrap_err();
        assert!(errs.contains(&DslError::ToolbarItemMissingId(0)));
    }

    #[test]
    fn window_size_stored() {
        let dsl = AppDsl::builder("App").nav(three_nav()).window_size(1100, 720).build().unwrap();
        assert_eq!(dsl.window_size, Some((1100, 720)));
    }

    #[test]
    fn status_default_is_ready() {
        let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
        assert_eq!(dsl.status, "Ready");
    }

    #[test]
    fn no_toolbar_hides_it() {
        let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
        assert!(!dsl.show_toolbar);
    }

    #[test]
    fn platform_default_is_windows() {
        let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
        assert_eq!(dsl.platform, Platform::Windows);
    }

    #[test]
    fn views_matching_nav_is_ok() {
        let dsl = AppDsl::builder("App")
            .nav(three_nav())
            .views(vec!["home", "list", "settings"])
            .build();
        assert!(dsl.is_ok());
    }

    #[test]
    fn nav_without_view_is_error() {
        let errs = AppDsl::builder("App")
            .nav(three_nav())
            .views(vec!["home", "list"])  // missing "settings"
            .build().unwrap_err();
        assert!(errs.iter().any(|e| matches!(e, DslError::NavWithoutView(id) if id == "settings")));
    }

    #[test]
    fn view_without_nav_is_error() {
        let errs = AppDsl::builder("App")
            .nav(three_nav())
            .views(vec!["home", "list", "settings", "extra"])
            .build().unwrap_err();
        assert!(errs.iter().any(|e| matches!(e, DslError::ViewWithoutNav(id) if id == "extra")));
    }

    #[test]
    fn views_optional_no_check_when_omitted() {
        // No .views() call — nav↔view check skipped
        let dsl = AppDsl::builder("App").nav(three_nav()).build();
        assert!(dsl.is_ok());
    }

    #[test]
    fn multiple_errors_collected() {
        let nav = vec![
            Nav::new("", "Home", "home"),      // missing id
            Nav::new("x", "X",   "bad-icon"),  // bad icon
        ];
        let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
        assert_eq!(errs.len(), 2);
    }
}
