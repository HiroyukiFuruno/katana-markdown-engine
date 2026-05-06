# Tasks: bootstrap-kme-document-model

## 1. Repository Baseline

### Definition of Ready

- [x] `katana/openspec/changes/establish-kme-markdown-platform` でKMEの責務が確定している
- [x] P0 `katana-ast-lint` の共通rule、違反形式、repository adapter方針が利用可能である

### Tasks

- [x] 1.1 KME crate構成を決める
- [x] 1.2 public DTOと内部parser moduleの境界を決める
- [x] 1.3 kcf/kdp/KatanA/editorへ依存しないことを検証する
- [x] 1.4 共通AST lintのKME adapter方針を決める

### Definition of Done

- [x] KMEが単独repositoryとして成立している
- [x] public contractに既存parser ASTが漏れていない
- [x] KME固有の一時AST lintや除外設定を品質ゲートにしていない

## 2. Document Model

### Definition of Ready

- [x] fixture contractが確定している

### Tasks

- [x] 2.1 `sample.md` の主要nodeをモデル化する
- [x] 2.2 README badgeをHTML block / inline image-link構造としてモデル化する
- [x] 2.3 alertとdescription listをモデル化する
- [ ] 2.4 table/gridに行、列、cell、alignment、source rangeを持たせる
- [ ] 2.5 emojiを削除せず、Unicodeとshortcode情報を保持する

### Definition of Done

- [x] fixtureごとのnode種別、source range、raw snippetがテストで固定されている

## 3. Metadata Target Resolution

### Definition of Ready

- [x] metadata schema案が定義済みである

### Tasks

- [x] 3.1 `README.md.metadata.json` のschemaを定義する
- [x] 3.2 file path、node id、byte range、line-column、fingerprint、前後文脈をtargetに含める
- [x] 3.3 旧本文と新本文からtarget移動を判定する
- [x] 3.4 復元できないtargetをunresolvedとして返す

### Definition of Done

- [x] 保存時metadata更新に必要なAPIがkleから利用できる
- [x] unresolved targetが削除されず保持される

## 4. Final Verification

- [x] 4.1 KME fixture testsを実行する
- [x] 4.2 共通AST lintのKME adapterで検査できることを確認する
- [x] 4.3 `scripts/openspec validate "bootstrap-kme-document-model" --strict` を実行する

## 5. Remaining Follow-up

- [ ] 5.1 table/gridのcell単位source rangeを追加する
- [ ] 5.2 emoji shortcodeとUnicode保持の専用nodeを追加する
- [ ] 5.3 canonical fixtureをKatanA本体fixtureの完全コピーまたはsubmodule/同期手段で固定する
- [ ] 5.4 release workflowを整備する
