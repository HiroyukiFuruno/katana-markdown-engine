# Tasks: prepare-downstream-handoff-contract

## Definition of Ready

- [ ] KME public DTOとmetadata APIのv0.1.0境界が固定されている
- [x] downstreamがKME内部parser型へ依存しない方針である

## Tasks

- [ ] 1.1 kdpへ渡すpreview inputとhit-test metadataを定義する
- [ ] 1.2 kleへ渡すsave-time metadata sync contractを定義する
- [ ] 1.3 KUWまたは明示的widget境界へ渡すmetadata/unresolved表示DTOを定義する
- [ ] 1.4 kcfへ渡すexport/paging metadata contractを定義する
- [ ] 1.5 KatanA統合で必要なfixture authorityとdependency version policyを定義する
- [ ] 1.6 kcf pending解除条件を文書化する

## Definition of Done

- [ ] downstream repoがKME内部parser型へ依存しない
- [ ] downstream repoが独自metadata schemaを作らない
- [ ] KCF pending解除条件が明確である

## Verification

- [ ] `scripts/openspec validate "prepare-downstream-handoff-contract" --strict`
- [ ] `just check`
