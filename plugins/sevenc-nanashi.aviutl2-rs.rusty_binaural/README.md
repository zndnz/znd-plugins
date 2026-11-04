# Rusty Binaural Filter

バイノーラルな感じの音を再現できるプラグイン

- 元のパッケージ: [Rusty Binaural Filter](https://github.com/sevenc-nanashi/aviutl2-rs)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-rs.rusty_binaural`

## znd での使い方

音声のあるレイヤーにエフェクト「Rusty Binaural」（カテゴリ「音声」）を掛けると、音をモノラルにして、横回転（左右、度）・縦回転（上下、度）の向きから聞こえるように左右の耳へ振り分けます（ヘッドホン向け）。ゲインで音量を決めます。

## 元との違い

- 元は IRCAM の HRIR（頭部インパルス応答）データを畳み込みますが、そのデータは同梱できないため、znd では頭の模型（Woodworth の式による左右の耳の時間差と、Brown–Duda の頭の影のフィルタ）で向きの手がかりを作ります。左右の定位は近いですが、前後や上下の聞こえ方は元ほどはっきりしません。
- 元は 44.1kHz に変換してから処理しますが、znd ではそのままのサンプリング周波数で処理します。
