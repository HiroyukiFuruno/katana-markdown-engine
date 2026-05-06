use crate::{ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan};

#[derive(Debug, Clone)]
pub(crate) struct SourceLine {
    pub number: usize,
    pub start: usize,
    pub end: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LineIndex {
    lines: Vec<SourceLine>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut lines = Vec::new();
        let mut offset = 0;
        for (index, segment) in source.split_inclusive('\n').enumerate() {
            let text = segment.trim_end_matches('\n').to_string();
            let end = offset + text.len();
            lines.push(SourceLine {
                number: index + 1,
                start: offset,
                end,
                text,
            });
            offset += segment.len();
        }
        if source.is_empty() {
            lines.push(SourceLine {
                number: 1,
                start: 0,
                end: 0,
                text: String::new(),
            });
        }
        Self { lines }
    }

    pub fn lines(&self) -> &[SourceLine] {
        &self.lines
    }

    pub fn source_span(&self, source: &str, start_line: usize, end_line: usize) -> SourceSpan {
        let first = &self.lines[start_line];
        let last = &self.lines[end_line - 1];
        let end = last.end;
        SourceSpan {
            byte_range: ByteRange {
                start: first.start,
                end,
            },
            line_column_range: LineColumnRange {
                start: LineColumn {
                    line: first.number,
                    column: 1,
                },
                end: LineColumn {
                    line: last.number,
                    column: last.text.chars().count() + 1,
                },
            },
            raw: RawSnippet::new(source[first.start..end].to_string()),
        }
    }
}
