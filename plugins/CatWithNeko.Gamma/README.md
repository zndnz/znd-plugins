# Gamma

RGBまたは全体にかかるガンマ補正

- 元のパッケージ: [Gamma](https://github.com/CatWithNeko/aviutl2-scripts#gamma)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Gamma`

## znd での使い方

エフェクト「Gamma」（カテゴリ「色調整」）を掛けると、色を 1/ガンマ 乗します（1 より大きいと明るくなる）。R・G・B は Gamma に掛ける各色の倍率です。

## 元との違い

- 元と同じく表示値（sRGB）で計算します。半透明の部分は、乗算済みアルファを戻した色で計算します（元は乗算済みのまま計算します）。
