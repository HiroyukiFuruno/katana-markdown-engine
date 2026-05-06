## Context

初期parserはpublic DTOとsource mappingの契約を固定するための最小実装である。

後続でparser engineを差し替えても、public DTOとtest contractを壊してはならない。

## Goals

- parser adapter境界を固定する。
- third-party parser ASTをpublic contractへ出さない。
- KatanA現行挙動を表すparse contractをテストで固定する。

## Non-Goals

- CommonMark全仕様の実装。
- renderer実装。
- downstream統合。

## Evaluation Policy

parser候補は次の条件で評価する。

- public DTOへ内部AST型が漏れない
- source rangeとraw snippetを保持できる
- README badge、alert、description list、table、diagram、mathを壊さない
- Unicode emojiとshortcode emojiを削除しない
- OSやfontに依存してKMEの文書モデルが変わらない

## Verification

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
scripts/openspec validate "lock-parser-adapter-strategy" --strict
just check
```
