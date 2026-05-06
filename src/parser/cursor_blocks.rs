use super::block;
use super::engine::ParserCursor;
use crate::KmeNodeKind;

impl ParserCursor<'_> {
    pub(super) fn code_block(&mut self) -> KmeNodeKind {
        let role = block::code_block_role(&self.current().text);
        self.line += 1;
        while self.line < self.index.lines().len() {
            let text = self.current().text.trim_start().to_string();
            self.line += 1;
            if text.starts_with("```") {
                break;
            }
        }
        KmeNodeKind::CodeBlock(role)
    }

    pub(super) fn html_block(&mut self) -> KmeNodeKind {
        let start = self.line;
        self.line += 1;
        while self.line < self.index.lines().len() && !self.html_closed(start, self.line) {
            self.line += 1;
        }
        let raw = self.raw_text(start, self.line);
        KmeNodeKind::HtmlBlock(block::html_role(&raw))
    }

    pub(super) fn table(&mut self) -> KmeNodeKind {
        let mut lines = Vec::new();
        while self.line < self.index.lines().len() && self.current().text.contains('|') {
            lines.push(self.current().text.clone());
            self.line += 1;
        }
        KmeNodeKind::Table(block::table_node(&lines))
    }

    pub(super) fn block_quote(&mut self) -> KmeNodeKind {
        let mut lines = Vec::new();
        while self.line < self.index.lines().len()
            && self.current().text.trim_start().starts_with('>')
        {
            lines.push(self.current().text.clone());
            self.line += 1;
        }
        block::alert_label(&lines)
            .map(|label| KmeNodeKind::Alert { label })
            .unwrap_or(KmeNodeKind::BlockQuote)
    }

    pub(super) fn description_list(&mut self) -> KmeNodeKind {
        let mut lines = Vec::new();
        while self.line + 1 < self.index.lines().len() && self.next_line_starts_description() {
            lines.push(self.current().text.clone());
            lines.push(self.index.lines()[self.line + 1].text.clone());
            self.line += 2;
            self.skip_description_gap();
        }
        KmeNodeKind::DescriptionList {
            items: block::description_items(&lines),
        }
    }

    pub(super) fn list(&mut self) -> KmeNodeKind {
        let mut lines = Vec::new();
        while self.line < self.index.lines().len() {
            let text = &self.current().text;
            if !block::unordered_list_line(text) && !block::ordered_list_line(text) {
                break;
            }
            lines.push(text.clone());
            self.line += 1;
        }
        KmeNodeKind::List(block::list_node(&lines))
    }

    pub(super) fn paragraph(&mut self) -> KmeNodeKind {
        self.line += 1;
        KmeNodeKind::Paragraph
    }

    pub(super) fn is_table_start(&self) -> bool {
        self.line + 1 < self.index.lines().len()
            && self.current().text.contains('|')
            && block::table_separator(&self.index.lines()[self.line + 1].text)
    }

    pub(super) fn is_description_start(&self) -> bool {
        self.line + 1 < self.index.lines().len() && self.next_line_starts_description()
    }

    pub(super) fn is_html_start(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.starts_with("<p")
            || trimmed.starts_with("<h1")
            || trimmed.starts_with("<details")
            || trimmed.starts_with("<div")
    }

    fn skip_description_gap(&mut self) {
        if self.line + 2 >= self.index.lines().len() || !self.current().text.trim().is_empty() {
            return;
        }
        if self.index.lines()[self.line + 2]
            .text
            .trim_start()
            .starts_with(':')
        {
            self.line += 1;
        }
    }

    fn next_line_starts_description(&self) -> bool {
        self.index.lines()[self.line + 1]
            .text
            .trim_start()
            .starts_with(':')
    }

    fn html_closed(&self, start: usize, end: usize) -> bool {
        let raw = self.raw_text(start, end);
        raw.contains("</p>")
            || raw.contains("</h1>")
            || raw.contains("</details>")
            || raw.contains("</div>")
    }
}
