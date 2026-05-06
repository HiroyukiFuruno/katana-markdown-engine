# Tasks: finalize-metadata-resolution-contract

## Definition of Ready

- [x] metadataをMarkdown本文へ埋め込まない方針である
- [x] `Resolved`、`Moved`、`Unresolved` の初期DTOが存在する

## Tasks

- [ ] 1.1 `Conflict` 状態をpublic DTOとして追加する
- [ ] 1.2 conflict判定の最小条件を定義する
- [ ] 1.3 editor保存時に必要なrequest/result DTOを固定する
- [ ] 1.4 unresolved targetを削除しない回帰テストを追加する
- [ ] 1.5 conflictをdownstreamへ未定義状態で流さない回帰テストを追加する
- [ ] 1.6 PDFページング、LLM注釈、AST単位copy/editのmetadata用途をfixture化する

## Definition of Done

- [ ] kleが保存時metadata同期に使うAPI境界が固定されている
- [ ] unresolved metadataが削除されない
- [ ] conflictがpublic contractとして扱える

## Verification

- [ ] `scripts/openspec validate "finalize-metadata-resolution-contract" --strict`
- [ ] `just check`
