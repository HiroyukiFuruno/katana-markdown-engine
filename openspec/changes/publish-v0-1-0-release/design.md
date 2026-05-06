## Context

KME `Cargo.toml` のversionは `0.1.0` である。

初回リリースでは、public DTO、metadata API、parser adapter境界、manual harness、downstream handoff条件が完了している必要がある。

## Goals

- `v0.1.0` の公開手順を固定する。
- 全change完了後にだけ公開する。
- crates.io公開後の検証を行う。
- KML同様のbranch戦略をKME向けに正式採用する。

## Non-Goals

- 全change完了前の先行公開。
- 製品CLIの追加。
- KMLの多チャネル配布の導入。

## Branch Policy

`v0.1.0` 以降のbranch戦略:

- 既定ブランチ: `master`
- release統合ブランチ: `release/vX.Y.Z`
- 補助ブランチ: `feature/vX.Y.Z-<short-slug>`
- release PR: `release/vX.Y.Z` から `master`
- `fix/vX.Y.Z-*`、`chore/vX.Y.Z-*`、`release-vX.Y.Z` は使わない
- merge時に `--admin` は使わない
- merge後にbranch hygieneを行う

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
scripts/openspec validate "publish-v0-1-0-release" --strict
just release-check
```
