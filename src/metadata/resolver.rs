use super::types::{
    MetadataDocument, MetadataEntry, TargetResolution, TargetResolutionKind, UnresolvedTarget,
};
use crate::{KmeDocument, KmeNode, TextFingerprint};

pub struct MetadataResolver;

impl MetadataResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn reconcile(
        &self,
        old_document: &KmeDocument,
        new_document: &KmeDocument,
        metadata: &MetadataDocument,
    ) -> Vec<TargetResolution> {
        metadata
            .entries
            .iter()
            .map(|entry| self.resolve_entry(old_document, new_document, entry))
            .collect()
    }

    fn resolve_entry(
        &self,
        old_document: &KmeDocument,
        new_document: &KmeDocument,
        entry: &MetadataEntry,
    ) -> TargetResolution {
        if let Some(node) = new_document.node_by_id(&entry.target.node_id) {
            return Self::resolved(entry, node);
        }
        if let Some(node) = self.find_by_fingerprint(new_document, &entry.target.text_fingerprint) {
            return Self::moved(entry, node);
        }
        Self::unresolved(old_document, entry)
    }

    fn find_by_fingerprint<'a>(
        &self,
        document: &'a KmeDocument,
        fingerprint: &TextFingerprint,
    ) -> Option<&'a KmeNode> {
        document
            .nodes
            .iter()
            .find(|node| &node.source.raw.fingerprint() == fingerprint)
    }

    fn resolved(entry: &MetadataEntry, node: &KmeNode) -> TargetResolution {
        TargetResolution {
            key: entry.key.clone(),
            kind: TargetResolutionKind::Resolved {
                node_id: node.id.clone(),
            },
        }
    }

    fn moved(entry: &MetadataEntry, node: &KmeNode) -> TargetResolution {
        TargetResolution {
            key: entry.key.clone(),
            kind: TargetResolutionKind::Moved {
                previous_node_id: entry.target.node_id.clone(),
                node_id: node.id.clone(),
            },
        }
    }

    fn unresolved(old_document: &KmeDocument, entry: &MetadataEntry) -> TargetResolution {
        let old_exists = old_document.node_by_id(&entry.target.node_id).is_some();
        let reason = if old_exists {
            "target disappeared"
        } else {
            "target unknown"
        };
        TargetResolution {
            key: entry.key.clone(),
            kind: TargetResolutionKind::Unresolved(UnresolvedTarget {
                node_id: entry.target.node_id.clone(),
                reason: reason.to_string(),
            }),
        }
    }
}

impl Default for MetadataResolver {
    fn default() -> Self {
        Self::new()
    }
}
