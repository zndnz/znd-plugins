# GIFベイヤーディザ出力

減色とベイヤーディザを掛けた、小さい GIF を書き出す。

- 元のパッケージ: [GIFベイヤーディザ出力](https://github.com/osakana4242/aviutl2-gif-bayer-dither-output)（作者: 三川おさかな、ライセンス: 0BSD）
- カタログ ID: `osakana4242.GifBayerDitherOutput`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:osakana4242.GifBayerDitherOutput/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| GIF（減色＋ベイヤーディザ） | `.gif` | 色数・ベイヤーの粗さ（0〜5）・フレームレート |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
