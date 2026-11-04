# PixelSorter_K（ピクセルソート）

明るさなどの値が範囲内にある画素が続く区間ごとに、その値の順に並べ替える（グリッチアートのピクセルソート）。

- 元のパッケージ: [AviUtl2_PixelSorter_K_Script](https://github.com/korarei/AviUtl2_PixelSorter_K_Script)（作者: korarei、ライセンス: MIT）
- カタログ ID: `korarei.PixelSorter_K`

## znd での使い方

エフェクト「スタイライズ › ピクセルソート（PixelSorter_K）」。方向（角度）で並べ替える向きを変えられる。「範囲外を透明にする」で並べ替えた区間だけを残す。

## 元との違い

- 元は並べ替えを DLL（C++）で行う。znd では同じ処理をこのプラグインの中で行う。
- 値は 8bit に丸めずに比べるので、同じ値の画素の並びが元と違うことがある。
