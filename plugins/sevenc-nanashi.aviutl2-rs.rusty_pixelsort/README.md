# Rusty Pixel Sort Filter

ピクセルソートを行うプラグイン

- 元のパッケージ: [Rusty Pixel Sort Filter](https://github.com/sevenc-nanashi/aviutl2-rs)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-rs.rusty_pixelsort`

## znd での使い方

エフェクト「Rusty Pixel Sort」（カテゴリ「加工」）を掛けると、行（または列）ごとに、明るさがしきい値以上（または以下）の画素が続く区間を、明るさの順に並べ替えます。ソート方向は 左右 / 左右（反転）/ 上下 / 上下（反転）です。

## 元との違い

- 計算は元と同じです（明るさは 8bit の表示値から求めます）。
