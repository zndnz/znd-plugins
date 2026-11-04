# svtAV1guiEx

SVT-AV1（libsvtav1）で AV1 に書き出す。

- 元のパッケージ: [svtAV1guiEx](https://github.com/rigaya/svtAV1guiEx)（作者: rigaya、ライセンス: MIT）
- カタログ ID: `rigaya.svtAV1guiEx`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:rigaya.svtAV1guiEx/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| SVT-AV1（AV1） | `.mp4` | 画質 CRF・プリセット・音声ビットレート（kbps）・追加の引数 |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
