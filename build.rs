fn main() {
    // ── Zero-literal scanners ──────────────────────────────────
    rustscanners::scan_project();
    slintscanners::scan_project();

    // ── Slint compilation ──────────────────────────────────────
    slint_build::compile("ui/lib.slint")
        .expect("Slint compilation failed — check ui/lib.slint syntax");
}
