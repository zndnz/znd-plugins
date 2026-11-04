# Rusty Equalizer Filter

イコライザーを実装するプラグイン

- 元のパッケージ: [Rusty Equalizer Filter](https://github.com/sevenc-nanashi/aviutl2-rs)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-rs.rusty_equalizer`

## znd での使い方

音声のあるレイヤーにエフェクト「Rusty Equalizer」（カテゴリ「音声」）を掛けると、低音・中音・高音をそれぞれの周波数で ±15dB 上げ下げし、必要ならハイパス・ローパスで帯域を切ります。Wet は加工した音を混ぜる割合、Bypass で一時的に切れます。

## 元との違い

- フィルタの式は元と同じ双 2 次（RBJ の式、Q = 1/√2）です。
- 元は前のブロックの結果を覚えて繰り返しの描画を速くしますが、znd ではいりません。
