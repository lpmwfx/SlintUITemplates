/// Bi-directional ratio solver for resizable panel layouts.
///
/// Given N panels with normalized ratios that sum to 1.0, dragging the handle
/// at `handle_idx` transfers ratio from one neighbor to the other — zero-sum,
/// both sides update atomically.
///
/// This is the Rust owner of all layout math. Slint is pure render.

use super::constraints::Constraint;

/// Tolerance for floating-point ratio sum checks.
const RATIO_EPSILON: f32 = 0.001;

/// A resizable panel with a normalized ratio and min/max constraints.
#[derive(Debug, Clone)]
pub struct Panel {
    pub id:    String,
    pub ratio: f32,
    pub constraint: Constraint,
}

impl Panel {
    pub fn new(id: impl Into<String>, ratio: f32) -> Self {
        Self { id: id.into(), ratio, constraint: Constraint::default() }
    }

    pub fn with_constraint(mut self, min: f32, max: f32) -> Self {
        self.constraint = Constraint::new(min, max);
        self
    }
}

/// Apply a drag delta to the handle between panels[handle_idx] and panels[handle_idx + 1].
///
/// `delta` is a signed ratio fraction: positive = handle moves right/down.
/// Both neighbors update; sum of all ratios stays at 1.0.
///
/// Returns the actual transfer applied (may be smaller than `delta` due to constraints).
pub fn drag(panels: &mut [Panel], handle_idx: usize, delta: f32) -> f32 {
    assert!(handle_idx + 1 < panels.len(), "handle_idx out of bounds");

    let left  = handle_idx;
    let right = handle_idx + 1;

    // Clamp transfer so neither neighbor violates its constraint
    let transfer = delta.clamp(
        panels[left].constraint.min  - panels[left].ratio,    // left can't go below min
        panels[right].ratio - panels[right].constraint.min,   // right can't go below min
    );

    // Further clamp: left can't exceed max
    let transfer = transfer.min(panels[left].constraint.max - panels[left].ratio);
    // Right can't exceed max
    let transfer = transfer.max(-(panels[right].constraint.max - panels[right].ratio));

    panels[left].ratio  += transfer;
    panels[right].ratio -= transfer;

    transfer
}

/// Normalize panel ratios so they sum to exactly 1.0.
/// Call this after constructing panels to ensure invariant holds.
pub fn normalize(panels: &mut [Panel]) {
    let total: f32 = panels.iter().map(|p| p.ratio).sum();
    if total > 0.0 {
        for p in panels.iter_mut() {
            p.ratio /= total;
        }
    }
}

/// Verify that panel ratios sum to approximately 1.0.
pub fn check_sum(panels: &[Panel]) -> bool {
    let total: f32 = panels.iter().map(|p| p.ratio).sum();
    (total - 1.0).abs() < RATIO_EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_panels() -> Vec<Panel> {
        vec![
            Panel::new("left",  0.5),
            Panel::new("right", 0.5),
        ]
    }

    fn three_panels() -> Vec<Panel> {
        vec![
            Panel::new("a", 0.33),
            Panel::new("b", 0.34),
            Panel::new("c", 0.33),
        ]
    }

    #[test]
    fn drag_transfers_ratio() {
        let mut panels = two_panels();
        drag(&mut panels, 0, 0.1);
        assert!((panels[0].ratio - 0.6).abs() < 0.001);
        assert!((panels[1].ratio - 0.4).abs() < 0.001);
    }

    #[test]
    fn sum_preserved_after_drag() {
        let mut panels = three_panels();
        drag(&mut panels, 0, 0.1);
        assert!(check_sum(&panels), "sum not 1.0 after drag");
        drag(&mut panels, 1, -0.05);
        assert!(check_sum(&panels), "sum not 1.0 after second drag");
    }

    #[test]
    fn drag_respects_min_constraint() {
        let mut panels = vec![
            Panel::new("left",  0.1).with_constraint(0.1, 0.9),
            Panel::new("right", 0.9).with_constraint(0.1, 0.9),
        ];
        // Try to drag left so it goes below min — should be clamped
        drag(&mut panels, 0, -0.2);
        assert!(panels[0].ratio >= 0.1 - 0.001, "left went below min");
    }

    #[test]
    fn drag_respects_max_constraint() {
        let mut panels = vec![
            Panel::new("left",  0.5).with_constraint(0.1, 0.6),
            Panel::new("right", 0.5).with_constraint(0.1, 0.9),
        ];
        // Try to drag left beyond max
        drag(&mut panels, 0, 0.3);
        assert!(panels[0].ratio <= 0.6 + 0.001, "left exceeded max");
    }

    #[test]
    fn normalize_makes_sum_one() {
        let mut panels = vec![
            Panel::new("a", 1.0),
            Panel::new("b", 2.0),
            Panel::new("c", 1.0),
        ];
        normalize(&mut panels);
        assert!(check_sum(&panels));
        assert!((panels[1].ratio - 0.5).abs() < 0.001);
    }
}
