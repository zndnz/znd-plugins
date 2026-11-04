# Rusty Single Image Output

今のフレームを静止画で書き出す。

- 元のパッケージ: [Rusty Single Image Output](https://github.com/sevenc-nanashi/aviutl2-rs)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-rs.rusty_single_image_output`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:sevenc-nanashi.aviutl2-rs.rusty_single_image_output/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| 静止画（PNG・今のフレーム） | `.png` |  |
| 静止画（JPG・今のフレーム） | `.jpg` | JPEG の画質（2 高〜31 低） |
| 静止画（WEBP・今のフレーム） | `.webp` |  |
| 静止画（BMP・今のフレーム） | `.bmp` |  |
| 静止画（TIFF・今のフレーム） | `.tiff` |  |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
