# SelfDisplace

自身を基準にディスプレイスメントマップ(移動・回転・拡大)をかける

- 元のパッケージ: [SelfDisplace](https://github.com/CatWithNeko/aviutl2-scripts#selfdisplace)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.SelfDisplace`

## znd での使い方

エフェクト「SelfDisplace」（カテゴリ「変形」）を掛けると、自分自身をぼかした絵の明るさ（0～1）を地図にして、明るいところほど Off X・Off Y だけずらし、Rotate 度回し、Zoom（%）拡大します（Zoom X・Zoom Y は縦横それぞれに効かせる割合）。Rel R・G・B で色ごとに効き方を変えると、色がずれて分かれます。Invert で明るさを反転、Bidirectional で 0～1 を -1～1 にします。

## 元との違い

- 明るさは元と同じく表示値（sRGB）で計算します。透明な部分は明るさ 0 として扱います。
- 色はアルファを超えないように切り詰めます（乗算済みアルファを保つため）。
