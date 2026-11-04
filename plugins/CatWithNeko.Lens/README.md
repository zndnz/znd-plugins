# Lens

レンズディストーションっぽいやつ

- 元のパッケージ: [Lens](https://github.com/CatWithNeko/aviutl2-scripts)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.Lens`

## znd での使い方

エフェクト「Lens」（カテゴリ「変形」）を掛けると、中心（X・Y でずらせる）からの距離の Pow 乗に比例して絵を歪ませます。Distortion が正なら樽型、負なら糸巻き型です。Distortion が Limit より小さいときは、歪みを Limit で止めて代わりに拡大します（元と同じ）。

## 元との違い

- 元と同じく最近傍で画素を拾います。
