## Context

現状のmetadata解決結果は `Resolved`、`Moved`、`Unresolved` を持つ。

kleなどのdownstreamが保存時に必要とする状態をKME側で固定しないと、各repoが独自metadata schemaを作る危険がある。

## Goals

- metadata解決のpublic DTOをv0.1.0境界として固定する。
- `Conflict` を未定義のままdownstreamへ流さない。
- unresolved targetを削除せず保持する。

## Non-Goals

- metadataをMarkdown本文へ埋め込むこと。
- editor UIを実装すること。
- PDF pagingやLLM注釈の実処理をKMEで実装すること。

## Contract

KMEは保存前source、保存後source、metadata documentを受け取り、targetごとの解決状態を返す。

最低限の状態:

- `Resolved`: 直接解決できた
- `Moved`: 同じ意味のtargetへ再対応できた
- `Unresolved`: 再対応できなかったがmetadataは保持する
- `Conflict`: 複数候補や曖昧な再対応があり、自動決定できない

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
scripts/openspec validate "finalize-metadata-resolution-contract" --strict
just check
```
