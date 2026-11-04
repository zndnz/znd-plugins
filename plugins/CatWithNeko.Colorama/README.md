# Colorama

簡易コロラマ

- 元のパッケージ: [Colorama](https://github.com/CatWithNeko/aviutl2-scripts#colorama)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Colorama`

## znd での使い方

エフェクト「Colorama」（カテゴリ「色調整」）を掛けると、明るさ（Lum Mode の式）を Color1～Color6 を巡るグラデーションに置き換えます。

- Shift: グラデーションをずらす（%）
- Cycle: 明るさ 0～1 の間にグラデーションを何周させるか
- Step: 使う色の数（2～6）
- Invert: 明るさを反転してから置き換える
- Blend Mode・mix: 置き換えた色を元の絵にどう重ねるか（元のスクリプトの合成の式と同じ）

## 元との違い

- 合成の計算は元のスクリプトの式（乗算済みの表示値）をそのまま使います。alpha_add2 は元と同じく通常の合成になります。
