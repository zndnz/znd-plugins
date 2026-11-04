# znd-plugins

[AviUtl2 Catalog](https://a2cw.sevenc7c.com) に載っている AviUtl ExEdit2 のプラグイン・スクリプトのうち、
ソースが公開されていて組み込めるライセンス（MIT・MIT-0・BSD・Apache-2.0・CC0・Unlicense など）のものを、
[znd](../znd) のプラグインとして再実装したもの。

全パッケージの状況（移植済み・対象外・ソース非公開・ライセンス不適合）は [CATALOG.md](CATALOG.md)。

## 構成

| 場所 | 内容 |
|---|---|
| `plugins/<カタログID>/` | 1 パッケージ = 1 フォルダ。znd のプラグインパッケージ（`plugin.json`）そのもの。元のライセンス文（`LICENSE`）と、元との違い（`README.md`）を同梱する |
| `plugins/<カタログID>/native/` | ネイティブのエフェクト（Rust、znd プラグイン ABI 3） |
| `sdk/` | Rust でエフェクトを書くための SDK（`plugin!` マクロ、画像・図形・色・ノイズの部品） |
| `harness/` | すべてのパッケージを znd 本体に読み込んで検証するテスト |
| `catalog/` | カタログのデータの写しと、パッケージごとの移植状況 |
| `tools/` | ビルド・検証・雛形作成の道具 |

## 使い方

```bash
tools/build.sh
```

`plugins/` の中のフォルダを znd のプラグインフォルダ（macOS: `~/Library/Application Support/znd/plugins`）にコピーするか、
`ZND_PLUGIN_PATH` に `plugins/` を指定して znd を起動する。

```bash
ZND_PLUGIN_PATH=$PWD/plugins cargo run --release -p znd-app --manifest-path ../znd/Cargo.toml
```

## 検証

```bash
tools/check.sh
```

デバッグビルドしたライブラリを置き、`harness` のテストで、すべてのパッケージが安全性スキャンで止められずに警告なく読み込めること、
すべてのエフェクトが既定値で有限の画像（音声）を返すこと、すべてのスクリプトが評価できることを確かめる。
プラグインごとの動作のテストは `harness/tests/` と各 `native/src` にある。

## AviUtl2 との主な違い

- **座標と単位**: AviUtl のスクリプトはオブジェクトの中心が原点・px 単位。znd のエフェクトはバッファの左上が原点で、
  px 単位のパラメータはプレビュー解像度に合わせて縮尺される（移植では中心からの位置に直している）。
- **色**: znd の画像はリニアライト・プリマルチプライドの float。AviUtl の 0〜255 の表示値で計算する処理は、表示値に直して計算している。
- **オブジェクトの大きさ**: AviUtl の `obj.effect("領域拡張")` などで画像を広げる処理は、znd の ABI 3 の `set_output` で行う。
  平面・素材・プリコンポのレイヤーでは広がったまま合成され、テキスト・シェイプ（コンポ全体のバッファ）では画面内に収まる。
- **カスタムオブジェクト**: AviUtl の「カスタムオブジェクト」（図形を作るスクリプト）は、平面レイヤーに掛けるエフェクトとして実装している
  （平面の色は無視して、エフェクトが絵を作る）。
- **シーンチェンジ・カメラ制御**: 他のレイヤーの絵は LAYER パラメータで受け取る。
- **汎用プラグイン**（タイムライン操作など）は、パッケージのコマンド・パネル（JavaScript）として実装している。
- **出力プラグイン**は、パッケージの書き出し形式（FFmpeg の引数の組）として実装している。

## ライセンス

各パッケージは元のライセンスに従う（各フォルダの `LICENSE`）。`sdk/`・`harness/`・`tools/` は MIT。
