## Why

KMEはlibrary-onlyだが、release前には人間がKME出力を目視できる環境が必要である。

KCUやKCFでは `just harness-up` を手動品質ゲートとして扱っている。KMEでも同様に、開発用harnessで文書モデルとmetadata解決結果を確認できるようにする。

## What Changes

- `just harness-up` を標準入口として追加する。
- Markdown本文、KME node一覧、source range、raw snippet、fingerprint、metadata解決状態を目視できる開発用環境を用意する。
- KME本体はlibrary-onlyを維持する。
- harnessのUI依存やbinaryが公開crateへ混入しない境界を固定する。
- fixtureごとの目視確認手順と確認結果の記録方法を用意する。

## Impact

- `tools/**` など公開crate外の開発用harness
- `Justfile`
- release前検証手順
- OpenSpec tasks
