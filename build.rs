fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── DEP_ env-var mechanism ─────────────────────────────────────────────
    // Cargo `links = "slint-ui-templates"` in [package] + the metadata line
    // below combine to expose `DEP_SLINT_UI_TEMPLATES_SLINT_INCLUDE_PATH`
    // to every crate that depends on this library.
    //
    // Consumer build.rs reads:
    //   std::env::var("DEP_SLINT_UI_TEMPLATES_SLINT_INCLUDE_PATH")
    // and passes the path to `slint_build::CompilerConfiguration::with_include_paths()`.
    //
    // Emit include path for consuming crates (must be before compile).
    let ui_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui");
    println!("cargo:SLINT_INCLUDE_PATH={}", ui_path.display());

    // ── Slint compilation ──────────────────────────────────────
    // lib-dev.slint includes viewer/docs-app for examples.
    // Published crate uses lib.slint (viewer/demo excluded via include[]).
    let slint_entry = if std::path::Path::new("ui/lib-dev.slint").exists() {
        "ui/lib-dev.slint"
    } else {
        "ui/lib.slint"
    };
    slint_build::compile(slint_entry)?;

    Ok(())
}
