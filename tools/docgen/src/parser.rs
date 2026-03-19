use crate::{PropMode, Prop, Cb, Field, Item, COMMENT_PREFIX_LEN};

/// Split a line into code and trailing `//` comment.
pub(crate) fn split_comment(s: &str) -> (&str, &str) {
    if let Some(p) = s.find("//") { (s[..p].trim_end(), s[p + COMMENT_PREFIX_LEN..].trim()) }
    else                          { (s, "") }
}

/// Parse a Slint property declaration line.
pub(crate) fn parse_prop(line: &str) -> Option<Prop> {
    let t    = line.trim();
    let mode = PropMode::detect(t)?;
    let ty_s = t.find('<')? + 1;
    let ty_e = t.find('>')?;
    let ty   = t[ty_s..ty_e].to_string();
    let rest = t[ty_e + 1..].trim();
    let (body, desc) = split_comment(rest);
    let body = body.trim_end_matches(';').trim();
    let (name, default) = body.split_once(':')
        .map(|(n, d)| (n.trim().to_string(), d.trim().to_string()))
        .unwrap_or_else(|| (body.to_string(), String::new()));
    Some(Prop { mode, ty, name, default, desc: desc.replace('|', "\\|") })
}

/// Parse a Slint callback declaration line.
pub(crate) fn parse_cb(line: &str) -> Option<Cb> {
    let t = line.trim();
    if !t.starts_with("callback ") { return None; }
    let (sig, desc) = split_comment(&t["callback ".len()..]);
    let sig  = sig.trim_end_matches(';').trim();
    let (name, args) = sig.split_once('(')
        .map(|(n, a)| (n.trim().to_string(), a.trim_end_matches(')').trim().to_string()))
        .unwrap_or_else(|| (sig.to_string(), String::new()));
    Some(Cb { name, args, desc: desc.replace('|', "\\|") })
}

fn first_word_after<'a>(s: &'a str, prefix: &str) -> &'a str {
    s[s.find(prefix).unwrap_or(0) + prefix.len()..].split_whitespace().next().unwrap_or("")
}

/// Collect top-level properties and callbacks from a component body.
fn collect_component_members(lines: &[&str], start: usize) -> (Vec<Prop>, Vec<Cb>, usize) {
    let (mut props, mut cbs, mut depth) = (vec![], vec![], 0i32);
    let mut i = start;
    while i < lines.len() {
        let lt = lines[i].trim();
        depth += lt.chars().filter(|&c| c == '{').count() as i32;
        depth -= lt.chars().filter(|&c| c == '}').count() as i32;
        if depth < 0 { break; }
        if depth == 0 {
            if let Some(p) = parse_prop(lines[i]) { props.push(p); }
            if let Some(c) = parse_cb(lines[i])   { cbs.push(c); }
        }
        i += 1;
    }
    (props, cbs, i)
}

/// Collect fields from a struct body.
fn collect_struct_fields(lines: &[&str], start: usize) -> (Vec<Field>, usize) {
    let mut fields = Vec::new();
    let mut i = start;
    while i < lines.len() {
        let lt = lines[i].trim();
        if lt == "}" { break; }
        if let Some((n, rest)) = lt.split_once(':') {
            let (ty_raw, _) = split_comment(rest);
            fields.push(Field {
                name: n.trim().to_string(),
                ty:   ty_raw.trim().trim_end_matches(',').to_string(),
            });
        }
        i += 1;
    }
    (fields, i)
}

/// Parse all exported components and structs from a Slint source file.
pub(crate) fn parse_file(src: &str, filename: &str) -> Vec<Item> {
    let lines: Vec<&str> = src.lines().collect();
    let mut items = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim();
        if t.starts_with("export component ") {
            let name = first_word_after(t, "component ").to_string();
            let (props, cbs, end) = collect_component_members(&lines, i + 1);
            i = end;
            items.push(Item::Comp { name, file: filename.into(), props, cbs });
        } else if t.starts_with("export struct ") {
            let name = first_word_after(t, "struct ").trim_end_matches('{').trim().to_string();
            let (fields, end) = collect_struct_fields(&lines, i + 1);
            i = end;
            items.push(Item::Struct { name, file: filename.into(), fields });
        }
        i += 1;
    }
    items
}
