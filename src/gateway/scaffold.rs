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
    write_build_rs(root)?;
    write_app_slint(root)?;
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
    std::fs::create_dir_all(root.join("ui"))?;
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
slint-ui-templates = {{ version = "0.1", features = ["rhai"] }}
rhai = "1"

[build-dependencies]
slint-build = "1.15"
"#
    );
    std::fs::write(root.join(CARGO_TOML), content)
}

fn write_build_rs(root: &std::path::Path) -> Result<(), std::io::Error> {
    std::fs::write(
        root.join("build.rs"),
        r#"fn main() {
    let ui_path = match std::env::var("DEP_SLINT_UI_TEMPLATES_SLINT_INCLUDE_PATH") {
        Ok(p) => std::path::PathBuf::from(p),
        Err(_) => {
            eprintln!("slint-ui-templates must be a dependency");
            std::process::exit(1);
        }
    };
    let config = slint_build::CompilerConfiguration::new()
        .with_include_paths(vec![ui_path]);
    if let Err(e) = slint_build::compile_with_config("ui/app.slint", config) {
        eprintln!("slint compile failed: {e}");
        std::process::exit(1);
    }
}
"#,
    )
}

fn write_app_slint(root: &std::path::Path) -> Result<(), std::io::Error> {
    std::fs::write(root.join("ui").join("app.slint"), "\
import { Button, Toggle, Card } from \"components.slint\";\n\
import { Colors, Spacing, Type } from \"theme.slint\";\n\
\n\
export component App inherits Window {\n\
    title: \"My App\";\n\
    preferred-width: 800px;\n\
    preferred-height: 600px;\n\
    background: Colors.bg-primary;\n\
\n\
    Text {\n\
        text: \"Hello from SlintUITemplates!\";\n\
        color: Colors.text-primary;\n\
        font-size: Type.subtitle-size;\n\
        horizontal-alignment: center;\n\
        vertical-alignment: center;\n\
    }\n\
}\n\
")
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
    let engine = build_engine(Rc::clone(&adapter)); // why shared? engine closures need adapter access\n\
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
