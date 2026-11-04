# MFOutput

MP4・M4A・WMV・WMA を手軽に書き出す。

- 元のパッケージ: [MFOutput](https://github.com/MonogoiNoobs/aviutl2-media-foundation-output-plugin)（作者: 依神紫苑、ライセンス: MIT）
- カタログ ID: `MonogoiNoobs.MFOutput`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:MonogoiNoobs.MFOutput/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| MP4（H.264＋AAC、手軽） | `.mp4` | 映像ビットレート（kbps）・音声ビットレート（kbps） |
| M4A（AAC） | `.m4a` | 音声ビットレート（kbps） |
| WMV | `.wmv` | 映像ビットレート（kbps）・音声ビットレート（kbps） |
| WMA | `.wma` | 音声ビットレート（kbps） |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
