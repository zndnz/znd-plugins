# AlphaDither_C

いわゆるディザ合成

- 元のパッケージ: [AlphaDither_C](https://github.com/CatWithNeko/aviutl2-scripts#alphadither_c)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.AlphaDither`

## znd での使い方

エフェクト「AlphaDither_C」（カテゴリ「抽出」）を掛けると、半透明の部分を、Size px の升目ごとに「不透明か透明か」へ乱数で振り分けます。不透明になる確率は元の不透明度 × Alpha（%）です。Seed で乱数の並びを変えます。

## 元との違い

- 乱数の式は元のシェーダーと同じです（GPU と CPU の sin の精度差で並びが変わることがあります）。
