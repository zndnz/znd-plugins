# Halftone

なんちゃってハーフトーン

- 元のパッケージ: [Halftone](https://github.com/CatWithNeko/aviutl2-scripts)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Halftone`

## znd での使い方

エフェクト「Halftone」（カテゴリ「色調整」）を掛けると、Size px の升目ごとに、升目の中心が暗いほど大きい Color2 の円を Color1 の地に描きます。Scale で円の大きさの倍率、Blur で色を拾う前のぼかしを決めます。

「Blend」を入れると、結果を元の絵に BlendMode（AviUtl の合成モードの番号: 0 通常・1 加算・2 減算・3 乗算・4 スクリーン・5 オーバーレイ・6 比較(明)・7 比較(暗)・8 輝度・9 色差）と Mix（不透明度）で重ねます。

## 元との違い

- 明るさは元と同じく表示値（sRGB）で計算します。
- 合成モードの計算は znd の SDK の近似です（輝度・色差は W3C の Luminosity・Color と同じ式）。
