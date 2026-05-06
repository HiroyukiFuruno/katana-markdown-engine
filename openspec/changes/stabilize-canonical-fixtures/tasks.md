# Tasks: stabilize-canonical-fixtures

## Definition of Ready

- [x] KME v0はKatanA現行挙動の踏襲を優先する方針である
- [x] canonical fixture候補が `openspec/project.md` に列挙されている

## Tasks

- [ ] 1.1 canonical fixtureの同期方法を決める
- [ ] 1.2 KatanA `sample.md` の主要node期待値を棚卸しする
- [ ] 1.3 KatanA `sample_basic.md` のalert期待値を棚卸しする
- [ ] 1.4 README badge列の期待値を棚卸しする
- [ ] 1.5 description list fixtureを正本fixtureとして固定する
- [ ] 1.6 node種別、source range、raw snippet、fingerprintをテストへ追加する
- [ ] 1.7 絶対パスに依存しないfixture運用を文書化する

## Definition of Done

- [ ] KatanA現行fixtureの主要構造がKME nodeとして固定されている
- [ ] source range、raw snippet、fingerprintの回帰テストがある
- [ ] downstreamが参照するfixture authorityが明確である

## Verification

- [ ] `scripts/openspec validate "stabilize-canonical-fixtures" --strict`
- [ ] `just check`
