//! docgen — generates docs/components.md Shared-Components section from ui/widgets/*.slint.
//! Usage: cargo run --bin docgen

use std::{fs, io::Write, path::Path};

#[derive(PartialEq)]
enum PropMode { In, InOut, Out }

impl PropMode {
    fn detect(t: &str) -> Option<Self> {
        if      t.starts_with("in-out property") { Some(Self::InOut) }
        else if t.starts_with("in property")     { Some(Self::In)    }
        else if t.starts_with("out property")    { Some(Self::Out)   }
        else                                     { None              }
    }
    fn label(&self) -> &'static str {
        match self { Self::In => "in", Self::InOut => "in-out", Self::Out => "out" }
    }
}

struct Prop { mode: PropMode, ty: String, name: String, default: String, desc: String }
struct Cb   { name: String, args: String, desc: String }
struct Field { name: String, ty: String }

enum Item {
    Struct { name: String, #[allow(dead_code)] file: String, fields: Vec<Field> },
    Comp   { name: String, file: String, props: Vec<Prop>, cbs: Vec<Cb> },
}

fn split_comment(s: &str) -> (&str, &str) {
    if let Some(p) = s.find("//") { (s[..p].trim_end(), s[p+2..].trim()) }
    else                          { (s, "") }
}

fn parse_prop(line: &str) -> Option<Prop> {
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

fn parse_cb(line: &str) -> Option<Cb> {
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

const SLINT_EXT: &str = "slint";

fn parse_file(src: &str, filename: &str) -> Vec<Item> {
    let lines: Vec<&str> = src.lines().collect();
    let mut items = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim();
        if t.starts_with("export component ") {
            let name = first_word_after(t, "component ").to_string();
            let (mut props, mut cbs, mut depth) = (vec![], vec![], 0i32);
            i += 1;
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
            items.push(Item::Comp { name, file: filename.into(), props, cbs });
        } else if t.starts_with("export struct ") {
            let name = first_word_after(t, "struct ").trim_end_matches('{').trim().to_string();
            let mut fields = Vec::new();
            i += 1;
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
            items.push(Item::Struct { name, file: filename.into(), fields });
        }
        i += 1;
    }
    items
}

fn render(all: &[Item]) -> String {
    let mut out = String::from("\n## Shared Components\n\n");
    for item in all {
        match item {
            Item::Struct { name, fields, .. } => {
                out += &format!("#### `{}` struct\n\n| Field | Type |\n|-------|------|\n", name);
                for f in fields { out += &format!("| `{}` | `{}` |\n", f.name, f.ty); }
                out += "\n";
            }
            Item::Comp { name, file, props, cbs } => {
                out += &format!("### {} (`ui/widgets/{}`)\n\n", name, file);
                if !props.is_empty() {
                    out += "| Property | Type | Default | Description |\n\
                            |----------|------|---------|-------------|\n";
                    for p in props {
                        let tag = if p.mode != PropMode::In { format!(" *({})*", p.mode.label()) }
                                  else { String::new() };
                        out += &format!("| `{}`{} | `{}` | `{}` | {} |\n",
                            p.name, tag, p.ty, p.default, p.desc);
                    }
                    out += "\n";
                }
                if !cbs.is_empty() {
                    out += "| Callback | Signature | Description |\n\
                            |----------|-----------|-------------|\n";
                    for cb in cbs {
                        let sig = if cb.args.is_empty() { format!("`{}()`", cb.name) }
                                  else { format!("`{}({})`", cb.name, cb.args) };
                        out += &format!("| {} | | {} |\n", sig, cb.desc);
                    }
                    out += "\n";
                }
                out += "---\n\n";
            }
        }
    }
    out
}

const START: &str = "<!-- AUTOGEN:shared START -->";
const END:   &str = "<!-- AUTOGEN:shared END -->";

fn update(existing: &str, generated: &str) -> String {
    if let (Some(s), Some(e)) = (existing.find(START), existing.find(END)) {
        format!("{}{}\n{}{}", &existing[..s], START, generated, &existing[e..])
    } else {
        format!("{}\n{}\n{}\n{}\n", existing.trim_end(), START, generated.trim_end(), END)
    }
}

type Err = Box<dyn std::error::Error>;

fn scan(shared: &Path) -> Result<Vec<Item>, Err> {
    let mut entries: Vec<_> = fs::read_dir(shared)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == SLINT_EXT))
        .collect();
    entries.sort_by_key(|e| e.file_name());
    let mut all = Vec::new();
    for entry in &entries {
        let name = entry.file_name().to_string_lossy().to_string();
        all.extend(parse_file(&fs::read_to_string(entry.path())?, &name));
    }
    Ok(all)
}

fn generate(all: &[Item], docs: &Path) -> Result<(usize, usize), Err> {
    let existing = if docs.exists() { fs::read_to_string(docs)? } else { String::new() };
    fs::write(docs, update(&existing, &render(all)))?;
    Ok((
        all.iter().filter(|i| matches!(i, Item::Comp   { .. })).count(),
        all.iter().filter(|i| matches!(i, Item::Struct { .. })).count(),
    ))
}

fn run() -> Result<(), Err> {
    let all = scan(Path::new("ui/shared"))?;
    let (c, s) = generate(&all, Path::new("docs/components.md"))?;
    writeln!(std::io::stdout(), "docgen: {c} components + {s} structs → docs/components.md")?;
    Ok(())
}

fn main() -> Result<(), Err> { run() }
