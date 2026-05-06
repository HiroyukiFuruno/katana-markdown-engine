use katana_markdown_engine::{
    CodeBlockRole, DiagramKind, HtmlBlockRole, KmeNodeKind, MarkdownInput, parse_markdown,
};

#[test]
fn parses_sample_fixture_into_renderer_neutral_nodes() {
    let document = parse_markdown(MarkdownInput::from_content(
        "tests/fixtures/sample.md",
        include_str!("fixtures/sample.md"),
    ))
    .unwrap();

    assert!(document.nodes.iter().all(|node| !node.id.0.is_empty()));
    assert!(
        document
            .nodes
            .iter()
            .all(|node| !node.source.raw.text.is_empty())
    );
    assert!(
        document
            .nodes_by_kind(|kind| matches!(kind, KmeNodeKind::Heading(_)))
            .len()
            >= 3
    );
    assert!(
        document
            .nodes_by_kind(|kind| matches!(kind, KmeNodeKind::Table(_)))
            .len()
            == 1
    );
    assert!(
        document
            .nodes
            .iter()
            .any(|node| { matches!(node.kind, KmeNodeKind::HtmlBlock(HtmlBlockRole::BadgeRow)) })
    );
    assert!(document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmeNodeKind::CodeBlock(CodeBlockRole::Diagram {
                kind: DiagramKind::Mermaid
            })
        )
    }));
}

#[test]
fn parses_alert_blocks_without_erasing_labels() {
    let document = parse_markdown(MarkdownInput::from_content(
        "tests/fixtures/alerts.md",
        include_str!("fixtures/alerts.md"),
    ))
    .unwrap();

    let labels: Vec<String> = document
        .nodes_by_kind(|kind| matches!(kind, KmeNodeKind::Alert { .. }))
        .iter()
        .filter_map(|node| match &node.kind {
            KmeNodeKind::Alert { label } => Some(label.clone()),
            _ => None,
        })
        .collect();

    assert_eq!(labels, vec!["NOTE", "WARNING"]);
}

#[test]
fn parses_description_list_as_owned_node() {
    let document = parse_markdown(MarkdownInput::from_content(
        "tests/fixtures/description_list.md",
        include_str!("fixtures/description_list.md"),
    ))
    .unwrap();

    let lists = document.nodes_by_kind(|kind| matches!(kind, KmeNodeKind::DescriptionList { .. }));
    assert_eq!(lists.len(), 1);
    assert!(matches!(
        &lists[0].kind,
        KmeNodeKind::DescriptionList { items } if items.len() == 2
    ));
}
