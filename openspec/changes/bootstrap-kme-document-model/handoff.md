# KME Cross-repo Handoff

## 結論

KMEで実現すべきことは、MarkdownをKatanA ecosystem共通の文書モデルとして解釈することである。

KMEはHTML変換器ではない。KMEはpreview、editor、export、KatanA統合が共有する文書構造、source mapping、metadata target解決の正本である。

## 現時点の判断

全体計画はまだ完全には安定していない。KMEの実装は進められるが、downstream連携は次の順序を守る。

1. P0 `katana-ast-lint`
2. P1 `katana-markdown-engine`
3. P2 `katana-ui-widget`
4. P3 `katana-document-preview`
5. P3 `katana-language-editor`
6. P3 `katana-canvas-forge`
7. P3 `katana` integration

KCFは、KME / KAL / KUW / preview / editor側の分離計画が安定するまでpendingである。

## Repository別責務

### katana-markdown-engine

KMEは次を所有する。

- Markdown document model
- source range
- line-column
- raw snippet
- stable node id
- text fingerprint
- metadata schema
- target resolution
- parser adapter boundary

KMEは次を所有しない。

- preview UI
- editor save UI
- export rendering
- Floem widget
- KatanA workspace state

### katana-ast-lint

KALは分離repoの共通AST lint gateを所有する。

KMEは `katana-ast-lint = "0.1.0"` を使う。KME固有の一時lintで代替しない。

### katana-ui-widget

KUWは未作成である。これは全体計画のリスクである。

KUWは次を所有する予定。

- Floem共通UI部品
- metadata badge
- unresolved metadata表示
- toolbar
- tabs
- copy/edit affordance

KUWがない状態でkdpやKatanA本体へUI部品を増やしすぎない。

### katana-document-preview

kdpはKME public DTOを入力にしてFloem previewを表示する。

kdpはKME parserを再実装しない。KME内部parser型へ依存しない。

### katana-language-editor

kleは保存直後のmetadata同期を担う。

kleはold source、new source、metadataをKMEへ渡し、resolved / moved / conflicted / unresolvedを受け取る。

kleはmetadataを削除しない。unresolvedは保持する。

### katana-canvas-forge

kcfは現時点ではpendingである。

kcfはKME文書モデルやmetadata schemaを先行定義しない。KMEとKUWの境界が固まった後に、export / PDF paging / output quality gateへ接続する。

### katana

KatanA本体はfixture authorityとintegrationを所有する。

KatanAはKME、kdp、kle、kcf、KUWを統合するが、KME内部parser型へ依存しない。

## 次セッションの最初の作業

1. KMEの次実装は、canonical fixture同期、table/grid cell source range、emoji nodeの順に進める。
2. metadata conflict DTOを追加し、kleが独自状態で代替しないようにする。
3. parser strategyを評価し、OS依存emojiやKatanA現行fixtureを壊すparser候補を除外する。
4. KME public DTOとmetadata APIが固定されたら、kdp、kle、KUW、kcfへhandoffする。

## 検証結果

2026-05-06 に以下を確認済みである。

- `katana-markdown-engine`: `scripts/openspec validate "bootstrap-kme-document-model" --strict`
- `katana`: `scripts/openspec validate "establish-kme-markdown-platform" --strict`
- `katana`: `scripts/openspec validate "adopt-kme-in-katana" --strict`
- `katana`: `scripts/openspec validate "extract-katana-ui-widget" --strict`
- `katana-document-preview`: `npx -y @fission-ai/openspec validate "adopt-kme-preview-model" --strict`
- `katana-language-editor`: `npx -y @fission-ai/openspec validate "sync-kme-metadata-on-save" --strict`
- `katana-canvas-forge`: `npx -y @fission-ai/openspec validate "v0-1-2-export-css-debug" --strict`
- `katana-ast-lint`: `scripts/openspec validate "shared-ast-lint" --strict`
- `katana-markdown-engine`: `just check`
- `katana-markdown-engine`: `cargo package --locked --allow-dirty`
- `katana-markdown-engine`: `cargo publish --dry-run --locked --allow-dirty`

## やってはいけないこと

- KMEより先にkcfでmetadata schemaを作る。
- kdpやkleでKME parserを再実装する。
- KME public DTOにthird-party parser ASTを漏らす。
- KUW未作成のままKatanA本体に共通UI部品を増やす。
- OpenSpecが薄いまま別セッションへ渡す。
