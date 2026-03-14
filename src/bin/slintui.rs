//! `slintui` — project scaffolding CLI
//!
//! Usage:
//!   slintui new <name>      Create a new slintui project in ./<name>/

use std::fs;
use std::path::Path;

/// Default window dimensions for scaffolded projects.
const DEFAULT_WIN_WIDTH: u32 = 1200;
const DEFAULT_WIN_HEIGHT: u32 = 800;

/// Scaffold template file names.
const CARGO_TOML: &str = "Cargo.toml";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run(&args) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        [_, cmd, name] if cmd.eq_ignore_ascii_case("new") => scaffold(name),
        _ => Err("Usage: slintui new <project-name>".into()),
    }
}

fn scaffold(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(name);
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

    eprintln!("Created project '{name}'");
    eprintln!();
    eprintln!("  cd {name} && cargo run");
    Ok(())
}

fn create_dirs(root: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(root.join("src"))?;
    fs::create_dir_all(root.join("views"))?;
    Ok(())
}

fn write_cargo_toml(root: &Path, name: &str) -> Result<(), std::io::Error> {
    let content = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{name}"
path = "src/main.rs"

[dependencies]
slint-ui-templates = {{ path = ".." }}
rhai = "1"
"#
    );
    fs::write(root.join(CARGO_TOML), content)
}

fn write_app_rhai(root: &Path, name: &str) -> Result<(), std::io::Error> {
    let display = capitalise(name);
    let content = format!(
        r#"// {display} — app configuration
set_nav(["home:Home:home", "list:List:list", "settings:Settings:settings"]);
set_window_size({DEFAULT_WIN_WIDTH}, {DEFAULT_WIN_HEIGHT});
set_bg_style("mica");
set_status("Ready");
"#
    );
    fs::write(root.join("app.rhai"), content)
}

fn write_view_rhai(root: &Path, view: &str, body: &str) -> Result<(), std::io::Error> {
    fs::write(root.join("views").join(format!("{view}.rhai")), body)
}

fn write_main_rs(root: &Path) -> Result<(), std::io::Error> {
    let content = r#"use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use slint_ui_templates::adapter::AppAdapter;
use slint_ui_templates::bindings::rhai::build_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // REASON: Rhai engine closures capture Rc clones for callback access
    let adapter = Rc::new(RefCell::new(AppAdapter::new()?));

    // Load per-view chrome configs from views/*.rhai
    adapter.borrow().load_view_configs(Path::new("views"))?;

    // Evaluate app.rhai — sets nav, window size, bg style, status
    let engine = build_engine(Rc::clone(&adapter));
    engine.eval::<()>(&std::fs::read_to_string("app.rhai")?)?;
    drop(engine);

    let adapter = Rc::try_unwrap(adapter)
        .map_err(|_| "adapter Rc still has multiple owners")?;
    adapter.into_inner().run()?;
    Ok(())
}
"#;
    fs::write(root.join("src").join("main.rs"), content)
}

fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None    => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
