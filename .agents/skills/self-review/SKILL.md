---
name: self-review
description: katana-markdown-engine の差分をcommitやhandoff前に自己レビューする。
---

# Self Review

現在の差分を対象に、次の観点で確認します。

## 範囲確認

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
git status --short
git diff --stat
```

## 設計確認

- KMEがKatanA、kcf、kdp、kle、UI frameworkへ依存していない。
- public contractに第三者parser ASTが漏れていない。
- metadataをMarkdown本文へ埋め込んでいない。
- rendererやexporterの責務をKMEへ持ち込んでいない。

## 品質確認

```bash
just check
```

FAILのままcommit、push、handoffへ進めません。
