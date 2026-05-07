## Context

KMEはP1の分離対象である。

downstream連携は、KME public DTOとmetadata APIが固定されてから始める。

## Goals

- downstreamごとの受け渡し条件を固定する。
- KME内部parser型への依存を禁止する。
- 独自metadata schemaの発生を防ぐ。
- kcf pending解除条件を明確にする。

## Non-Goals

- downstream repositoryの実装。
- kdp、kle、kcf、KatanA側のOpenSpec更新。
- UI widgetの実装。

## Handoff Targets

- kdp: KME public DTOをpreview inputとして使う。
- kle: 保存時metadata同期でKME APIを使う。
- kcf: KME文書モデルとmetadata schemaが固定された後にexport/pagingへ接続する。
- KatanA: fixture authorityと統合順序を管理する。

## Decision

受け渡し境界の正本は `docs/downstream-handoff.md` とする。

downstreamが使ってよい入口は、`parse_markdown`、`reconcile_metadata`、`reconcile_metadata_targets` に限定する。

kcf pending解除は、KME `v0.1.0` 公開、manual harness確認、KUWまたは明示的widget境界、kdp/kleの採用準備が揃った後に行う。

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
scripts/openspec validate "prepare-downstream-handoff-contract" --strict
just check
```
