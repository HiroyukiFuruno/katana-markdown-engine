use super::block::BlockParser;
use super::emoji::EmojiParser;
use super::line_index::{LineIndex, SourceLine};
use crate::{
    KmeDocument, KmeError, KmeNode, KmeNodeKind, MarkdownInput, SourceSpan, TextFingerprint,
};
use std::collections::HashMap;

pub(crate) struct MarkdownParser;

impl MarkdownParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: MarkdownInput) -> Result<KmeDocument, KmeError> {
        let (path, content) = input.into_parts()?;
        if content.trim().is_empty() {
            return Err(KmeError::EmptySource);
        }
        let index = LineIndex::new(&content);
        let mut cursor = ParserCursor::new(&content, &index);
        Ok(KmeDocument {
            path,
            fingerprint: TextFingerprint::for_text(&content),
            nodes: cursor.parse_nodes(),
        })
    }
}

pub(super) struct ParserCursor<'a> {
    pub(super) source: &'a str,
    pub(super) index: &'a LineIndex,
    pub(super) line: usize,
    ordinals: HashMap<&'static str, usize>,
}

impl<'a> ParserCursor<'a> {
    fn new(source: &'a str, index: &'a LineIndex) -> Self {
        Self {
            source,
            index,
            line: 0,
            ordinals: HashMap::new(),
        }
    }

    fn parse_nodes(&mut self) -> Vec<KmeNode> {
        let mut nodes = Vec::new();
        while self.line < self.index.lines().len() {
            if self.current().text.trim().is_empty() {
                self.line += 1;
                continue;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    fn parse_node(&mut self) -> KmeNode {
        let start = self.line;
        let kind = self.node_kind();
        let span = self.span(start);
        let children = self.inline_children(&kind, &span);
        let mut node = self.node(kind, span);
        node.children = children;
        node
    }

    fn node_kind(&mut self) -> KmeNodeKind {
        let line = self.current().text.as_str();
        if let Some(kind) = BlockParser::heading(line) {
            self.line += 1;
            return kind;
        }
        if line.trim_start().starts_with("```") {
            return self.code_block();
        }
        if self.is_html_start(line) {
            return self.html_block();
        }
        if self.is_table_start() {
            return self.table();
        }
        if line.trim_start().starts_with('>') {
            return self.block_quote();
        }
        if self.is_description_start() {
            return self.description_list();
        }
        if BlockParser::unordered_list_line(line) || BlockParser::ordered_list_line(line) {
            return self.list();
        }
        if line.trim() == "---" {
            self.line += 1;
            return KmeNodeKind::ThematicBreak;
        }
        self.paragraph()
    }

    fn node(&mut self, kind: KmeNodeKind, span: SourceSpan) -> KmeNode {
        let label = kind.label();
        let ordinal = *self
            .ordinals
            .entry(label)
            .and_modify(|it| *it += 1)
            .or_insert(0);
        let raw = span.raw.text.clone();
        KmeNode::new(kind, &raw, ordinal, span)
    }

    fn span(&self, start: usize) -> SourceSpan {
        self.index.source_span(self.source, start, self.line)
    }

    fn inline_children(&self, kind: &KmeNodeKind, span: &SourceSpan) -> Vec<KmeNode> {
        if !matches!(
            kind,
            KmeNodeKind::Heading(_) | KmeNodeKind::Paragraph | KmeNodeKind::List(_)
        ) {
            return Vec::new();
        }
        let base = span.byte_range.start;
        EmojiParser::emoji_nodes(&span.raw.text, |start, end| {
            self.index
                .source_span_for_byte_range(self.source, base + start, base + end)
        })
    }

    pub(super) fn current(&self) -> &SourceLine {
        &self.index.lines()[self.line]
    }

    pub(super) fn raw_text(&self, start: usize, end: usize) -> String {
        let span = self.index.source_span(self.source, start, end);
        span.raw.text
    }
}
