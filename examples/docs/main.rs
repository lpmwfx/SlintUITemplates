use slint::ComponentHandle;
use slint_ui_templates::{docs, dsl::BgStyle, platform, DocBlock, DocsApp, NavItem, Theme};

// Map: nav-id → (sidebar label, icon name, docs/ filename)
const PAGES: &[(&str, &str, &str, &str)] = &[
    ("getting-started", "Getting Started", "home",   "getting-started.md"),
    ("architecture",    "Architecture",    "layout", "architecture.md"),
    ("theme-system",    "Theme System",    "settings","theme-system.md"),
    ("dsl",             "DSL",             "apps",   "dsl.md"),
    ("settings",        "Settings",        "filter", "settings.md"),
    ("tokens",          "Tokens",          "grid",   "tokens.md"),
    ("components",      "Components",      "list",   "components.md"),
    ("platform-rules",  "Platform Rules",  "code",   "platform-rules.md"),
];

// ── Helpers ───────────────────────────────────────────────────────────────────

fn icon(name: &str) -> slint::SharedString {
    slint_ui_templates::dsl::icons::fluent_icon(name)
        .map(slint::SharedString::from)
        .unwrap_or_default()
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
        if c.is_dir() { return c.clone(); }
    }
    std::path::PathBuf::from("docs")
}

fn load_page(page_id: &str) -> Vec<DocBlock> {
    let filename = PAGES.iter()
        .find(|(id, ..)| *id == page_id)
        .map(|(.., f)| *f)
        .unwrap_or("getting-started.md");

    let path = docs_dir().join(filename);
    match std::fs::read_to_string(&path) {
        Ok(md) => docs::parse(&md),
        Err(e) => docs::parse(&format!(
            "# {page_id}\n\nCould not read `{}`:\n\n```\n{e}\n```\n\nRun from the repo root: `cargo run --example docs`",
            path.display()
        )),
    }
}

fn title_for(page_id: &str) -> slint::SharedString {
    PAGES.iter()
        .find(|(id, ..)| *id == page_id)
        .map(|(_, label, ..)| slint::SharedString::from(*label))
        .unwrap_or_default()
}

fn push_blocks(handle: &DocsApp, page_id: &str) {
    let blocks = load_page(page_id);
    handle.set_doc_blocks(slint::ModelRc::new(slint::VecModel::from(blocks)));
    handle.set_doc_title(title_for(page_id));
    handle.set_status_text(format!("{} — SlintUITemplates Docs", title_for(page_id)).into());
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = DocsApp::new()?;

    // Build nav items
    let nav: Vec<NavItem> = PAGES.iter().map(|(id, label, icon_name, _)| NavItem {
        id:    (*id).into(),
        label: (*label).into(),
        icon:  icon(icon_name),
    }).collect();
    ui.set_nav_items(slint::ModelRc::new(slint::VecModel::from(nav)));

    // Load initial page
    let initial = ui.get_active_view().to_string();
    push_blocks(&ui, &initial);

    // Navigate handler
    let weak = ui.as_weak();
    ui.on_navigate(move |id| {
        if let Some(h) = weak.upgrade() {
            push_blocks(&h, id.as_str());
        }
    });

    // Theme/backdrop toggle via menu
    let weak = ui.as_weak();
    ui.on_request_bg_style(move |style| {
        if let Some(h) = weak.upgrade() {
            let bg = match style.as_str() {
                "mica"    => BgStyle::Mica,
                "acrylic" => BgStyle::Acrylic,
                _         => BgStyle::Solid,
            };
            platform::apply_backdrop(h.window(), bg);
            h.global::<Theme>().set_material(style);
        }
    });

    ui.show()?;
    platform::apply_backdrop(ui.window(), BgStyle::Solid);
    ui.run()?;
    Ok(())
}
