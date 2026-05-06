## Context

KME v0はCommonMark完全準拠を先に狙わない。現在KatanAが実現できているMarkdown挙動を落とさず、文書構造として扱えることを優先する。

## Goals

- `sample.md`、README badge、alert、description listをモデル化する。
- 全nodeにstable id、source range、line-column、byte offset、raw snippetを持たせる。
- metadata targetをnodeへ解決し、移動、衝突、unresolvedを返す。
- Mermaid、draw.io、PlantUML、math、emojiを構造として保持する。
- 既存parserのASTをpublic contractにしない。
- P0 `katana-ast-lint` を品質ゲートとして使う。

## Non-Goals

- KME v0でMarkdown全仕様を完全実装すること。
- HTML/PDF/PNG/JPG出力をKMEが直接担当すること。
- Floemやkcfなど利用側実装へ依存すること。
- metadataをMarkdown本文へ埋め込むこと。

## Decisions

### Document Model First

KMEはHTML変換ではなく文書モデルを正本にする。HTML生成、preview描画、PDFページングは利用側がKMEモデルを消費して行う。

### P0 Quality Gate

KMEはP1として実装する。P0 `katana-ast-lint` の共通ruleとrepository adapterを使い、KME固有の一時lintや除外設定で品質ゲートを代替しない。

### Lossless-leaning Source Mapping

AST単位コピー・編集とmetadata更新のため、nodeは元テキスト断片とsource rangeを保持する。完全なformatterではなく、編集対象範囲を安全に特定できることを優先する。

### Metadata Resolution

KMEは旧本文、新本文、metadataを受け取り、targetの再対応結果を返す。復元できないtargetはunresolvedとして返し、削除しない。

### Renderer Interface

diagram/mathは非同期renderer interfaceへ委譲する。rendererが無い、失敗した、未対応の場合はraw blockとして扱う。

## Public Model Candidates

- `KmeDocument`
- `KmeNode`
- `KmeNodeId`
- `SourceRange`
- `LineColumnRange`
- `RawSnippet`
- `MetadataDocument`
- `MetadataTarget`
- `TargetResolution`
- `UnresolvedTarget`
- `DiagramNode`
- `HighlightedSpan`
- `AstLintAdapterConfig`
