//! `slintui` — project scaffolding CLI
//!
//! Usage:
//!   slintui new <name>      Create a new slintui project in ./<name>/

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run(&args) {
        tracing::error!("{e}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        [_, cmd, name] if cmd.eq_ignore_ascii_case("new") => {
            slint_ui_templates::gateway::scaffold::scaffold(name)?;
            tracing::info!("Created project '{name}'");
            tracing::info!("  cd {name} && cargo run");
            Ok(())
        }
        _ => Err("Usage: slintui new <project-name>".into()),
    }
}
