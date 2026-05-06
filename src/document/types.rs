use crate::{SourceSpan, TextFingerprint};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KmeNodeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KmeDocument {
    pub path: PathBuf,
    pub fingerprint: TextFingerprint,
    pub nodes: Vec<KmeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KmeNode {
    pub id: KmeNodeId,
    pub kind: KmeNodeKind,
    pub source: SourceSpan,
    pub children: Vec<KmeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KmeNodeKind {
    Heading(HeadingNode),
    Paragraph,
    HtmlBlock(HtmlBlockRole),
    List(ListNode),
    CodeBlock(CodeBlockRole),
    Table(TableNode),
    BlockQuote,
    Alert { label: String },
    DescriptionList { items: Vec<DescriptionItem> },
    ThematicBreak,
    RawBlock { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadingNode {
    pub level: u8,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HtmlBlockRole {
    Generic,
    Centered,
    BadgeRow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListNode {
    pub ordered: bool,
    pub task_markers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeBlockRole {
    Plain { language: Option<String> },
    Diagram { kind: DiagramKind },
    Math,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagramKind {
    Mermaid,
    DrawIo,
    PlantUml,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableNode {
    pub alignments: Vec<TableAlignment>,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableAlignment {
    Left,
    Center,
    Right,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableCell {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptionItem {
    pub term: String,
    pub description: String,
}
