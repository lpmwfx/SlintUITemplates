use std::{fs, path::Path};
use crate::{Item, PropMode, SLINT_EXT, Err};
use crate::parser::parse_file;

/// Render parsed items to a Markdown string.
pub(crate) fn render(all: &[Item]) -> String {
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

/// Splice generated content between AUTOGEN markers in existing text.
pub(crate) fn update(existing: &str, generated: &str) -> String {
    if let (Some(s), Some(e)) = (existing.find(START), existing.find(END)) {
        format!("{}{}\n{}{}", &existing[..s], START, generated, &existing[e..])
    } else {
        format!("{}\n{}\n{}\n{}\n", existing.trim_end(), START, generated.trim_end(), END)
    }
}

/// Scan a directory for `.slint` files and parse all items.
pub(crate) fn scan(shared: &Path) -> Result<Vec<Item>, Err> {
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

/// Write rendered Markdown to `docs` path. Returns (component_count, struct_count).
pub(crate) fn generate(all: &[Item], docs: &Path) -> Result<(usize, usize), Err> {
    let existing = if docs.exists() { fs::read_to_string(docs)? } else { String::new() };
    fs::write(docs, update(&existing, &render(all)))?;
    Ok((
        all.iter().filter(|i| matches!(i, Item::Comp   { .. })).count(),
        all.iter().filter(|i| matches!(i, Item::Struct { .. })).count(),
    ))
}
