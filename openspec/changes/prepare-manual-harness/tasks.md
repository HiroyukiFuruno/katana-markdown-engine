# Tasks: prepare-manual-harness

## Definition of Ready

- [x] KME本体はlibrary-onlyを維持する方針である
- [x] release前に目視できる検証環境が必要である
- [x] KCU/KCFの `just harness-up` を参考にする方針である

## Tasks

- [ ] 1.1 harnessを公開crateへ含めない配置に決める
- [ ] 1.2 `just harness-up` の入口を追加する
- [ ] 1.3 Markdown本文とKME node一覧を表示する
- [ ] 1.4 選択nodeのsource range、line-column、raw snippet、fingerprintを表示する
- [ ] 1.5 metadata解決状態を表示する
- [ ] 1.6 代表fixtureごとの目視確認手順を文書化する
- [ ] 1.7 目視確認結果の記録方法を文書化する
- [ ] 1.8 package対象にharnessが混入しないことを確認する

## Definition of Done

- [ ] `just harness-up` で目視確認環境が起動する
- [ ] KME本体に製品binary targetが追加されていない
- [ ] fixtureごとの目視確認手順がある
- [ ] release前品質ゲートとして扱える

## Verification

- [ ] `scripts/openspec validate "prepare-manual-harness" --strict`
- [ ] `just harness-up`
- [ ] `cargo package --locked --allow-dirty --list` でharness混入がないことを確認する
