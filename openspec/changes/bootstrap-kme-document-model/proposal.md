## Why

KatanAのMarkdown previewとexportは、既存libraryのASTやrenderer都合に引っ張られ、表（table/grid）、badge、脚注、alert、絵文字、コード同期、PDFページング、LLM注釈を同じ仕様で扱いにくい。

KMEはMarkdown文書モデルを自前で所有し、preview、editor、exportが同じ文書解釈を共有できるようにする。

ただし、repository分離の優先順位ではP1とし、P0 `katana-ast-lint` の共通品質ゲートを先に前提化する。

## What Changes

- KME文書モデルを定義する
- metadata schemaを定義する
- metadata targetの位置解決APIを定義する
- KatanA現行fixtureをモデル化する
- diagram/math/emojiは描画実装ではなく構造ノードとrenderer interfaceとして扱う
- 共通AST lintをKMEの品質ゲートとして採用する

## Capabilities

### New Capabilities

- `kme-document-model`: Markdown文書をKME独自モデルとして表現する
- `kme-metadata-target`: 外部metadata targetを文書モデルへ解決する

## Impact

- 新規crate構成の定義
- fixture contractの追加
- `katana-ast-lint`: 共通AST lint gate
- kdp/kle/kcf/KatanAが参照するpublic DTOの定義
