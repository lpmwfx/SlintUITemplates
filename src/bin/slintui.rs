//! `slintui` — project scaffolding CLI
//!
//! Usage:
//!   slintui new <name>      Create a new slintui project in ./<name>/

use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, cmd, name] if cmd == "new" => scaffold(name),
        _ => {
            eprintln!("Usage: slintui new <project-name>");
            std::process::exit(1);
        }
    }
}

fn scaffold(name: &str) {
    let root = Path::new(name);
    if root.exists() {
        eprintln!("Error: directory '{}' already exists", name);
        std::process::exit(1);
    }

    create_dirs(root);
    write_cargo_toml(root, name);
    write_app_rhai(root, name);
    write_view_rhai(root, "home",     "view_status(\"Welcome\");");
    write_view_rhai(root, "list",     "view_status(\"Browse\");\nview_toolbar([\"add:add:New\"]);");
    write_view_rhai(root, "settings", "view_status(\"Configure\");");
    write_main_rs(root);

    println!("Created project '{}'", name);
    println!();
    println!("  cd {} && cargo run", name);
}

fn create_dirs(root: &Path) {
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("views")).unwrap();
}

fn write_cargo_toml(root: &Path, name: &str) {
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
    fs::write(root.join("Cargo.toml"), content).unwrap();
}

fn write_app_rhai(root: &Path, name: &str) {
    let display = capitalise(name);
    let content = format!(
        r#"// {display} — app configuration
set_nav(["home:Home:home", "list:List:list", "settings:Settings:settings"]);
set_window_size(1200, 800);
set_bg_style("mica");
set_status("Ready");
"#
    );
    fs::write(root.join("app.rhai"), content).unwrap();
}

fn write_view_rhai(root: &Path, view: &str, body: &str) {
    fs::write(root.join("views").join(format!("{view}.rhai")), body).unwrap();
}

fn write_main_rs(root: &Path) {
    let content = r#"use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use slint_ui_templates::adapter::AppAdapter;
use slint_ui_templates::bindings::rhai::build_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter = Rc::new(RefCell::new(AppAdapter::new()?));

    // Load per-view chrome configs from views/*.rhai
    adapter.borrow().load_view_configs(Path::new("views"))?;

    // Evaluate app.rhai — sets nav, window size, bg style, status
    let engine = build_engine(Rc::clone(&adapter));
    engine.eval::<()>(&std::fs::read_to_string("app.rhai")?)?;
    drop(engine);

    Rc::try_unwrap(adapter).unwrap().into_inner().run()?;
    Ok(())
}
"#;
    fs::write(root.join("src").join("main.rs"), content).unwrap();
}

fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None    => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
