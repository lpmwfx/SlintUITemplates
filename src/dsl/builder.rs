use crate::shell::Platform;
use crate::dsl::icons::fluent_icon;
use super::{AppDsl, ResolvedNav, ResolvedToolbar, DslError, Nav, Toolbar};

/// Maximum nav items allowed on mobile platforms (Android/iOS).
const NAV_MAX_MOBILE: usize = 5;
/// Maximum nav items allowed on desktop platforms (Windows/macOS/Linux).
const NAV_MAX_DESKTOP: usize = 7;

/// Fluent builder for constructing a validated `AppDsl` configuration.
pub struct AppDslBuilder {
    title:       String,
    platform:    Platform,
    nav:         Vec<Nav>,
    status:      String,
    toolbar:     Vec<Toolbar>,
    window_size: Option<(u32, u32)>,
    bg_style:    super::BgStyle,
    views:       Vec<String>,
}

fn resolve_nav_item(item: &Nav, index: usize) -> (Option<ResolvedNav>, Vec<DslError>) {
    let mut errors = Vec::new();
    if item.id.is_empty()    { errors.push(DslError::NavItemMissingId(index)); }
    if item.label.is_empty() { errors.push(DslError::NavItemMissingLabel(index)); }
    match fluent_icon(&item.icon) {
        Some(code) => (Some(ResolvedNav {
            id:        item.id.to_owned(),
            label:     item.label.clone(),
            icon_code: code.into(),
        }), errors),
        None => {
            errors.push(DslError::UnknownIcon { context: "nav", index, name: item.icon.clone() });
            (None, errors)
        }
    }
}

fn resolve_toolbar_item(item: &Toolbar, index: usize) -> (Option<ResolvedToolbar>, Vec<DslError>) {
    let mut errors = Vec::new();
    if item.id.is_empty() { errors.push(DslError::ToolbarItemMissingId(index)); }
    match fluent_icon(&item.icon) {
        Some(code) => (Some(ResolvedToolbar {
            id:        item.id.to_owned(),
            icon_code: code.into(),
            tooltip:   item.tooltip.clone(),
        }), errors),
        None => {
            errors.push(DslError::UnknownIcon { context: "toolbar", index, name: item.icon.clone() });
            (None, errors)
        }
    }
}

impl AppDslBuilder {
    /// Create a new builder with the given window title.
    pub(crate) fn new(title: impl Into<String>) -> Self {
        Self {
            title:       title.into(),
            platform:    Platform::Windows,
            nav:         Vec::new(),
            status:      "Ready".into(),
            toolbar:     Vec::new(),
            window_size: None,
            bg_style:    super::BgStyle::Solid,
            views:       Vec::new(),
        }
    }

    // REASON: builder pattern — each method is a single-field setter
    /// Set the target platform (affects nav-item limits and layout rules).
    pub fn platform(mut self, p: Platform) -> Self {
        self.platform = p;
        self
    }

    /// Set the sidebar navigation items (validated at `build()` time).
    pub fn nav(mut self, items: Vec<Nav>) -> Self {
        self.nav = items;
        self
    }

    /// Set the initial status-bar text (defaults to "Ready").
    pub fn status(mut self, text: impl Into<String>) -> Self {
        self.status = text.into();
        self
    }

    /// Set the toolbar icon buttons (validated at `build()` time).
    pub fn toolbar(mut self, items: Vec<Toolbar>) -> Self {
        self.toolbar = items;
        self
    }

    /// Set the initial window dimensions in logical pixels.
    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_size = Some((width, height));
        self
    }

    /// Set OS-level window backdrop (Windows 11 only; no-op elsewhere).
    pub fn bg_style(mut self, style: super::BgStyle) -> Self {
        self.bg_style = style;
        self
    }

    /// Register ViewSlot ids — validated to match nav ids 1:1.
    pub fn views(mut self, ids: Vec<&str>) -> Self {
        self.views = ids.into_iter().map(String::from).collect();
        self
    }

    /// Validate all composition rules. Returns sealed `AppDsl` or list of errors.
    pub fn build(self) -> Result<AppDsl, Vec<DslError>> {
        let mut errors: Vec<DslError> = Vec::new();

        let nav_max = if self.platform.is_mobile() || self.platform.is_small() {
            NAV_MAX_MOBILE
        } else {
            NAV_MAX_DESKTOP
        };

        if self.nav.is_empty() {
            errors.push(DslError::NoNavItems);
        } else if self.nav.len() > nav_max {
            errors.push(DslError::TooManyNavItems { max: nav_max, got: self.nav.len() });
        }

        let mut resolved_nav: Vec<ResolvedNav> = Vec::new();
        for (i, item) in self.nav.iter().enumerate() {
            let (resolved, errs) = resolve_nav_item(item, i);
            errors.extend(errs);
            if let Some(nav) = resolved { resolved_nav.push(nav); }
        }

        let mut resolved_toolbar: Vec<ResolvedToolbar> = Vec::new();
        for (i, item) in self.toolbar.iter().enumerate() {
            let (resolved, errs) = resolve_toolbar_item(item, i);
            errors.extend(errs);
            if let Some(tb) = resolved { resolved_toolbar.push(tb); }
        }

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
                nav:          resolved_nav,
                status:       self.status,
                show_toolbar: !resolved_toolbar.is_empty(),
                toolbar:      resolved_toolbar,
                window_size:  self.window_size,
                bg_style:     self.bg_style,
                platform:     self.platform,
            })
        } else {
            Err(errors)
        }
    }
}
