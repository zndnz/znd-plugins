# Exposure

露出・ガンマ・オフセットの補正をかける

- 元のパッケージ: [Exposure](https://github.com/CatWithNeko/aviutl2-scripts#exposure)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Exposure`

## znd での使い方

エフェクト「Exposure」（カテゴリ「色調整」）を掛けると、色に Exposure を掛け、0～1 に収めてから 1/Gamma 乗し、最後に Offset を足します。

## 元との違い

- 半透明の部分は、乗算済みアルファを戻した色で計算します（元は乗算済みのまま計算します）。
