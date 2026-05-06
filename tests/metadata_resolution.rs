use katana_markdown_engine::{
    ContextAnchor, KmeNodeKind, MarkdownInput, MetadataDocument, MetadataEntry, MetadataTarget,
    TargetResolutionKind, parse_markdown, reconcile_metadata_targets,
};
use serde_json::json;
use std::path::PathBuf;

#[test]
fn resolves_metadata_target_by_stable_node_id() {
    let document = sample_document("# Title\n\nBody text\n");
    let heading = document
        .nodes_by_kind(|kind| matches!(kind, KmeNodeKind::Heading(_)))
        .remove(0);
    let metadata = metadata_for_node("title-note", &document.path, heading);

    let resolutions = reconcile_metadata_targets(&document, &document, &metadata);

    assert!(matches!(
        &resolutions[0].kind,
        TargetResolutionKind::Resolved { node_id } if node_id == &heading.id
    ));
}

#[test]
fn unresolved_metadata_is_returned_without_deletion() {
    let old_document = sample_document("# Title\n\nBody text\n");
    let new_document = sample_document("# Other\n\nBody text\n");
    let heading = old_document
        .nodes_by_kind(|kind| matches!(kind, KmeNodeKind::Heading(_)))
        .remove(0);
    let metadata = metadata_for_node("title-note", &old_document.path, heading);

    let resolutions = reconcile_metadata_targets(&old_document, &new_document, &metadata);

    assert!(matches!(
        &resolutions[0].kind,
        TargetResolutionKind::Unresolved(unresolved) if unresolved.node_id == heading.id
    ));
}

#[test]
fn resolves_metadata_target_by_fingerprint_when_node_id_changes() {
    let old_document = sample_document("# Title\n\nBody text\n");
    let new_document = sample_document("# Title\n\nBody text\n");
    let heading = old_document
        .nodes_by_kind(|kind| matches!(kind, KmeNodeKind::Heading(_)))
        .remove(0);
    let mut metadata = metadata_for_node("title-note", &old_document.path, heading);
    metadata.entries[0].target.node_id.0 = "kme-old-id".to_string();

    let resolutions = reconcile_metadata_targets(&old_document, &new_document, &metadata);

    assert!(matches!(
        &resolutions[0].kind,
        TargetResolutionKind::Moved { previous_node_id, .. } if previous_node_id.0 == "kme-old-id"
    ));
}

fn sample_document(content: &str) -> katana_markdown_engine::KmeDocument {
    parse_markdown(MarkdownInput::from_content("README.md", content)).unwrap()
}

fn metadata_for_node(
    key: &str,
    path: &std::path::Path,
    node: &katana_markdown_engine::KmeNode,
) -> MetadataDocument {
    MetadataDocument {
        markdown_path: path.to_path_buf(),
        entries: vec![MetadataEntry {
            key: key.to_string(),
            target: MetadataTarget {
                file_path: PathBuf::from(path),
                node_id: node.id.clone(),
                byte_range: node.source.byte_range,
                line_column_range: node.source.line_column_range,
                text_fingerprint: node.source.raw.fingerprint(),
                context: ContextAnchor {
                    before: String::new(),
                    after: String::new(),
                },
            },
            payload: json!({ "kind": "note" }),
        }],
    }
}
