# ImageCropExport

今のフレームの一部を切り抜いて PNG で書き出す。

- 元のパッケージ: [ImageCropExport](https://github.com/azurite581/AviUtl2-ImageCropExport)（作者: azurite、ライセンス: MIT）
- カタログ ID: `azurite.ImageCropExport`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:azurite.ImageCropExport/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| 今のフレームを切り抜いて画像に | `.png` | 左（px）・上（px）・幅（px）・高さ（px） |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
