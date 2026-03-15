fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    // ── Zero-literal scanners ──────────────────────────────────
    rustscanners::scan_project();
    slintscanners::scan_project();

    // ── Slint compilation ──────────────────────────────────────
    slint_build::compile("ui/lib.slint")?;

    // ── Model B: expose ui/ as Slint library path for downstream consumers ──
    // Consumers read DEP_SLINT_UI_TEMPLATES_SLINT_INCLUDE_PATH in their build.rs.
    // writeln! on stdout is the cargo build script metadata protocol — not logging.
    let manifest = std::env::var("CARGO_MANIFEST_DIR")?;
    writeln!(std::io::stdout(), "cargo:SLINT_INCLUDE_PATH={}/ui", manifest)?;

    Ok(())
}
