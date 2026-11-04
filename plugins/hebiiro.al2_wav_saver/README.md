# WAVファイル出力

WAV ファイルを書き出す（音声だけ）。

- 元のパッケージ: [WAVファイル出力](https://github.com/hebiiro/al2_wav_saver/)（作者: 蛇色、ライセンス: MIT）
- カタログ ID: `hebiiro.al2_wav_saver`

## znd での使い方

「書き出し」画面の形式に、次のプラグインの形式が並ぶ（CLI では `znd export … --format plugin:hebiiro.al2_wav_saver/<ID> --param 名前=値`）。

| 形式 | 拡張子 | 設定 |
|---|---|---|
| WAV（音声だけ） | `.wav` | 形式 |

## 元との違い

- 元は AviUtl2 の出力プラグイン（専用の設定画面とエンコーダー）。znd ではパッケージの書き出し形式（FFmpeg の引数の組）として実装したので、エンコードは znd が使う FFmpeg が行う。
  そのコーデックに対応した FFmpeg が必要（GPU エンコーダーは対応する GPU とドライバーも必要）。
- 元の細かいエンコーダーの設定は、「追加の引数」がある形式ではそこに FFmpeg の引数として書く。
