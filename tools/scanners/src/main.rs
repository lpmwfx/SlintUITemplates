//! scanners — runs AI doc generation + zero-literal scanners.
//! Usage: cargo run -p scanners

fn main() {
    rustdocumenter::document_project();
    rustscanners::scan_project();
    slintscanners::scan_project();
}
