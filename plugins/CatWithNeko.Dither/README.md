# Dither

閾値がランダム変化する二値化

- 元のパッケージ: [Dither](https://github.com/CatWithNeko/aviutl2-scripts#dither)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Dither`

## znd での使い方

エフェクト「Dither」（カテゴリ「色調整」）を掛けると、明るさが閾値以上なら Color1、未満なら Color2 で塗ります。閾値は size px の升目ごとに乱数で ±strength/2 揺れます。seed で乱数の並びを変えます。

## 元との違い

- 明るさは元と同じく表示値（sRGB）で計算します。
