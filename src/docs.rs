//! Markdown parser — converts CommonMark + GFM to `[DocBlock]` for `MarkdownView`.
//!
//! # Example
//! ```rust,no_run
//! let blocks = slint_ui_templates::docs::parse("# Hello\n\nWorld");
//! // push to DocsApp or MarkdownView via set_doc_blocks() / set_blocks()
//! ```

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use crate::DocBlock;

/// Well-known doc block kinds used as discriminators.
const DOC_KIND_HR: &str = "hr";
const DOC_KIND_CODE: &str = "code";

/// Parse a CommonMark string into a flat list of [`DocBlock`] items.
///
/// Supported block types:
/// - `"h1"` / `"h2"` / `"h3"` — headings
/// - `"p"` — paragraph
/// - `"code"` — fenced code block (Consolas font in MarkdownView)
/// - `"li"` / `"li2"` — list item (level 0 / level 1+)
/// - `"th"` / `"tr"` — table header / data row (cells joined with ` │ `)
/// - `"hr"` — horizontal rule
/// - `"bq"` — blockquote
pub fn parse(input: &str) -> Vec<DocBlock> {
    let opts = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(input, opts);

    let mut blocks: Vec<DocBlock> = Vec::new();
    let mut kind         = String::new();
    let mut text         = String::new();
    let mut list_depth: i32 = 0;
    let mut bq_depth:   i32 = 0;
    let mut bq_parts: Vec<String> = Vec::new();
    let mut bq_buf  = String::new();

    // Table accumulation
    let mut is_table_head = false;
    let mut in_table_row  = false;
    let mut row_cells: Vec<String> = Vec::new();
    let mut cell_buf = String::new();

    fn push(blocks: &mut Vec<DocBlock>, kind: &str, text: String, indent: i32) {
        let t = text.trim().to_string();
        if !t.is_empty() || kind == DOC_KIND_HR {
            blocks.push(DocBlock { kind: kind.into(), text: t.into(), indent });
        }
    }

    for event in parser {
        match event {
            // ── Headings ──────────────────────────────────────────────────────
            Event::Start(Tag::Heading { level, .. }) => {
                kind = match level {
                    HeadingLevel::H1 => "h1",
                    HeadingLevel::H2 => "h2",
                    _                => "h3",
                }.into();
                text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                push(&mut blocks, &kind, std::mem::take(&mut text), 0);
            }

            // ── Paragraphs ────────────────────────────────────────────────────
            Event::Start(Tag::Paragraph) => {
                if bq_depth == 0 { kind = "p".into(); text.clear(); }
                else             { bq_buf.clear(); }
            }
            Event::End(TagEnd::Paragraph) => {
                if bq_depth == 0 {
                    push(&mut blocks, "p", std::mem::take(&mut text), 0);
                } else {
                    let s = std::mem::take(&mut bq_buf);
                    let s = s.trim().to_string();
                    if !s.is_empty() { bq_parts.push(s); }
                }
            }

            // ── Code blocks ───────────────────────────────────────────────────
            Event::Start(Tag::CodeBlock(_)) => { kind = "code".into(); text.clear(); }
            Event::End(TagEnd::CodeBlock) => {
                let t = text.trim_end_matches('\n').to_string();
                push(&mut blocks, "code", t, 0);
                text.clear();
            }

            // ── Lists ─────────────────────────────────────────────────────────
            Event::Start(Tag::List(_)) => { list_depth += 1; }
            Event::End(TagEnd::List(_)) => { list_depth -= 1; }

            Event::Start(Tag::Item) => {
                kind = if list_depth > 1 { "li2" } else { "li" }.into();
                text.clear();
            }
            Event::End(TagEnd::Item) => {
                push(&mut blocks, &kind, std::mem::take(&mut text), list_depth - 1);
            }

            // ── Blockquotes ───────────────────────────────────────────────────
            // Paragraphs inside blockquotes are accumulated into bq_parts
            // so they don't push separate "p" blocks.
            Event::Start(Tag::BlockQuote(_)) => {
                bq_depth += 1;
                bq_parts.clear();
                bq_buf.clear();
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                if !bq_buf.trim().is_empty() {
                    bq_parts.push(bq_buf.trim().to_string());
                }
                let combined = bq_parts.join(" ");
                push(&mut blocks, "bq", combined, 0);
                bq_parts.clear();
                bq_depth -= 1;
            }

            // ── Tables ────────────────────────────────────────────────────────
            Event::Start(Tag::Table(_)) | Event::End(TagEnd::Table) => {}

            Event::Start(Tag::TableHead) => { is_table_head = true; row_cells.clear(); }
            Event::End(TagEnd::TableHead) => {
                push(&mut blocks, "th", row_cells.join(" │ "), 0);
                is_table_head = false;
            }

            Event::Start(Tag::TableRow) => { in_table_row = true; row_cells.clear(); }
            Event::End(TagEnd::TableRow) => {
                if !is_table_head {
                    push(&mut blocks, "tr", row_cells.join(" │ "), 0);
                }
                in_table_row = false;
            }

            Event::Start(Tag::TableCell) => { cell_buf.clear(); }
            Event::End(TagEnd::TableCell) => {
                row_cells.push(std::mem::take(&mut cell_buf).trim().to_string());
            }

            // ── Thematic break ────────────────────────────────────────────────
            Event::Rule => {
                blocks.push(DocBlock { kind: "hr".into(), text: "".into(), indent: 0 });
            }

            // ── Inline text ───────────────────────────────────────────────────
            Event::Text(t) => {
                if in_table_row || is_table_head { cell_buf.push_str(&t); }
                else if bq_depth > 0             { bq_buf.push_str(&t); }
                else                             { text.push_str(&t); }
            }
            Event::Code(t) => {
                let s = format!("`{t}`");
                if in_table_row || is_table_head { cell_buf.push_str(&s); }
                else if bq_depth > 0             { bq_buf.push_str(&s); }
                else                             { text.push_str(&s); }
            }
            Event::SoftBreak => {
                if kind == DOC_KIND_CODE { text.push('\n'); } else { text.push(' '); }
            }
            Event::HardBreak => { text.push('\n'); }

            _ => {}
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_levels() {
        let b = parse("# H1\n## H2\n### H3");
        assert_eq!(b[0].kind.as_str(), "h1");
        assert_eq!(b[1].kind.as_str(), "h2");
        assert_eq!(b[2].kind.as_str(), "h3");
        assert_eq!(b[0].text.as_str(), "H1");
    }

    #[test]
    fn code_block_preserved() {
        let b = parse("```rust\nfn main() {}\n```");
        assert_eq!(b[0].kind.as_str(), "code");
        assert!(b[0].text.contains("fn main()"));
    }

    #[test]
    fn list_items() {
        let b = parse("- alpha\n- beta\n  - nested");
        let kinds: Vec<&str> = b.iter().map(|x| x.kind.as_str()).collect();
        assert!(kinds.contains(&"li"));
        assert!(kinds.contains(&"li2"));
    }

    #[test]
    fn table_header_and_row() {
        let b = parse("| A | B |\n|---|---|\n| 1 | 2 |");
        let kinds: Vec<&str> = b.iter().map(|x| x.kind.as_str()).collect();
        assert!(kinds.contains(&"th"));
        assert!(kinds.contains(&"tr"));
        assert!(b.iter().find(|x| x.kind == "th").unwrap().text.contains("│"));
    }

    #[test]
    fn horizontal_rule() {
        let b = parse("---");
        assert_eq!(b[0].kind.as_str(), "hr");
    }

    #[test]
    fn blockquote() {
        let b = parse("> Note this");
        assert_eq!(b[0].kind.as_str(), "bq");
        assert!(b[0].text.contains("Note this"));
    }
}
