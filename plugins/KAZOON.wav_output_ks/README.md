# WAVファイル出力

モノラル・ステレオを選んで WAV を書き出す。

- 元のパッケージ: [WAVファイル出力](https://github.com/cycloawaodorin/wav_output_ks)（作者: KAZOON、ライセンス: MIT）
- カタログ ID: `KAZOON.wav_output_ks`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:KAZOON.wav_output_ks/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| WAV（モノラル/ステレオ） | `.wav` | チャンネル・量子化 |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
