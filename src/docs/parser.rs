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
/// - `"code"` — fenced code block
/// - `"li"` / `"li2"` — list item (level 0 / level 1+)
/// - `"th"` / `"tr"` — table header / data row
/// - `"hr"` — horizontal rule
/// - `"bq"` — blockquote
/// P ar se.
pub fn parse(input: &str) -> Vec<DocBlock> {
    BlockParser::new().run(input)
}

/// Stateful accumulator that converts a CommonMark event stream into DocBlocks.
struct BlockParser {
    blocks:          Vec<DocBlock>,
    kind:            String,
    text:            String,
    list_depth:      i32,
    bq_depth:        i32,
    bq_parts:        Vec<String>,
    bq_buf:          String,
    is_table_head:   bool,
    is_in_table_row: bool,
    row_cells:       Vec<String>,
    cell_buf:        String,
}

impl BlockParser {
    fn new() -> Self {
        Self {
            blocks: Vec::new(), kind: String::new(), text: String::new(),
            list_depth: 0, bq_depth: 0,
            bq_parts: Vec::new(), bq_buf: String::new(),
            is_table_head: false, is_in_table_row: false,
            row_cells: Vec::new(), cell_buf: String::new(),
        }
    }

    fn run(mut self, input: &str) -> Vec<DocBlock> {
        let opts = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
        for event in Parser::new_ext(input, opts) { self.handle(event); }
        self.blocks
    }

    fn handle(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag)  => self.handle_start(tag),
            Event::End(tag)    => self.handle_end(tag),
            Event::Text(t)     => self.handle_text(&t, false),
            Event::Code(t)     => self.handle_text(&format!("`{t}`"), true),
            Event::SoftBreak   => { if self.kind == DOC_KIND_CODE { self.text.push('\n'); } else { self.text.push(' '); } }
            Event::HardBreak   => { self.text.push('\n'); }
            Event::Rule        => { self.blocks.push(DocBlock { kind: "hr".into(), text: "".into(), indent: 0 }); }
            _ => {}
        }
    }

    fn handle_start(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Heading { level, .. } => { self.kind = Self::heading_kind(level).into(); self.text.clear(); }
            Tag::Paragraph  => { if self.bq_depth == 0 { self.kind = "p".into(); self.text.clear(); } else { self.bq_buf.clear(); } }
            Tag::CodeBlock(_) => { self.kind = "code".into(); self.text.clear(); }
            Tag::List(_)    => { self.list_depth += 1; }
            Tag::Item       => { self.kind = if self.list_depth > 1 { "li2" } else { "li" }.into(); self.text.clear(); }
            Tag::BlockQuote(_) => { self.bq_depth += 1; self.bq_parts.clear(); self.bq_buf.clear(); }
            Tag::TableHead  => { self.is_table_head = true; self.row_cells.clear(); }
            Tag::TableRow   => { self.is_in_table_row = true; self.row_cells.clear(); }
            Tag::TableCell  => { self.cell_buf.clear(); }
            _ => {}
        }
    }

    fn handle_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Heading(_) => {
                let text = std::mem::take(&mut self.text);
                self.push_block(&self.kind.clone(), text, 0);
            }
            TagEnd::Paragraph if self.bq_depth == 0 => {
                let text = std::mem::take(&mut self.text);
                self.push_block("p", text, 0);
            }
            TagEnd::Paragraph    => { self.flush_bq(); }
            TagEnd::CodeBlock    => {
                let t = self.text.trim_end_matches('\n').to_string();
                self.push_block("code", t, 0);
                self.text.clear();
            }
            TagEnd::List(_)      => { self.list_depth -= 1; }
            TagEnd::Item         => {
                let text  = std::mem::take(&mut self.text);
                let depth = self.list_depth - 1;
                let kind  = self.kind.clone();
                self.push_block(&kind, text, depth);
            }
            TagEnd::BlockQuote(_) => {
                self.flush_bq();
                let joined = self.bq_parts.join(" ");
                self.push_block("bq", joined, 0);
                self.bq_parts.clear();
                self.bq_depth -= 1;
            }
            tag => self.handle_end_table(tag),
        }
    }

    fn handle_end_table(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::TableHead => {
                let text = self.row_cells.join(" │ ");
                self.push_block("th", text, 0);
                self.is_table_head = false;
            }
            TagEnd::TableRow if !self.is_table_head => {
                let text = self.row_cells.join(" │ ");
                self.push_block("tr", text, 0);
                self.is_in_table_row = false;
            }
            TagEnd::TableRow  => { self.is_in_table_row = false; }
            TagEnd::TableCell => {
                let cell = std::mem::take(&mut self.cell_buf).trim().to_string();
                self.row_cells.push(cell);
            }
            _ => {}
        }
    }

    fn handle_text(&mut self, s: &str, is_code: bool) {
        if self.is_in_table_row || self.is_table_head { self.cell_buf.push_str(s); }
        else if self.bq_depth > 0                    { self.bq_buf.push_str(s); }
        else if is_code                              { self.text.push_str(s); }
        else                                         { self.text.push_str(s); }
    }

    fn push_block(&mut self, kind: &str, text: String, indent: i32) {
        let t = text.trim().to_string();
        if !t.is_empty() || kind == DOC_KIND_HR {
            self.blocks.push(DocBlock { kind: kind.into(), text: t.into(), indent });
        }
    }

    fn flush_bq(&mut self) {
        let s = std::mem::take(&mut self.bq_buf).trim().to_string();
        if !s.is_empty() { self.bq_parts.push(s); }
    }

    fn heading_kind(level: HeadingLevel) -> &'static str {
        match level { HeadingLevel::H1 => "h1", HeadingLevel::H2 => "h2", _ => "h3" }
    }
}
