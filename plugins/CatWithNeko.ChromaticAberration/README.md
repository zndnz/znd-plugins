# Chromatic Aberration

簡易色収差

- 元のパッケージ: [Chromatic Aberration](https://github.com/CatWithNeko/aviutl2-scripts#chromatic-aberration)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.ChromaticAberration`

## znd での使い方

エフェクト「Chromatic Aberration」（カテゴリ「加工」）を掛けると、Channel で選んだ 1 色をそのまま残し、残りの 2 色を X・Y の向きに逆向きにずらします。Zoom（%）で CenterX・CenterY を中心にずれる色を拡大縮小し、Bidrectional を入れると 2 色が逆向きに拡大縮小します。

## 元との違い

- Channel は数値のトラックではなく選択肢です。
- 色はアルファを超えないように切り詰めます（乗算済みアルファを保つため）。
