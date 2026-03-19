/// Ratio-space min/max constraints for panel sizing.
pub mod constraints;
/// DSL v2 parser with named slot declarations.
pub mod dsl_v2;
/// Named-slot parse_named implementation.
pub mod named_parser;
/// DSL v1 parser that converts layout strings into a `PanelNode` tree.
pub mod parser;
/// Bi-directional ratio solver for interactive panel resizing.
pub mod ratio_solver;
/// Flattens a `PanelNode` tree into positioned `SolvedItem` rectangles.
pub mod solver;

pub use parser::parse;
pub use solver::{Solver, SolvedItem, ItemKind};
pub use ratio_solver::{Panel, drag, normalize, is_valid_sum};
pub use dsl_v2::{to_panels, NamedPanel, SplitDir};
pub use named_parser::parse_named;

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
