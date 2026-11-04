# Erosion

輪郭を削るだけのシンプルなエフェクト 縁取りの逆

- 元のパッケージ: [Erosion](https://github.com/CatWithNeko/aviutl2-scripts#erosion)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Erosion`

## znd での使い方

エフェクト「Erosion」（カテゴリ「加工」）を掛けると、不透明な部分の輪郭を strength px 削ります（縁取りの逆）。

## 元との違い

- 元は 3×3 の最小値を strength 回くり返しますが、znd では同じ結果になる一辺 2×strength+1 の正方形の最小値を一度に取ります。
- 色はそのままで、アルファだけが削られます。
