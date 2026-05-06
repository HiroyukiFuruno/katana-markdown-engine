## Why

KMEのpublic DTOとmetadata APIが固定されたら、kdp、kle、kcf、KatanAへ受け渡す必要がある。

受け渡し条件が曖昧だと、downstreamがKME parserを再実装したり、独自metadata schemaを作ったりする危険がある。

## What Changes

- kdp、kle、kcf、KatanAが参照するKME public DTO境界をまとめる。
- KME内部parser型へ依存しない条件を明記する。
- downstreamが独自metadata schemaを作らない条件を明記する。
- kcf pending解除条件をKME側から固定する。

## Impact

- `docs/roadmap.md`
- `openspec/changes/**/handoff.md` 相当の受け渡し文書
- downstream側OpenSpecのDoR
