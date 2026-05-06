## Context

KMEは製品UIを持たない。

それでも、文書モデル、source mapping、metadata解決は人間が確認できる形で表示できないと、release前の判断が難しい。

## Goals

- `just harness-up` で目視確認環境を起動する。
- Markdown本文とKME出力を同時に確認できる。
- metadata解決結果を確認できる。
- harnessを公開crateから分離する。

## Non-Goals

- 製品UIの実装。
- KME本体へのbinary target追加。
- HTML/PDF出力の実装。
- downstream UIの代替。

## Harness View

画面上では次を確認できるようにする。

- Markdown本文
- KMEが解釈したnode一覧
- 選択nodeのsource range
- 選択nodeのline-column
- raw snippet
- fingerprint
- metadata解決状態

これは手動品質ゲートであり、表示そのものを製品仕様にはしない。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
scripts/openspec validate "prepare-manual-harness" --strict
just harness-up
```
