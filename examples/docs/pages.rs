use std::rc::Rc;

use slint::{ComponentHandle, Model, ModelRc, VecModel};
use slint_ui_templates::{docs, dsl::BgStyle, platform, DocBlock, DocsApp, NavItem, Theme};

// Map: nav-id → (sidebar label, icon name, docs/ filename)
const PAGES: &[(&str, &str, &str, &str)] = &[
    ("getting-started", "Getting Started", "home",    "getting-started.md"),
    ("architecture",    "Architecture",    "layout",  "architecture.md"),
    ("theme-system",    "Theme System",    "settings","theme-system.md"),
    ("dsl",             "DSL",             "apps",    "dsl.md"),
    ("settings",        "Settings",        "filter",  "settings.md"),
    ("tokens",          "Tokens",          "grid",    "tokens.md"),
    ("components",      "Components",      "list",    "components.md"),
    ("platform-rules",  "Platform Rules",  "code",    "platform-rules.md"),
];

const BG_MICA: &str = "mica";
const BG_ACRYLIC: &str = "acrylic";

/// Build and configure a `DocsApp` with nav items, initial page, and all callbacks wired.
pub fn build() -> Result<DocsApp, Box<dyn std::error::Error>> {
    let ui = DocsApp::new()?;
    let model = wire_nav(&ui);
    wire_callbacks(&ui, model);
    Ok(ui)
}

/// Push doc blocks for `page_id` into the UI and update title + status bar.
pub fn push_blocks(handle: &DocsApp, page_id: &str) {
    let blocks = load_page(page_id);
    handle.set_doc_blocks(ModelRc::new(VecModel::from(blocks)));
    handle.set_doc_title(title_for(page_id));
    handle.set_status_text(
        format!("{} — SlintUITemplates Docs", title_for(page_id)).into(),
    );
}

// ── Nav ──────────────────────────────────────────────────────────────────────

fn wire_nav(ui: &DocsApp) -> Rc<VecModel<NavItem>> {
    let nav = build_nav_items();
    // Rc: shared between set_nav_items and the toggle_group closure
    let model = Rc::new(VecModel::from(nav));
    ui.set_nav_items(ModelRc::new(model.clone()));
    push_blocks(ui, "getting-started");
    model
}

fn build_nav_items() -> Vec<NavItem> {
    PAGES
        .iter()
        .map(|(id, label, icon_name, _)| NavItem {
            id:        (*id).into(),
            label:     (*label).into(),
            icon:      icon(icon_name),
            is_header: false,
            hidden:    false,
        })
        .collect()
}

fn icon(name: &str) -> slint::SharedString {
    slint_ui_templates::dsl::icons::fluent_icon(name)
        .map(slint::SharedString::from)
        .unwrap_or_default()
}

// ── Callbacks ─────────────────────────────────────────────────────────────────

fn wire_callbacks(ui: &DocsApp, nav_model: Rc<VecModel<NavItem>>) {
    wire_navigate(ui);
    wire_toggle(ui, nav_model);
    wire_bg_style(ui);
}

fn wire_navigate(ui: &DocsApp) {
    let weak = ui.as_weak();
    ui.on_navigate(move |id| {
        if let Some(h) = weak.upgrade() {
            push_blocks(&h, id.as_str());
        }
    });
}

fn wire_toggle(ui: &DocsApp, nav_model: Rc<VecModel<NavItem>>) {
    ui.on_toggle_group(move |group_id| {
        // Pass 1: find current hidden state of first child under this header
        let mut is_in_group = false;
        let mut is_currently_hidden = false;
        for i in 0..nav_model.row_count() {
            let Some(nav_entry) = nav_model.row_data(i) else { continue };
            if nav_entry.id.as_str() == group_id.as_str() {
                is_in_group = true;
                continue;
            }
            if is_in_group {
                if nav_entry.is_header { break; }
                is_currently_hidden = nav_entry.hidden;
                break;
            }
        }
        // Pass 2: toggle all children under this header
        let mut is_in_group = false;
        for i in 0..nav_model.row_count() {
            let Some(mut nav_entry) = nav_model.row_data(i) else { continue };
            if nav_entry.id.as_str() == group_id.as_str() {
                is_in_group = true;
                continue;
            }
            if is_in_group {
                if nav_entry.is_header { break; }
                nav_entry.hidden = !is_currently_hidden;
                nav_model.set_row_data(i, nav_entry);
            }
        }
    });
}

fn wire_bg_style(ui: &DocsApp) {
    let weak = ui.as_weak();
    ui.on_request_bg_style(move |style| {
        if let Some(h) = weak.upgrade() {
            let bg = match style.as_str() {
                BG_MICA    => BgStyle::Mica,
                BG_ACRYLIC => BgStyle::Acrylic,
                _          => BgStyle::Solid,
            };
            platform::apply_backdrop(h.window(), bg);
            h.global::<Theme>().set_material(style);
        }
    });
}

// ── Page loading ──────────────────────────────────────────────────────────────

fn load_page(page_id: &str) -> Vec<DocBlock> {
    let filename = PAGES
        .iter()
        .find(|(id, ..)| *id == page_id)
        .map(|(.., f)| *f)
        .unwrap_or("getting-started.md");

    let path = docs_dir().join(filename);
    match crate::gateway::read_md(&path) {
        Some(md) => docs::parse(&md),
        None => docs::parse(&format!(
            "# {page_id}\n\nCould not read docs.\n\nRun from the repo root: `cargo run --example docs`",
        )),
    }
}

fn docs_dir() -> std::path::PathBuf {
    // Try: next to exe → cwd → ../../docs (when run via cargo run)
    let candidates = [
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("docs"))),
        Some(std::path::PathBuf::from("docs")),
        Some(std::path::PathBuf::from("../../docs")),
    ];
    for c in candidates.iter().flatten() {
        if c.is_dir() {
            return c.clone();
        }
    }
    std::path::PathBuf::from("docs")
}

fn title_for(page_id: &str) -> slint::SharedString {
    PAGES
        .iter()
        .find(|(id, ..)| *id == page_id)
        .map(|(_, label, ..)| slint::SharedString::from(*label))
        .unwrap_or_default()
}
