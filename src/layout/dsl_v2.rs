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

use super::ratio_solver::{Panel, normalize};

/// A parsed named panel with resolved ratio.
#[derive(Debug, Clone)]
pub struct NamedPanel {
    pub name:  String,
    pub ratio: f32,
    pub dir:   SplitDir,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SplitDir { H, V }

/// Parse DSL v2 string into a flat list of named panels (horizontal split only at top level).
/// Nested VSplit ("header/content/footer") returns panels in row order.
pub fn parse_named(dsl: &str) -> Vec<NamedPanel> {
    let dsl = dsl.trim();

    // Top-level: check for "/" (VSplit rows) or ":" (HSplit columns)
    let top_parts: Vec<&str> = split_level(dsl, '/');
    if top_parts.len() > 1 {
        // VSplit at top level
        let mut panels: Vec<NamedPanel> = Vec::new();
        for part in &top_parts {
            let sub = split_level(part, ':');
            if sub.len() > 1 {
                // Nested HSplit within a row
                for slot in &sub {
                    panels.push(parse_slot(slot, SplitDir::H));
                }
            } else {
                panels.push(parse_slot(part, SplitDir::V));
            }
        }
        // Panels with no explicit ratio get ratio=1.0 (equal share before normalize)
        let mut ratio_panels: Vec<Panel> = panels.iter()
            .map(|p| Panel::new(&p.name, if p.ratio == 0.0 { 1.0 } else { p.ratio }))
            .collect();
        normalize(&mut ratio_panels);
        for (i, p) in panels.iter_mut().enumerate() {
            p.ratio = ratio_panels[i].ratio;
        }
        return panels;
    }

    // HSplit at top level
    let parts: Vec<&str> = split_level(dsl, ':');
    let mut panels: Vec<NamedPanel> = parts.iter()
        .map(|s| parse_slot(s, SplitDir::H))
        .collect();

    // Resolve ratios: explicit ratios kept, unnamed get equal share of remainder
    let explicit_sum: f32 = panels.iter()
        .filter(|p| p.ratio > 0.0)
        .map(|p| p.ratio)
        .sum();
    let unnamed_count = panels.iter().filter(|p| p.ratio == 0.0).count();
    let remainder = (1.0 - explicit_sum).max(0.0);
    let share = if unnamed_count > 0 { remainder / unnamed_count as f32 } else { 0.0 };

    for p in panels.iter_mut() {
        if p.ratio == 0.0 {
            p.ratio = share;
        }
    }

    panels
}

/// Convert named panels to ratio_solver Panels (ready for drag()).
pub fn to_panels(named: &[NamedPanel]) -> Vec<Panel> {
    named.iter().map(|n| Panel::new(&n.name, n.ratio)).collect()
}

// ── internal helpers ──────────────────────────────────────────────────────

/// Parse a single slot entry like "sidebar" or "sidebar(0.2)".
fn parse_slot(s: &str, dir: SplitDir) -> NamedPanel {
    let s = s.trim();
    if let Some(paren) = s.find('(') {
        let name  = s[..paren].trim().to_string();
        let ratio_str = s[paren + 1..].trim_end_matches(')').trim();
        let ratio = ratio_str.parse::<f32>().unwrap_or(0.0);
        NamedPanel { name, ratio, dir }
    } else {
        NamedPanel { name: s.to_string(), ratio: 0.0, dir }
    }
}

/// Split string on delimiter only at the top level (no paren nesting implemented yet).
fn split_level(s: &str, delim: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            c if c == delim && depth == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&s[start..]);
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // inspector gets 0.25, sidebar gets 0.2, content gets the rest 0.55
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
