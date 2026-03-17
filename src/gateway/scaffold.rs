/// Default window dimensions for scaffolded projects.
const DEFAULT_WIN_WIDTH: u32 = 1200;
const DEFAULT_WIN_HEIGHT: u32 = 800;

/// Scaffold template file names.
const CARGO_TOML: &str = "Cargo.toml";

/// Scaffold a new SlintUI project at `./<name>/`.
///
/// Creates `src/`, `views/`, `Cargo.toml`, `app.rhai`, `views/*.rhai`, and `src/main.rs`.
pub fn scaffold(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::Path::new(name);
    if root.exists() {
        return Err(format!("directory '{}' already exists", name).into());
    }
    create_dirs(root)?;
    write_cargo_toml(root, name)?;
    write_app_rhai(root, name)?;
    write_view_rhai(root, "home",     "view_status(\"Welcome\");")?;
    write_view_rhai(root, "list",     "view_status(\"Browse\");\nview_toolbar([\"add:add:New\"]);")?;
    write_view_rhai(root, "settings", "view_status(\"Configure\");")?;
    write_main_rs(root)?;
    Ok(())
}

fn create_dirs(root: &std::path::Path) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(root.join("src"))?;
    std::fs::create_dir_all(root.join("views"))?;
    Ok(())
}

fn write_cargo_toml(root: &std::path::Path, name: &str) -> Result<(), std::io::Error> {
    let content = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{name}"
path = "src/main.rs"

[dependencies]
slint-ui-templates = {{ path = "..", features = ["rhai"] }}
rhai = "1"
"#
    );
    std::fs::write(root.join(CARGO_TOML), content)
}

fn write_app_rhai(root: &std::path::Path, name: &str) -> Result<(), std::io::Error> {
    let display = capitalise(name);
    let content = format!(
        r#"// {display} — app configuration
set_nav(["home:Home:home", "list:List:list", "settings:Settings:settings"]);
set_window_size({DEFAULT_WIN_WIDTH}, {DEFAULT_WIN_HEIGHT});
set_bg_style("mica");
set_status("Ready");
"#
    );
    std::fs::write(root.join("app.rhai"), content)
}

fn write_view_rhai(root: &std::path::Path, view: &str, body: &str) -> Result<(), std::io::Error> {
    std::fs::write(root.join("views").join(format!("{view}.rhai")), body)
}

/// Template source for the scaffolded `src/main.rs`.
const MAIN_RS_TEMPLATE: &str = "\
use std::cell::RefCell;\n\
use std::path::Path;\n\
use std::rc::Rc;\n\
\n\
use slint_ui_templates::adapter::AppAdapter;\n\
use slint_ui_templates::bindings::rhai::build_engine;\n\
\n\
fn main() -> Result<(), Box<dyn std::error::Error>> {\n\
    // REASON: Rhai engine closures capture Rc clones for callback access\n\
    let adapter = Rc::new(RefCell::new(AppAdapter::new()?));\n\
    adapter.borrow().load_view_configs(Path::new(\"views\"))?;\n\
    let engine = build_engine(Rc::clone(&adapter));\n\
    engine.eval::<()>(&std::fs::read_to_string(\"app.rhai\")?)?;\n\
    drop(engine);\n\
    let adapter = Rc::try_unwrap(adapter)\n\
        .map_err(|_| \"adapter Rc still has multiple owners\")?;\n\
    adapter.into_inner().run()?;\n\
    Ok(())\n\
}\n\
";

fn write_main_rs(root: &std::path::Path) -> Result<(), std::io::Error> {
    std::fs::write(root.join("src").join("main.rs"), MAIN_RS_TEMPLATE)
}

fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None    => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
