use katana_markdown_engine::{
    ContextAnchor, KmeDocument, KmeNode, KmeNodeId, MarkdownInput, MetadataDocument, MetadataEntry,
    MetadataReconcileRequest, MetadataTarget, TargetResolution, TargetResolutionKind,
    parse_markdown, reconcile_metadata,
};
use serde_json::json;
use std::path::{Path, PathBuf};

const MARKDOWN_SOURCE: &str =
    include_str!("../../../tests/fixtures/canonical/katana_sample_basic.md");
const OLD_METADATA_SOURCE: &str = "# Title\n\nSame\n\nGone\n";
const NEW_METADATA_SOURCE: &str = "# Title\n\nSame\n\nSame\n";

fn main() {
    let document = parse_markdown(MarkdownInput::from_content(
        "tests/fixtures/canonical/katana_sample_basic.md",
        MARKDOWN_SOURCE,
    ))
    .expect("manual harness fixture must parse");
    let metadata_result = reconcile_metadata(metadata_request());

    print_markdown(MARKDOWN_SOURCE);
    print_nodes(&document);
    print_selected_node(&document);
    print_metadata(&metadata_result.resolutions);
}

fn print_markdown(source: &str) {
    println!("== Markdown入力 ==");
    for line in source.lines().take(24) {
        println!("{line}");
    }
    println!();
}

fn print_nodes(document: &KmeDocument) {
    println!("== KME node一覧 ==");
    for (index, node) in document.nodes.iter().enumerate() {
        println!(
            "#{index:02} {:<18} bytes {}..{} lines {}:{}..{}:{}",
            node.kind.label(),
            node.source.byte_range.start,
            node.source.byte_range.end,
            node.source.line_column_range.start.line,
            node.source.line_column_range.start.column,
            node.source.line_column_range.end.line,
            node.source.line_column_range.end.column
        );
    }
    println!();
}

fn print_selected_node(document: &KmeDocument) {
    println!("== 選択node（初期選択: 0） ==");
    let Some(node) = document.nodes.first() else {
        println!("nodeなし");
        return;
    };
    println!("id: {}", node.id.0);
    println!("kind: {}", node.kind.label());
    println!(
        "source range: {}..{}",
        node.source.byte_range.start, node.source.byte_range.end
    );
    println!(
        "line-column: {}:{}..{}:{}",
        node.source.line_column_range.start.line,
        node.source.line_column_range.start.column,
        node.source.line_column_range.end.line,
        node.source.line_column_range.end.column
    );
    println!("fingerprint: {}", node.source.raw.fingerprint().value);
    println!("raw snippet:");
    println!("{}", node.source.raw.text);
    println!();
}

fn print_metadata(resolutions: &[TargetResolution]) {
    println!("== metadata解決状態 ==");
    for resolution in resolutions {
        println!("{}: {}", resolution.key, resolution_label(&resolution.kind));
    }
}

fn resolution_label(kind: &TargetResolutionKind) -> String {
    match kind {
        TargetResolutionKind::Resolved { node_id } => format!("Resolved -> {}", node_id.0),
        TargetResolutionKind::Moved {
            previous_node_id,
            node_id,
        } => {
            format!("Moved {} -> {}", previous_node_id.0, node_id.0)
        }
        TargetResolutionKind::Conflict(conflict) => {
            format!("Conflict candidates={}", conflict.candidate_node_ids.len())
        }
        TargetResolutionKind::Unresolved(unresolved) => {
            format!("Unresolved {}", unresolved.reason)
        }
    }
}

fn metadata_request() -> MetadataReconcileRequest {
    let old_document = parse_markdown(MarkdownInput::from_content(
        "metadata-harness.md",
        OLD_METADATA_SOURCE,
    ))
    .expect("old metadata fixture must parse");
    let new_document = parse_markdown(MarkdownInput::from_content(
        "metadata-harness.md",
        NEW_METADATA_SOURCE,
    ))
    .expect("new metadata fixture must parse");
    let markdown_path = PathBuf::from("metadata-harness.md");

    MetadataReconcileRequest {
        metadata: MetadataDocument {
            markdown_path: markdown_path.clone(),
            entries: metadata_entries(&old_document, &markdown_path),
        },
        old_document,
        new_document,
    }
}

fn metadata_entries(document: &KmeDocument, path: &Path) -> Vec<MetadataEntry> {
    let title = node_containing(document, "# Title");
    let same = node_containing(document, "Same");
    let gone = node_containing(document, "Gone");

    vec![
        metadata_entry("resolved-title", path, title, title.id.clone()),
        metadata_entry(
            "moved-title",
            path,
            title,
            KmeNodeId("old-title".to_string()),
        ),
        metadata_entry("unresolved-gone", path, gone, gone.id.clone()),
        metadata_entry(
            "conflict-same",
            path,
            same,
            KmeNodeId("old-same".to_string()),
        ),
    ]
}

fn node_containing<'a>(document: &'a KmeDocument, expected: &str) -> &'a KmeNode {
    document
        .nodes
        .iter()
        .find(|node| node.source.raw.text.contains(expected))
        .expect("manual harness metadata node must exist")
}

fn metadata_entry(key: &str, path: &Path, node: &KmeNode, node_id: KmeNodeId) -> MetadataEntry {
    MetadataEntry {
        key: key.to_string(),
        target: MetadataTarget {
            file_path: path.to_path_buf(),
            node_id,
            byte_range: node.source.byte_range,
            line_column_range: node.source.line_column_range,
            text_fingerprint: node.source.raw.fingerprint(),
            context: ContextAnchor {
                before: String::new(),
                after: String::new(),
            },
        },
        payload: json!({ "kind": "manual-harness" }),
    }
}
