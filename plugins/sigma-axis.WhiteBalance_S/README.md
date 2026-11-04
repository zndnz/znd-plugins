# WhiteBalance_S（ホワイトバランスσ・色温度σ）

ホワイトバランスを調整する。「ホワイトバランスσ」は 2 色を指定して一方を他方の色へ（または成分ごとの倍率で）、「色温度σ」は色温度（K または Mired）を指定して変換する。

- 元のパッケージ: [aviutl2_script_WhiteBalance_S](https://github.com/sigma-axis/aviutl2_script_WhiteBalance_S)（作者: σ軸、ライセンス: MIT）
- カタログ ID: `sigma-axis.WhiteBalance_S`

## znd での使い方

エフェクト「カラー補正 › ホワイトバランスσ」「カラー補正 › 色温度σ」。色空間 XYZ（既定）はリニアの XYZ で、RGB は表示値の RGB で倍率を掛ける。

## 元との違い

- 「色温度σ(Mired)」（非推奨・凍結）は「色温度σ」の単位 Mired に含めた。
- 元の色温度σは色空間 RGB を選んでも XYZ の倍率をそのまま RGB に掛ける（文字列と数の比較の不具合）。znd では意図どおり RGB の倍率に直して掛ける。
- `PI` は無い。
