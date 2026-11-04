# Rusty FFmpeg Output

FFmpeg で動画を書き出す（引数を自分で決める）。

- 元のパッケージ: [Rusty FFmpeg Output](https://github.com/sevenc-nanashi/aviutl2-rs)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-rs.rusty_ffmpeg`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:sevenc-nanashi.aviutl2-rs.rusty_ffmpeg/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| rusty_ffmpeg（FFmpeg の引数を指定） | `.mp4` | 映像の引数・音声の引数 |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
