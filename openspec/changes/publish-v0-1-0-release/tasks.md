# Tasks: publish-v0-1-0-release

## Definition of Ready

- [ ] `stabilize-release-readiness-gates` が完了している
- [ ] `stabilize-canonical-fixtures` が完了している
- [ ] `finalize-metadata-resolution-contract` が完了している
- [ ] `lock-parser-adapter-strategy` が完了している
- [ ] `prepare-manual-harness` が完了している
- [ ] `prepare-downstream-handoff-contract` が完了している
- [x] 初回公開versionは `v0.1.0` である
- [x] 既定ブランチは `master` 維持である

## Tasks

- [ ] 1.1 `v0.1.0` release runbookを作成する
- [ ] 1.2 `CARGO_REGISTRY_TOKEN` の登録手順を明記する
- [ ] 1.3 `release/v0.1.0` から `master` へのrelease PR手順を明記する
- [ ] 1.4 GitHub Release作成手順を実装または文書化する
- [ ] 1.5 crates.io publish手順を実装または文書化する
- [ ] 1.6 公開済みversionを再利用しない失敗時方針を明記する
- [ ] 1.7 公開後verify手順を実装または文書化する
- [ ] 1.8 release後のbranch hygiene手順を明記する

## Definition of Done

- [ ] 全OpenSpec changeのtaskが完了している
- [ ] `just release-check` が成功している
- [ ] `v0.1.0` GitHub Releaseが作成されている
- [ ] `v0.1.0` がcrates.ioへ公開されている
- [ ] 公開後verifyが成功している
- [ ] release後のbranch hygieneが完了している

## Verification

- [ ] `scripts/openspec validate "publish-v0-1-0-release" --strict`
- [ ] `just release-check`
- [ ] 公開後verify
