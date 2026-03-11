pub mod parser;
pub mod solver;

pub use parser::parse;
pub use solver::{Solver, SolvedItem, ItemKind};

/// Parse a DSL string and solve layout for given window dimensions.
/// Returns a flat Vec<SolvedItem> ready to push to Slint as [PanelItem].
pub fn build(dsl: &str, win_w: f32, win_h: f32) -> Vec<SolvedItem> {
    let tree = parse(dsl);
    Solver::new(win_w, win_h).solve(&tree)
}
