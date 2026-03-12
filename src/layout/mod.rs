pub mod constraints;
pub mod dsl_v2;
pub mod parser;
pub mod ratio_solver;
pub mod solver;

pub use parser::parse;
pub use solver::{Solver, SolvedItem, ItemKind};
pub use ratio_solver::{Panel, drag, normalize, check_sum};
pub use dsl_v2::{parse_named, to_panels, NamedPanel};

/// Parse a DSL string and solve layout for given window dimensions.
/// Returns a flat Vec<SolvedItem> ready to push to Slint as [PanelItem].
pub fn build(dsl: &str, win_w: f32, win_h: f32) -> Vec<SolvedItem> {
    let tree = parse(dsl);
    Solver::new(win_w, win_h).solve(&tree)
}

/// Parse a DSL v2 named-slot string into resizable Panels.
/// Use `drag()` on the returned panels to apply interactive resize.
pub fn build_v2(dsl: &str) -> Vec<Panel> {
    let named = parse_named(dsl);
    to_panels(&named)
}
