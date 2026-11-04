# ffmpegOut

FFmpeg の映像・音声の引数を自分で書いて書き出す（入れ物は MP4・MKV・MOV・WebM）。

- 元のパッケージ: [ffmpegOut](https://github.com/rigaya/ffmpegOut)（作者: rigaya、ライセンス: MIT）
- カタログ ID: `rigaya.ffmpegOut`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:rigaya.ffmpegOut/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| ffmpegOut（MP4・引数を指定） | `.mp4` | 映像の引数・音声の引数 |
| ffmpegOut（MKV・引数を指定） | `.mkv` | 映像の引数・音声の引数 |
| ffmpegOut（MOV・引数を指定） | `.mov` | 映像の引数・音声の引数 |
| ffmpegOut（WEBM・引数を指定） | `.webm` | 映像の引数・音声の引数 |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
