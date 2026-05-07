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

## Decision

`v0.1.0` では、開発用のブラウザGUIとしてharnessを提供する。

配置は `tools/manual-harness` とし、KME本体へ製品binary targetは追加しない。公開crateには `tools/**` を含めない。

目視確認結果は `docs/release-readiness/<version>-manual-harness.md` に記録する。

既定表示対象は `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` とする。存在しない環境ではKME内のcanonical fixtureへfallbackする。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
scripts/openspec validate "prepare-manual-harness" --strict
just harness-check
just harness-up /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md
cargo package --locked --allow-dirty --list
```
