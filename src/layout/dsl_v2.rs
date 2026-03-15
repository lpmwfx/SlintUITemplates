/// DSL v2 — named slot layout declarations.
///
/// Named slots replace bare numbers. Explicit ratios optional.
///
/// Syntax:
///   "sidebar:content"               → two columns, equal ratios
///   "sidebar(0.2):content"          → sidebar=20%, content=80%
///   "sidebar:content:inspector"     → three equal columns
///   "sidebar(0.2):content:inspector(0.25)" → explicit ratios, remainder to content
///   "header/sidebar:content/footer" → rows with nested column split
///
/// Returns a Vec<NamedPanel> in left-to-right / top-to-bottom order.

use crate::layout::ratio_solver::Panel;

/// A parsed named panel with resolved ratio.
#[derive(Debug, Clone)]
/// N am ed pa ne l struct.
pub struct NamedPanel {
    /// N am e.
    pub name:  String,
    /// R at io.
    pub ratio: f32,
    /// D ir.
    pub dir:   SplitDir,
}

/// Direction of a named-slot split: horizontal (columns) or vertical (rows).
#[derive(Debug, Clone, PartialEq)]
/// S pl it di r enum.
pub enum SplitDir { H, V }

/// Convert named panels to ratio_solver Panels (ready for drag()).
pub fn to_panels(named: &[NamedPanel]) -> Vec<Panel> {
    named.iter().map(|n| Panel::new(&n.name, n.ratio)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::named_parser::parse_named;

    #[test]
    fn two_equal_slots() {
        let panels = parse_named("sidebar:content");
        assert_eq!(panels.len(), 2);
        assert_eq!(panels[0].name, "sidebar");
        assert_eq!(panels[1].name, "content");
        assert!((panels[0].ratio - 0.5).abs() < 0.001);
        assert!((panels[1].ratio - 0.5).abs() < 0.001);
    }

    #[test]
    fn explicit_ratio_sidebar() {
        let panels = parse_named("sidebar(0.2):content");
        assert!((panels[0].ratio - 0.2).abs() < 0.001, "sidebar ratio");
        assert!((panels[1].ratio - 0.8).abs() < 0.001, "content ratio");
    }

    #[test]
    fn three_equal_slots() {
        let panels = parse_named("a:b:c");
        assert_eq!(panels.len(), 3);
        for p in &panels {
            assert!((p.ratio - 1.0 / 3.0).abs() < 0.01);
        }
    }

    #[test]
    fn explicit_and_implicit_ratios() {
        let panels = parse_named("sidebar(0.2):content:inspector(0.25)");
        assert_eq!(panels.len(), 3);
        assert!((panels[0].ratio - 0.2 ).abs() < 0.001, "sidebar");
        assert!((panels[1].ratio - 0.55).abs() < 0.001, "content");
        assert!((panels[2].ratio - 0.25).abs() < 0.001, "inspector");
    }

    #[test]
    fn vsplit_rows() {
        let panels = parse_named("header/content/footer");
        assert_eq!(panels.len(), 3);
        assert_eq!(panels[0].dir, SplitDir::V);
        for p in &panels {
            assert!((p.ratio - 1.0 / 3.0).abs() < 0.01);
        }
    }
}
