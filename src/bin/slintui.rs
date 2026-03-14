//! `slintui` — project scaffolding CLI
//!
//! Usage:
//!   slintui new <name>      Create a new slintui project in ./<name>/

use std::fs;
use std::path::Path;

/// Default window dimensions for scaffolded projects.
const DEFAULT_WIN_WIDTH: u32 = 1200;
const DEFAULT_WIN_HEIGHT: u32 = 800;

/// Available slintui subcommands.
enum Command {
    New,
}

impl Command {
    fn from_str(s: &str) -> Option<Self> {
        if s.eq_ignore_ascii_case("new") {
            Some(Command::New)
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, cmd_str, name] => {
            match Command::from_str(cmd_str) {
                Some(Command::New) => {
                    if let Err(e) = scaffold(name) {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
                None => {
                    eprintln!("Usage: slintui new <project-name>");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Usage: slintui new <project-name>");
            std::process::exit(1);
        }
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

    println!("Created project '{}'", name);
    println!();
    println!("  cd {} && cargo run", name);
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
    fs::write(root.join("Cargo.toml"), content)
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
