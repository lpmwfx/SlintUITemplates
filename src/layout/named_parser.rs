use crate::layout::ratio_solver::{Panel, normalize};
use crate::layout::{NamedPanel, SplitDir};

/// Parse DSL v2 string into a flat list of named panels.
/// Horizontal split at top level; vertical split when "/" separates rows.
/// N am ed p ar se r.
pub fn parse_named(dsl: &str) -> Vec<NamedPanel> {
    let dsl = dsl.trim();
    let top_parts: Vec<&str> = split_level(dsl, '/');
    if top_parts.len() > 1 {
        let mut panels = vec![];
        for part in &top_parts {
            panels.extend(parse_row_slots(part));
        }
        normalize_vsplit(&mut panels);
        return panels;
    }

    let parts: Vec<&str> = split_level(dsl, ':');
    let mut panels: Vec<NamedPanel> = parts.iter()
        .map(|s| parse_slot(s, SplitDir::H))
        .collect();

    let explicit_sum: f32 = panels.iter().filter(|p| p.ratio > 0.0).map(|p| p.ratio).sum();
    let unnamed_count = panels.iter().filter(|p| p.ratio == 0.0).count();
    let remainder = (1.0 - explicit_sum).max(0.0);
    let share = if unnamed_count > 0 { remainder / unnamed_count as f32 } else { 0.0 };
    for p in panels.iter_mut() {
        if p.ratio == 0.0 { p.ratio = share; }
    }
    panels
}

fn normalize_vsplit(panels: &mut Vec<NamedPanel>) {
    let mut ratio_panels: Vec<Panel> = panels.iter()
        .map(|p| Panel::new(&p.name, if p.ratio == 0.0 { 1.0 } else { p.ratio }))
        .collect();
    normalize(&mut ratio_panels);
    for (i, p) in panels.iter_mut().enumerate() {
        p.ratio = ratio_panels[i].ratio;
    }
}

fn parse_row_slots(part: &str) -> Vec<NamedPanel> {
    let sub = split_level(part, ':');
    if sub.len() > 1 {
        sub.iter().map(|slot| parse_slot(slot, SplitDir::H)).collect()
    } else {
        vec![parse_slot(part, SplitDir::V)]
    }
}

fn parse_slot(s: &str, dir: SplitDir) -> NamedPanel {
    let s = s.trim();
    if let Some(paren) = s.find('(') {
        let name      = s[..paren].trim().to_string();
        let ratio_str = s[paren + 1..].trim_end_matches(')').trim();
        let ratio     = ratio_str.parse::<f32>().unwrap_or(0.0);
        NamedPanel { name, ratio, dir }
    } else {
        NamedPanel { name: s.to_string(), ratio: 0.0, dir }
    }
}

fn split_level(s: &str, delim: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            c if c == delim && depth == 0 => { parts.push(&s[start..i]); start = i + 1; }
            _ => {}
        }
    }
    parts.push(&s[start..]);
    parts
}
