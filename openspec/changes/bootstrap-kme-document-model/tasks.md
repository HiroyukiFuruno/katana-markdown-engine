# Tasks: bootstrap-kme-document-model

## 0. 全体見直し

### Definition of Ready

- [x] KMEがMarkdown仕様の中核であり、preview、editor、exportがKME文書モデルを消費する方針が確認済みである
- [x] KCFはKME / KAL / KUW / preview / editor計画が安定するまでpending扱いにする方針が確認済みである
- [x] 親OpenSpec `katana/openspec/changes/establish-kme-markdown-platform` に、KMEを出発点にした実装順序とrepo別DoR/DoDが反映されている
- [x] 周辺repoのOpenSpecに、KME完了前に進めてよい範囲とpending条件が反映されている

### Tasks

- [x] 0.1 KMEの責務を「HTML変換器ではなく文書モデルの正本」として再確認する
- [x] 0.2 KCF v0.1.2を他repo安定待ちのpendingとして明記する
- [x] 0.3 KMEから見たcross-repo実装順序をhandoffに固定する
- [x] 0.4 `katana-ui-widget` が未作成であることをリスクとして明記する
- [x] 0.5 kdp / kle / kcf / KatanA統合のDoRがKME完了条件を参照しているか確認する

### Definition of Done

- [x] 別セッションがKME OpenSpecだけを読んでも、次に何を実装するか判断できる
- [x] 親計画とKME計画の順序が矛盾していない
- [x] KCFがKMEより先に独自metadataや独自文書モデルを作らないことが明記されている

## 1. Repository Baseline

### Definition of Ready

- [x] `katana/openspec/changes/establish-kme-markdown-platform` でKMEの責務が定義されている
- [x] P0 `katana-ast-lint` v0.1.0 がcrates.ioで利用可能である

### Tasks

- [x] 1.1 KME crate構成を決める
- [x] 1.2 public DTOと内部parser moduleの境界を決める
- [x] 1.3 kcf/kdp/KatanA/editorへ依存しないことを検証する
- [x] 1.4 `katana-ast-lint = "0.1.0"` をKME品質ゲートへ接続する
- [x] 1.5 `just check`、`lefthook`、CI、repo-local skillを横展開する
- [x] 1.6 GitHub repositoryを作成し、`master` をdefault branchにする
- [ ] 1.7 KML相当のbranch protectionを `release-preflight.yml` push後に再設定する

### Definition of Done

- [x] KMEが単独repositoryとして成立している
- [x] public contractに既存parser ASTが漏れていない
- [x] KME固有の一時AST lintや除外設定を品質ゲートにしていない
- [x] `just check` がPASSする

## 2. Document Model Bootstrap

### Definition of Ready

- [x] 初期fixture contractが決まっている
- [ ] canonical fixtureの取り込み方法が決まっている

### Tasks

- [x] 2.1 `sample.md` の主要nodeを軽量fixtureでモデル化する
- [x] 2.2 README badgeをHTML block / badge rowとしてモデル化する
- [x] 2.3 alertとdescription listをモデル化する
- [x] 2.4 tableに行、列、cell、alignmentを持たせる
- [ ] 2.5 table/gridのcell単位source rangeを持たせる
- [ ] 2.6 emojiを削除せず、Unicodeとshortcode情報を保持する専用nodeを追加する
- [ ] 2.7 footnote、image、link、HTML inline、math inlineをKatanA現行仕様として棚卸しする
- [ ] 2.8 `katana/assets/fixtures/sample.md`、`sample_basic.md`、`katana/README.md` badgeをcanonical fixtureとして同期する

### Definition of Done

- [ ] KatanA現行fixtureの主要構造がKME nodeとして固定されている
- [ ] node種別、source range、raw snippet、fingerprintがfixture testで固定されている
- [ ] KMEのpublic DTOだけでkdp / kle / kcf / KatanAが参照できる

## 3. Metadata Target Resolution

### Definition of Ready

- [x] metadataをMarkdown本文へ埋め込まない方針が決まっている

### Tasks

- [x] 3.1 `README.md.metadata.json` のschema初期案を定義する
- [x] 3.2 file path、node id、byte range、line-column、fingerprint、前後文脈をtargetに含める
- [x] 3.3 旧本文と新本文からtarget移動を判定する
- [x] 3.4 復元できないtargetをunresolvedとして返す
- [ ] 3.5 conflict状態をpublic DTOとして追加する
- [ ] 3.6 PDFページング、LLM注釈、AST単位copy/editをmetadata用途としてfixture化する
- [ ] 3.7 editor保存時に必要なrequest/result DTOをkleがそのまま使える形へ固定する

### Definition of Done

- [ ] 保存時metadata更新に必要なAPIがkleから利用できる
- [ ] unresolved targetが削除されず保持される
- [ ] conflictが未定義のままdownstreamへ流れない

## 4. Parser Strategy

### Definition of Ready

- [ ] KatanA現行fixtureとbadge fixtureの必須node一覧がある

### Tasks

- [ ] 4.1 現行の初期parserを維持するか、parser engineを差し替えるかを評価する
- [ ] 4.2 候補parserがOS依存emojiを壊さないか確認する
- [ ] 4.3 parser内部型をpublic DTOへ出さないadapter境界を固定する
- [ ] 4.4 table/grid、HTML badge、alert、description list、footnote、diagram、mathのparse contractをtestで固定する

### Definition of Done

- [ ] parser engineを差し替えてもpublic DTOが壊れない
- [ ] KatanA現行挙動を再現できないparserを選ばない

## 5. Cross-repo Handoff

### Definition of Ready

- [ ] KME public DTOとmetadata APIのv0境界が固定されている

### Tasks

- [ ] 5.1 kdpへ渡すpreview inputとhit-test metadataを定義する
- [ ] 5.2 kleへ渡すsave-time metadata sync contractを定義する
- [ ] 5.3 KUWへ渡すmetadata/unresolved表示DTOを定義する
- [ ] 5.4 kcfへ渡すexport/paging metadata contractを定義する
- [ ] 5.5 KatanA統合で必要なfixture authorityとdependency version policyを定義する

### Definition of Done

- [ ] downstream repoがKME内部parser型へ依存しない
- [ ] downstream repoが独自metadata schemaを作らない
- [ ] KCF pending解除条件が明確である

## 6. Verification

- [x] 6.1 `just check` を実行する
- [x] 6.2 KME fixture testsを実行する
- [x] 6.3 KAL AST lintのKME adapterで検査できることを確認する
- [x] 6.4 `scripts/openspec validate "bootstrap-kme-document-model" --strict` を実行する
- [x] 6.5 親OpenSpecと周辺repo OpenSpecを再検証する
