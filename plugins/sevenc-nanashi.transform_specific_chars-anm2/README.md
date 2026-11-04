# transform_specific_chars.anm2（特定の文字を変形）

テキストの中の特定の文字（文字の集合、または正規表現に一致する部分）だけを、移動・回転・拡大・透明度・文字色で変える。

- 元のパッケージ: [transform_specific_chars.anm2](https://github.com/sevenc-nanashi/transform_specific_chars.anm2)（作者: sevenc-nanashi、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.transform_specific_chars-anm2`

## znd での使い方

テキストレイヤーを選び、「プラグイン › 特定の文字を変形」パネルで対象の文字と変形を決めて「付ける」。
「transform_specific_chars」という名前のアニメーターが付き、対象の文字だけを選ぶエクスプレッションセレクターが入る。動かしたいときはアニメーターの値にキーフレームを打つ。

## 元との違い

- 元は「文字毎に個別オブジェクト」のテキストに掛けるアニメーション効果で、描画のたびに対象を判定する。znd ではパネルで付けたときに対象の文字の位置を数えるので、テキストを書き換えたら付け直す。
- Z 方向の移動・X/Y 軸回転・中心の移動・「エフェクト終端」は無い（znd のテキストのアニメーターは 2D）。
- 正規表現は JavaScript の正規表現（元は Rust の regex）。
