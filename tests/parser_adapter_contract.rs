use katana_markdown_engine::{
    CodeBlockRole, DescriptionItem, DiagramKind, HtmlBlockRole, KmeDocument, KmeMarkdownEngine,
    KmeNodeKind, MarkdownInput, TableAlignment,
};

#[test]
fn locks_core_block_contracts_behind_public_dto() {
    let document = parse(
        "contracts.md",
        r##"<p align="center">
  <a href="#"><img src="badge.svg" alt="CI"></a>
</p>

> [!WARNING]
> Risk

Term
: Description

| Left | Center | Right |
| :--- | :---: | ---: |
| a | b | c |
"##,
    );

    assert!(has_html_role(&document, HtmlBlockRole::BadgeRow));
    assert!(has_alert(&document, "WARNING"));
    assert!(has_description_item(&document, "Term", "Description"));

    let table = document
        .nodes_by_kind(|kind| matches!(kind, KmeNodeKind::Table(_)))
        .remove(0);
    let KmeNodeKind::Table(table) = &table.kind else {
        panic!("expected table node");
    };
    assert_eq!(
        table.alignments,
        vec![
            TableAlignment::Left,
            TableAlignment::Center,
            TableAlignment::Right,
        ]
    );
    assert_eq!(table.rows[2].cells[2].text, "c");
    assert_eq!(table.rows[2].cells[2].source.raw.text, " c ");
}

#[test]
fn locks_diagram_math_footnote_and_emoji_contracts() {
    let document = parse(
        "inline-contracts.md",
        r#"# Title :sparkles:

Body with inline math $E = mc^2$ and footnote[^1] 🚀.

```mermaid
graph TD
  A --> B
```

```plantuml
@startuml
Alice -> Bob
@enduml
```

```drawio
<mxGraphModel></mxGraphModel>
```

```math
f(x) = x^2
```

[^1]: Footnote body
"#,
    );

    assert_diagram(&document, DiagramKind::Mermaid);
    assert_diagram(&document, DiagramKind::PlantUml);
    assert_diagram(&document, DiagramKind::DrawIo);
    assert!(
        document
            .nodes
            .iter()
            .any(|node| { matches!(&node.kind, KmeNodeKind::CodeBlock(CodeBlockRole::Math)) })
    );
    assert!(document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmeNodeKind::Paragraph
                if node.source.raw.text.contains("footnote[^1]")
                    && node.source.raw.text.contains("$E = mc^2$")
        )
    }));
    assert_eq!(emoji_values(&document), vec![":sparkles:", "🚀"]);
}

fn parse(path: &str, source: &str) -> KmeDocument {
    KmeMarkdownEngine::parse(MarkdownInput::from_content(path, source)).unwrap()
}

fn has_html_role(document: &KmeDocument, expected: HtmlBlockRole) -> bool {
    document
        .nodes
        .iter()
        .any(|node| matches!(&node.kind, KmeNodeKind::HtmlBlock(role) if *role == expected))
}

fn has_alert(document: &KmeDocument, expected: &str) -> bool {
    document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmeNodeKind::Alert { label } if label == expected
        )
    })
}

fn has_description_item(document: &KmeDocument, term: &str, description: &str) -> bool {
    document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmeNodeKind::DescriptionList { items }
                if items == &vec![DescriptionItem { term: term.to_string(), description: description.to_string() }]
        )
    })
}

fn assert_diagram(document: &KmeDocument, expected: DiagramKind) {
    assert!(document.nodes.iter().any(|node| {
        matches!(
            &node.kind,
            KmeNodeKind::CodeBlock(CodeBlockRole::Diagram { kind }) if *kind == expected
        )
    }));
}

fn emoji_values(document: &KmeDocument) -> Vec<String> {
    document
        .nodes
        .iter()
        .flat_map(|node| node.children.iter())
        .filter_map(|node| match &node.kind {
            KmeNodeKind::Emoji(emoji) => Some(emoji.value.clone()),
            _ => None,
        })
        .collect()
}
