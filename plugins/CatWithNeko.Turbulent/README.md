# Turbulent

簡易タービュレントディスプレイス

- 元のパッケージ: [Turbulent](https://github.com/CatWithNeko/aviutl2-scripts#turbulent)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Turbulent`

## znd での使い方

エフェクト「Turbulent」（カテゴリ「変形」）を掛けると、ノイズの明るさに合わせて絵を斜め（右下・左上）に最大 Amount px ずらします。Scale はノイズの周期（px）、Speed はノイズが変わる速さ、Blur はノイズに掛けるぼかしです。Expansion を入れると、ずれではみ出る分だけ周りを広げます。

## 元との違い

- 元は AviUtl の「ノイズ」フィルタ（Type1）を使いますが、znd では SDK の値ノイズを使うので模様は違います。
- シードにはレイヤーごとのシードも混ぜます（元はオブジェクトの番号を足します）。
