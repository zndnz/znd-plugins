# MaxMin

最大値・最小値フィルタ

- 元のパッケージ: [MaxMin](https://github.com/CatWithNeko/aviutl2-scripts#maxmin)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.MaxMin`

## znd での使い方

エフェクト「MaxMin」（カテゴリ「加工」）を掛けると、各画素の周り（一辺 2×Scale+1 の正方形を Angle 度回したもの）で、R・G・B ごとの最大値（Max）か最小値（Min）を取ります。アルファはそのままです。

## 元との違い

- Max/Min は数値のトラックではなく選択肢です。
- 色がアルファを超えないように切り詰めます（乗算済みアルファを保つため）。
