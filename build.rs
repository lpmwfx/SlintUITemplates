fn main() {
    // ── Zero-literal scanners ──────────────────────────────────
    rustscanners::scan_project();
    slintscanners::scan_project();

    // ── Slint compilation ──────────────────────────────────────
    slint_build::compile("ui/lib.slint").unwrap();
}
