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
    CodeBlockRole, DescriptionItem, DiagramKind, EmojiNode, HeadingNode, HtmlBlockRole,
    KmeDocument, KmeNode, KmeNodeId, KmeNodeKind, ListNode, TableAlignment, TableCell, TableNode,
    TableRow,
};
pub use error::KmeError;
pub use input::MarkdownInput;
pub use metadata::{
    ConflictedTarget, ContextAnchor, MetadataDocument, MetadataEntry, MetadataReconcileRequest,
    MetadataReconcileResult, MetadataResolver, MetadataTarget, TargetResolution,
    TargetResolutionKind, UnresolvedTarget,
};
pub use source::{ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TextFingerprint};

pub struct KmeMarkdownEngine;

impl KmeMarkdownEngine {
    pub fn parse(input: MarkdownInput) -> Result<KmeDocument, KmeError> {
        parser::MarkdownParser::new().parse(input)
    }

    pub fn reconcile_targets(
        old_document: &KmeDocument,
        new_document: &KmeDocument,
        metadata: &MetadataDocument,
    ) -> Vec<TargetResolution> {
        MetadataResolver::new().reconcile(old_document, new_document, metadata)
    }

    pub fn reconcile(request: MetadataReconcileRequest) -> MetadataReconcileResult {
        MetadataResolver::new().reconcile_request(request)
    }
}
