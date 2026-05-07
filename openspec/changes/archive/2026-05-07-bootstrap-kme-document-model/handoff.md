# KME Cross-repo Handoff

## 結論

KMEで実現すべきことは、MarkdownをKatanA ecosystem共通の文書モデルとして解釈することである。

KMEはHTML変換器ではない。KMEはviewer、editor、export、KatanA統合が共有する文書構造、source mapping、metadata target解決の正本である。

KMEは同期制御を持たない。editor-viewer同期はKatanAが担い、KatanAがviewerやeditorへ命令する。

## 現時点の判断

全体計画はまだ完全には安定していない。KMEの実装は進められるが、downstream連携は次の順序を守る。

1. P0 `katana-ast-lint`
2. P1 `katana-markdown-engine`
3. P2 `katana-ui-widget`
4. P3 `katana-document-viewer`
5. P3 `katana-language-editor`
6. P3 `katana-canvas-forge`
7. P3 `katana` integration

KCFは外部描画へ責務を縮小する。既存exportはKDV移譲まで維持するが、新規export計画はKDVへ移す。

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

- viewer UI
- editor save UI
- export rendering
- Floem widget
- KatanA workspace state
- editor-viewer同期制御

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

### katana-document-viewer

KDVはKME public DTOを入力にしてviewer表示、hit-test、HTML/PDF/PNG/JPG exportを担う。

KDVはKME parserを再実装しない。KME内部parser型へ依存しない。

### katana-language-editor

kleは保存直後のmetadata同期を担う。

kleはold source、new source、metadataをKMEへ渡し、resolved / moved / conflicted / unresolvedを受け取る。

kleはmetadataを削除しない。unresolvedは保持する。

### katana-canvas-forge

kcfは現時点ではpendingである。

kcfはMermaid、Draw.io、PlantUML、mathなどの外部描画を担う。

既存exportはKDV実装完了まで維持する。KDVにviewer/exportが入った後、KCF側のHTML/PDF/PNG/JPG exportはKDVへ移譲して削除する。

### katana

KatanA本体はfixture authorityとintegrationを所有する。

KatanAはKME、KDV、KLE、KCF、KUWを統合するが、KME内部parser型へ依存しない。

KatanAはeditor-viewer同期のcoordinatorである。KMEのnode id、source range、line-column、raw snippet、fingerprintを使って対応付けを行い、viewerまたはeditorへ命令する。

## v0.1.0 release PR後の作業

1. `release/v0.1.0` PRを `master` へmergeする。
2. `docs/release-runbook.md` に沿ってGitHub Releaseとcrates.io公開を行う。
3. 公開後verifyを実行する。
4. release後のbranch hygieneを実行する。

downstream handoffの正本は `docs/downstream-handoff.md` とする。

## 検証結果

2026-05-06 に以下を確認済みである。

- `katana-markdown-engine`: `scripts/openspec validate "bootstrap-kme-document-model" --strict`
- `katana`: `scripts/openspec validate "establish-kme-markdown-platform" --strict`
- `katana`: `scripts/openspec validate "adopt-kme-in-katana" --strict`
- `katana`: `scripts/openspec validate "extract-katana-ui-widget" --strict`
- `katana-document-viewer`（現repo名は `katana-document-preview`）: `npx -y @fission-ai/openspec validate "adopt-kme-preview-model" --strict`
- `katana-language-editor`: `npx -y @fission-ai/openspec validate "sync-kme-metadata-on-save" --strict`
- `katana-canvas-forge`: `npx -y @fission-ai/openspec validate "v0-1-3-export-css-debug" --strict`
- `katana-ast-lint`: `scripts/openspec validate "shared-ast-lint" --strict`
- `katana-markdown-engine`: `just check`
- `katana-markdown-engine`: `cargo package --locked --allow-dirty`
- `katana-markdown-engine`: `cargo publish --dry-run --locked --allow-dirty`

## やってはいけないこと

- KMEより先にkcfでmetadata schemaを作る。
- KDVやKLEでKME parserを再実装する。
- KME、KDV、KLEへeditor-viewer同期制御を持たせる。
- KME public DTOにthird-party parser ASTを漏らす。
- KUW未作成のままKatanA本体に共通UI部品を増やす。
- OpenSpecが薄いまま別セッションへ渡す。
