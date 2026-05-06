# katana-markdown-engine

`katana-markdown-engine`（KME）は、KatanA ecosystem のMarkdown文書モデル、外部メタデータ（metadata）、位置解決を担うlibraryです。

KMEはHTML変換器ではありません。KatanAのプレビュー（preview）、編集機能（editor）、出力（export）が同じ文書解釈を共有するための中核です。

分離優先順位ではP1です。P0 `katana-ast-lint` の共通品質ゲートを前提にして進めます。

## 初期方針

- KME v0はCommonMark完全準拠より、現在KatanAで実現できている挙動の踏襲を優先します。
- 主fixtureは `/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md` です。
- README badge、alert、description listも必須fixtureにします。
- metadataはMarkdown本文へ埋め込まず、外部ファイルとして扱います。
- KMEはkcf、kdp、KatanA、editorへ依存しません。
- KME固有の一時lintを作らず、共通AST lintを品質ゲートにします。
