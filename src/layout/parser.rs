// Parses the panel layout DSL into a PanelNode tree.
//
// DSL syntax:
//   ":"  = HSplit — columns side by side (vertical drag handle between)
//   "/"  = VSplit — rows stacked (horizontal drag handle between)
//   numbers = size ratios
//
// Examples:
//   "1:2:1"        → three columns, ratios 1:2:1
//   "1/1"          → two rows, equal height
//   "1:2/1:1:1"    → left(1) | center[top(2)/bottom(1:1)] | right(1)

#[derive(Debug, Clone)]
pub enum SplitDir { H, V }

#[derive(Debug, Clone)]
pub enum PanelNode {
    Leaf { ratio: f32 },
    Split { dir: SplitDir, ratio: f32, children: Vec<PanelNode> },
}

impl PanelNode {
    pub fn ratio(&self) -> f32 {
        match self {
            PanelNode::Leaf { ratio } => *ratio,
            PanelNode::Split { ratio, .. } => *ratio,
        }
    }
}

/// Parse a DSL string into a PanelNode tree.
/// Top-level is always an HSplit if ":" is present, VSplit if only "/" is present.
pub fn parse(dsl: &str) -> PanelNode {
    parse_hsplit(dsl.trim(), 1.0)
}

fn parse_hsplit(s: &str, ratio: f32) -> PanelNode {
    let parts = split_top_level(s, ':');
    if parts.len() == 1 {
        return parse_vsplit(s, ratio);
    }
    let children = parts.iter().map(|p| parse_vsplit(p.trim(), 1.0)).collect::<Vec<_>>();
    PanelNode::Split { dir: SplitDir::H, ratio, children }
}

fn parse_vsplit(s: &str, ratio: f32) -> PanelNode {
    let parts = split_top_level(s, '/');
    if parts.len() == 1 {
        return parse_leaf(s, ratio);
    }
    let children = parts.iter().map(|p| parse_hsplit(p.trim(), 1.0)).collect::<Vec<_>>();
    PanelNode::Split { dir: SplitDir::V, ratio, children }
}

fn parse_leaf(s: &str, _ratio: f32) -> PanelNode {
    let ratio = s.trim().parse::<f32>().unwrap_or(1.0);
    PanelNode::Leaf { ratio }
}

/// Split on delimiter only at the top level (not inside parens — future use).
fn split_top_level(s: &str, delim: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    for (i, c) in s.char_indices() {
        if c == delim {
            parts.push(&s[start..i]);
            start = i + 1;
        }
    }
    parts.push(&s[start..]);
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_leaf() {
        let n = parse("1");
        assert!(matches!(n, PanelNode::Leaf { ratio } if ratio == 1.0));
    }

    #[test]
    fn test_three_columns() {
        let n = parse("1:2:1");
        assert!(matches!(n, PanelNode::Split { dir: SplitDir::H, .. }));
        if let PanelNode::Split { children, .. } = n {
            assert_eq!(children.len(), 3);
        }
    }

    #[test]
    fn test_nested() {
        // "1:2/1:1:1" → HSplit[Leaf(1), VSplit[Leaf(2), HSplit[Leaf(1),Leaf(1)]], Leaf(1)]
        let n = parse("1:2/1:1:1");
        assert!(matches!(n, PanelNode::Split { dir: SplitDir::H, .. }));
        if let PanelNode::Split { children, .. } = n {
            assert_eq!(children.len(), 3);
            assert!(matches!(children[1], PanelNode::Split { dir: SplitDir::V, .. }));
        }
    }
}
