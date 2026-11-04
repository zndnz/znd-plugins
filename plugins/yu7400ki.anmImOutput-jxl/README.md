# JXL出力プラグイン

動画をアニメーション JPEG XL で書き出す。

- 元のパッケージ: [JXL出力プラグイン](https://yu7400ki.me/aviutl2-animated-image-output/)（作者: yu7400ki、ライセンス: MIT）
- カタログ ID: `yu7400ki.anmImOutput-jxl`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:yu7400ki.anmImOutput-jxl/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| JPEG XL（アニメーション） | `.jxl` | 距離（0 で可逆、1 で高画質）・手間（1〜9） |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
