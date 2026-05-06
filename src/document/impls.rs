use super::types::{KmeDocument, KmeNode, KmeNodeId, KmeNodeKind};
use crate::TextFingerprint;

impl KmeDocument {
    pub fn nodes_by_kind(&self, predicate: impl Fn(&KmeNodeKind) -> bool) -> Vec<&KmeNode> {
        self.nodes
            .iter()
            .filter(|node| predicate(&node.kind))
            .collect()
    }

    pub fn node_by_id(&self, id: &KmeNodeId) -> Option<&KmeNode> {
        self.nodes.iter().find(|node| &node.id == id)
    }
}

impl KmeNode {
    pub(crate) fn new(
        kind: KmeNodeKind,
        raw: &str,
        ordinal: usize,
        source: crate::SourceSpan,
    ) -> Self {
        Self {
            id: KmeNodeId::from_parts(&kind, raw, ordinal),
            kind,
            source,
            children: Vec::new(),
        }
    }
}

impl KmeNodeId {
    pub(crate) fn from_parts(kind: &KmeNodeKind, raw: &str, ordinal: usize) -> Self {
        let seed = format!("{}\0{}\0{}", kind.label(), raw, ordinal);
        Self(format!("kme-{}", TextFingerprint::for_text(&seed).value))
    }
}

impl KmeNodeKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Heading(_) => "heading",
            Self::Paragraph => "paragraph",
            Self::Emoji(_) => "emoji",
            Self::HtmlBlock(_) => "html-block",
            Self::List(_) => "list",
            Self::CodeBlock(_) => "code-block",
            Self::Table(_) => "table",
            Self::BlockQuote => "blockquote",
            Self::Alert { .. } => "alert",
            Self::DescriptionList { .. } => "description-list",
            Self::ThematicBreak => "thematic-break",
            Self::RawBlock { .. } => "raw-block",
        }
    }
}
