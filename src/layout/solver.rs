// Converts a PanelNode tree into a flat Vec<SolvedItem> with normalized
// x, y, w, h coordinates (0.0..1.0) and drag handle positions.
//
// Slint multiplies these by actual pixel dimensions at render time,
// making the layout fully responsive — no pixel math in .slint files.

use crate::layout::parser::{PanelNode, SplitDir};

const HANDLE_THICKNESS: f32 = 5.0;   // logical px — kept in sync with Spacing.handle-thickness

/// A positioned layout element with normalized coordinates ready for rendering.
#[derive(Debug, Clone)]
/// S ol ve di te m struct.
pub struct SolvedItem {
    /// Stable runtime id used to map solved items back to the UI model.
    pub id:    i32,
    /// Whether the item is a content panel or a drag handle.
    pub kind:  ItemKind,
    /// Human-readable label used by demo views and debug output.
    pub label: String,
    /// Normalized left position in the range `0.0..=1.0`.
    pub x: f32,
    /// Normalized top position in the range `0.0..=1.0`.
    pub y: f32,
    /// Normalized width in the range `0.0..=1.0`.
    pub w: f32,
    /// Normalized height in the range `0.0..=1.0`.
    pub h: f32,
}

/// Discriminates between content panels and drag handles in a solved layout.
#[derive(Debug, Clone, PartialEq)]
/// I te mk in d enum.
pub enum ItemKind { Panel, HandleH, HandleV }

impl ItemKind {
    /// Return a stable string identifier for this item kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemKind::Panel   => "panel",
            ItemKind::HandleH => "handle-h",
            ItemKind::HandleV => "handle-v",
        }
    }
}

/// Walks a `PanelNode` tree and emits flat `SolvedItem` rectangles with normalized coordinates.
pub struct Solver {
    next_id: i32,
    items:   Vec<SolvedItem>,
    /// Window pixel size — used to convert handle thickness to normalized fraction
    win_w: f32,
    win_h: f32,
}

impl Solver {
    /// Create a solver targeting the given window pixel dimensions.
    pub fn new(win_w: f32, win_h: f32) -> Self {
        Self { next_id: 0, items: Vec::new(), win_w, win_h }
    }

    /// Consume the solver, recursively visit the tree, and return all solved items.
    pub fn solve(mut self, root: &PanelNode) -> Vec<SolvedItem> {
        self.visit(root, 0.0, 0.0, 1.0, 1.0);
        self.items
    }

    fn alloc_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn visit(&mut self, node: &PanelNode, x: f32, y: f32, w: f32, h: f32) {
        match node {
            PanelNode::Leaf { .. } => self.emit_leaf(x, y, w, h),
            PanelNode::Split { dir, children, .. } => {
                self.emit_split(dir, children, x, y, w, h);
            }
        }
    }

    fn push_handle(&mut self, kind: ItemKind, x: f32, y: f32, w: f32, h: f32) {
        let id = self.alloc_id();
        self.items.push(SolvedItem { id, kind, label: String::new(), x, y, w, h });
    }

    fn emit_h_child(&mut self, child: &PanelNode, cursor: &mut f32, y: f32, child_size: f32, h: f32, is_last: bool, handle_frac_w: f32) {
        self.visit(child, *cursor, y, child_size, h);
        *cursor += child_size;
        if !is_last {
            self.push_handle(ItemKind::HandleH, *cursor, y, handle_frac_w, h);
            *cursor += handle_frac_w;
        }
    }

    fn emit_v_child(&mut self, child: &PanelNode, cursor: &mut f32, x: f32, child_size: f32, w: f32, is_last: bool, handle_frac_h: f32) {
        self.visit(child, x, *cursor, w, child_size);
        *cursor += child_size;
        if !is_last {
            self.push_handle(ItemKind::HandleV, x, *cursor, w, handle_frac_h);
            *cursor += handle_frac_h;
        }
    }

    fn emit_leaf(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let id = self.alloc_id();
        self.items.push(SolvedItem {
            id, kind: ItemKind::Panel,
            label: format!("Panel {}", id),
            x, y, w, h,
        });
    }

    // REASON: single-caller by design — split direction handler
    fn emit_split(&mut self, dir: &SplitDir, children: &[PanelNode], x: f32, y: f32, w: f32, h: f32) {
        let total_ratio: f32 = children.iter().map(|c| c.ratio()).sum();
        let handle_frac_w = HANDLE_THICKNESS / self.win_w;
        let handle_frac_h = HANDLE_THICKNESS / self.win_h;
        let n_handles = (children.len() - 1) as f32;

        let handle_total = match dir {
            SplitDir::H => handle_frac_w * n_handles,
            SplitDir::V => handle_frac_h * n_handles,
        };
        let content_space = match dir {
            SplitDir::H => w - handle_total,
            SplitDir::V => h - handle_total,
        };

        let mut cursor = match dir {
            SplitDir::H => x,
            SplitDir::V => y,
        };

        for (i, child) in children.iter().enumerate() {
            let frac = child.ratio() / total_ratio;
            let child_size = content_space * frac;
            let is_last = i == children.len() - 1;

            match dir {
                SplitDir::H => self.emit_h_child(child, &mut cursor, y, child_size, h, is_last, handle_frac_w),
                SplitDir::V => self.emit_v_child(child, &mut cursor, x, child_size, w, is_last, handle_frac_h),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::parser::parse;

    #[test]
    fn test_single_panel_fills_container() {
        let tree = parse("1");
        let items = Solver::new(1280.0, 800.0).solve(&tree);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].kind, ItemKind::Panel);
        assert!((items[0].w - 1.0).abs() < 0.001);
        assert!((items[0].h - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_three_columns_have_two_handles() {
        let tree = parse("1:2:1");
        let items = Solver::new(1280.0, 800.0).solve(&tree);
        let handles: Vec<_> = items.iter().filter(|i| i.kind == ItemKind::HandleH).collect();
        assert_eq!(handles.len(), 2);
    }

    #[test]
    fn test_widths_sum_to_one() {
        let tree = parse("1:2:1");
        let items = Solver::new(1280.0, 800.0).solve(&tree);
        let total: f32 = items.iter().map(|i| i.w).sum();
        assert!((total - 1.0).abs() < 0.01);
    }
}
