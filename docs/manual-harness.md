# Manual Harness

## 結論

`just harness-up` は、KMEの出力をrelease前に人間がGUIで目視確認するための開発用入口である。

これは製品CLIでも製品UIでもない。KME本体はlibrary-onlyを維持する。

## 画面で確認するもの

ブラウザ画面に次を表示する。

- Markdown入力
- KME node一覧
- 選択nodeのsource range
- 選択nodeのline-column
- 選択nodeのraw snippet
- 選択nodeのfingerprint
- metadata解決状態

metadata解決状態は `Resolved`、`Moved`、`Unresolved`、`Conflict` を同じ画面で確認する。

## 起動方法

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
just harness-up
```

既定では次のKatanA fixtureを表示する。

```text
/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md
```

別のMarkdownを確認する場合は、引数で指定する。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
just harness-up /Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md
```

## fixture確認手順

1. Markdown入力にKatanA `sample.md` の内容が表示されることを確認する。
2. node一覧にheading、alert、table、description list、diagram、math、emojiが表示されることを確認する。
3. 選択nodeにsource range、line-column、raw snippet、fingerprintが表示されることを確認する。
4. metadata解決状態に `Resolved`、`Moved`、`Unresolved`、`Conflict` が表示されることを確認する。
5. 確認結果を `docs/release-readiness/<version>-manual-harness.md` に記録する。

## package確認

公開crateへharnessを混入させないため、release前に次を実行する。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-markdown-engine
cargo package --locked --allow-dirty --list
```

出力に `tools/manual-harness` が含まれていないことを確認する。
