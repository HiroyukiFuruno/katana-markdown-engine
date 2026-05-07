# Downstream Handoff Contract

## 結論

KME `v0.1.0` のdownstream受け渡し境界は、KME public DTOとmetadata APIだけで固定する。

downstreamは `src/parser/**`、third-party parser AST、KME内部実装型へ依存しない。

## Public API境界

downstreamが使ってよい入口は次に限定する。

- `parse_markdown(MarkdownInput) -> Result<KmeDocument, KmeError>`
- `reconcile_metadata(MetadataReconcileRequest) -> MetadataReconcileResult`
- `reconcile_metadata_targets(&KmeDocument, &KmeDocument, &MetadataDocument) -> Vec<TargetResolution>`

downstreamが参照してよいDTOは次に限定する。

- `KmeDocument`
- `KmeNode`
- `KmeNodeKind`
- `SourceSpan`
- `RawSnippet`
- `TextFingerprint`
- `MetadataDocument`
- `MetadataEntry`
- `MetadataTarget`
- `MetadataReconcileRequest`
- `MetadataReconcileResult`
- `TargetResolution`
- `TargetResolutionKind`

## kdp

kdpは `KmeDocument` をpreview inputとして受け取る。

hit-test metadataは、KME nodeから次を参照する。

- `KmeNodeId`
- `KmeNodeKind`
- `SourceSpan.byte_range`
- `SourceSpan.line_column_range`
- `SourceSpan.raw`
- `SourceSpan.raw.fingerprint()`

kdpはMarkdownを再parseしない。HTML変換結果をKMEの代替contractにしない。

## kle

kleは保存時に、old document、new document、metadata documentをKMEへ渡す。

保存時の標準入口は `reconcile_metadata(MetadataReconcileRequest)` とする。

kleは `TargetResolutionKind` を次の状態として扱う。

- `Resolved`: targetは同じnodeに残っている
- `Moved`: fingerprintで移動先nodeを特定できた
- `Unresolved`: targetを復元できない
- `Conflict`: fingerprint候補が複数あり、KMEは1つに決めない

kleは `Unresolved` と `Conflict` のmetadata entryを削除しない。

## KUWまたはwidget境界

KUWが存在する場合、metadata表示UIはKUW側に置く。

KUWが未作成の場合でも、KatanAやkdpへ暗黙に共通widget責務を増やさない。代替する場合は、OpenSpecで明示的なwidget境界を作る。

metadata表示に渡す情報は次に限定する。

- `MetadataEntry.key`
- `MetadataEntry.payload`
- `TargetResolutionKind`
- 対象nodeの `KmeNodeId`
- 対象nodeの `SourceSpan`

## kcf

kcfはexport、PDF paging、LLM注釈、AST単位copy/editのmetadataをKME contractとして扱う。

kcfはKMEより先にmetadata schemaを増やさない。新しいmetadata用途が必要な場合は、KME側のfixtureとOpenSpecを先に更新する。

kcfが参照するpayload用途は、`tests/fixtures/metadata_uses.json` の次のkindをv0境界にする。

- `pdf-page`
- `llm-annotation`
- `ast-edit`

## KatanA

KatanA本体は統合順序とfixture authorityを管理する。

KMEのcanonical fixtureは `tests/fixtures/canonical/**` を正とする。KatanA側のfixtureが変わる場合、KME側のfixture同期、contract test、目視確認結果を同じchangeで更新する。

`v0.1.0` 公開後、downstreamは crates.io の `katana-markdown-engine = "0.1.0"` を基準に採用する。公開前の統合検証だけは、release branchまたはgit revisionを明示して扱う。

## KCF pending解除条件

kcfのpending解除は、次をすべて満たした後に行う。

- KME `v0.1.0` が公開されている
- KME public DTOとmetadata APIがこの文書どおり固定されている
- `just harness-up` の目視確認結果が記録されている
- KUWまたは明示的widget境界が定義されている
- kdpが `KmeDocument` をpreview inputとして採用できる
- kleが `MetadataReconcileRequest` / `MetadataReconcileResult` を保存時contractとして採用できる

## 禁止事項

- KME内部parser型をdownstream public APIへ出す
- downstreamで独自metadata schemaを作る
- unresolved metadataを保存時に削除する
- KCFがKMEより先にexport/paging metadata contractを固定する
- KatanA本体へ共通metadata widget責務を暗黙に吸収させる
