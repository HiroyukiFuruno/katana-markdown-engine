#![deny(warnings, clippy::all)]
#![allow(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod document;
pub mod error;
pub mod input;
pub mod metadata;
pub mod source;

mod parser;

pub use document::{
    CodeBlockRole, DescriptionItem, DiagramKind, HeadingNode, HtmlBlockRole, KmeDocument, KmeNode,
    KmeNodeId, KmeNodeKind, ListNode, TableAlignment, TableCell, TableNode, TableRow,
};
pub use error::KmeError;
pub use input::MarkdownInput;
pub use metadata::{
    ContextAnchor, MetadataDocument, MetadataEntry, MetadataResolver, MetadataTarget,
    TargetResolution, TargetResolutionKind, UnresolvedTarget,
};
pub use source::{ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TextFingerprint};

pub fn parse_markdown(input: MarkdownInput) -> Result<KmeDocument, KmeError> {
    parser::MarkdownParser::new().parse(input)
}

pub fn reconcile_metadata_targets(
    old_document: &KmeDocument,
    new_document: &KmeDocument,
    metadata: &MetadataDocument,
) -> Vec<TargetResolution> {
    MetadataResolver::new().reconcile(old_document, new_document, metadata)
}
