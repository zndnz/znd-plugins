# Rusty Chiptune Filter

チップチューンで使われるような音を実装するプラグイン

- 元のパッケージ: [Rusty Chiptune Filter](https://github.com/sevenc-nanashi/aviutl2-rs)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-rs.rusty_chiptune`

## znd での使い方

平面やヌルなどのレイヤーにエフェクト「Rusty Chiptune」（カテゴリ「音声」）を掛けると、そのレイヤーの長さのあいだ音が鳴ります。音源（矩形波・三角波・のこぎり波・正弦波・ノイズ）と音量、高さ（MIDI ノート番号か周波数 Hz）を選べます。音声のあるレイヤーに掛けると、元の音を置き換えます。

## 元との違い

- ノイズは毎回同じ並びになるよう、レイヤーのシードとサンプル位置から作ります（元は毎回違う乱数）。
