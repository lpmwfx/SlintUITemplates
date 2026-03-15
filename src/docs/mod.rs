//! Markdown parser — converts CommonMark + GFM to `[DocBlock]` for `MarkdownView`.
//!
//! # Example
//! ```rust,no_run
//! let blocks = slint_ui_templates::docs::parse("# Hello\n\nWorld");
//! // push to DocsApp or MarkdownView via set_doc_blocks() / set_blocks()
//! ```

/// Markdown-to-DocBlock parser implementation.
pub mod parser;

pub use parser::parse;

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
