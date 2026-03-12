//! Composition DSL — fluent builder API for SlintUI shell configuration.
//!
//! Rules from `rules/slint/composition.md` are enforced **by construction**:
//! wrong configurations are `DslError`, not silent visual failures.
//!
//! # Example
//! ```rust,no_run
//! use slint_ui_templates::dsl::{AppDsl, Nav, Toolbar};
//!
//! let dsl = AppDsl::builder("My App")
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

// ── Public types ──────────────────────────────────────────────────────────────

/// A navigation destination passed to AppShell / DesktopShell.
#[derive(Debug, Clone)]
pub struct Nav {
    pub id:    String,
    pub label: String,
    /// Icon name (e.g. "home") — resolved to Segoe Fluent codepoint by DSL.
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

/// A toolbar icon button.
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

/// Rule violations detected at `build()` time.
#[derive(Debug, PartialEq)]
pub enum DslError {
    /// Nav must have at least one item.
    NoNavItems,
    /// Nav must have at most 7 items (WinUI3 NavigationView rule).
    TooManyNavItems(usize),
    /// Nav item at index N has an empty id.
    NavItemMissingId(usize),
    /// Nav item at index N has an empty label.
    NavItemMissingLabel(usize),
    /// Icon name at index N is not in the FluentIcons registry.
    UnknownIcon { index: usize, name: String },
}

impl std::fmt::Display for DslError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DslError::NoNavItems =>
                write!(f, "nav: at least one nav item required"),
            DslError::TooManyNavItems(n) =>
                write!(f, "nav: {n} items exceeds WinUI3 maximum of 7"),
            DslError::NavItemMissingId(i) =>
                write!(f, "nav[{i}]: id must not be empty"),
            DslError::NavItemMissingLabel(i) =>
                write!(f, "nav[{i}]: label must not be empty"),
            DslError::UnknownIcon { index, name } =>
                write!(f, "nav[{index}]: unknown icon \"{name}\" — see dsl::icons::fluent_icon()"),
        }
    }
}

// ── Validated configuration ───────────────────────────────────────────────────

/// Validated shell configuration — can only be constructed via `AppDsl::builder().build()`.
/// Holds resolved codepoints, not raw icon names.
#[derive(Debug)]
pub struct AppDsl {
    pub(crate) title:        String,
    pub(crate) nav:          Vec<ResolvedNav>,
    pub(crate) status:       String,
    pub(crate) toolbar:      Vec<Toolbar>,
    pub(crate) show_toolbar: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedNav {
    pub id:        String,
    pub label:     String,
    pub icon_code: String,   // resolved Segoe Fluent codepoint
}

// ── Builder ───────────────────────────────────────────────────────────────────

pub struct AppDslBuilder {
    title:   String,
    nav:     Vec<Nav>,
    status:  String,
    toolbar: Vec<Toolbar>,
}

impl AppDsl {
    pub fn builder(title: impl Into<String>) -> AppDslBuilder {
        AppDslBuilder {
            title:   title.into(),
            nav:     Vec::new(),
            status:  "Ready".into(),
            toolbar: Vec::new(),
        }
    }
}

impl AppDslBuilder {
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

    /// Validate all composition rules and return a sealed `AppDsl` or a list of errors.
    pub fn build(self) -> Result<AppDsl, Vec<DslError>> {
        let mut errors: Vec<DslError> = Vec::new();

        // Rule: 1–7 nav items (WinUI3 NavigationView)
        if self.nav.is_empty() {
            errors.push(DslError::NoNavItems);
        } else if self.nav.len() > 7 {
            errors.push(DslError::TooManyNavItems(self.nav.len()));
        }

        // Rule: each nav item needs id + label + known icon
        let mut resolved: Vec<ResolvedNav> = Vec::new();
        for (i, item) in self.nav.iter().enumerate() {
            if item.id.is_empty()    { errors.push(DslError::NavItemMissingId(i)); }
            if item.label.is_empty() { errors.push(DslError::NavItemMissingLabel(i)); }

            match fluent_icon(&item.icon) {
                Some(code) => resolved.push(ResolvedNav {
                    id:        item.id.clone(),
                    label:     item.label.clone(),
                    icon_code: code.into(),
                }),
                None => errors.push(DslError::UnknownIcon {
                    index: i,
                    name:  item.icon.clone(),
                }),
            }
        }

        if errors.is_empty() {
            Ok(AppDsl {
                title:        self.title,
                nav:          resolved,
                status:       self.status,
                show_toolbar: !self.toolbar.is_empty(),
                toolbar:      self.toolbar,
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
        let dsl = AppDsl::builder("My App").nav(three_nav()).build();
        assert!(dsl.is_ok());
        let dsl = dsl.unwrap();
        assert_eq!(dsl.nav.len(), 3);
        assert_eq!(dsl.nav[0].id, "home");
        assert_eq!(dsl.nav[0].icon_code, "\u{E80F}");  // resolved home codepoint
    }

    #[test]
    fn empty_nav_is_error() {
        let errs = AppDsl::builder("App").build().unwrap_err();
        assert!(errs.contains(&DslError::NoNavItems));
    }

    #[test]
    fn too_many_nav_items_is_error() {
        let nav = (0..8).map(|i| Nav::new(
            format!("id{i}"), format!("Label {i}"), "home"
        )).collect();
        let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
        assert!(errs.contains(&DslError::TooManyNavItems(8)));
    }

    #[test]
    fn unknown_icon_is_error() {
        let nav = vec![Nav::new("home", "Home", "not-an-icon")];
        let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
        assert!(matches!(&errs[0], DslError::UnknownIcon { name, .. } if name == "not-an-icon"));
    }

    #[test]
    fn missing_id_is_error() {
        let nav = vec![Nav::new("", "Home", "home")];
        let errs = AppDsl::builder("App").nav(nav).build().unwrap_err();
        assert!(errs.contains(&DslError::NavItemMissingId(0)));
    }

    #[test]
    fn status_default_is_ready() {
        let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
        assert_eq!(dsl.status, "Ready");
    }

    #[test]
    fn status_override() {
        let dsl = AppDsl::builder("App").nav(three_nav()).status("Connected").build().unwrap();
        assert_eq!(dsl.status, "Connected");
    }

    #[test]
    fn toolbar_sets_show_toolbar() {
        let dsl = AppDsl::builder("App")
            .nav(three_nav())
            .toolbar(vec![Toolbar::new("save", "save", "Save")])
            .build().unwrap();
        assert!(dsl.show_toolbar);
    }

    #[test]
    fn no_toolbar_hides_it() {
        let dsl = AppDsl::builder("App").nav(three_nav()).build().unwrap();
        assert!(!dsl.show_toolbar);
    }
}
