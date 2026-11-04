# ドット絵変形.anm2

ドット絵の拡大縮小・回転を行うスクリプト

- 元のパッケージ: [ドット絵変形.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/tree/main/scripts/%E3%83%89%E3%83%83%E3%83%88%E7%B5%B5%E5%A4%89%E5%BD%A2#%E3%83%89%E3%83%83%E3%83%88%E7%B5%B5%E5%A4%89%E5%BD%A2anm2)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-scripts.pixel-transform-anm2`

## znd での使い方

エフェクト「ドット絵変形」（カテゴリ「変形」）を掛けると、ドット絵をぼかさずにドットのまま拡大縮小・回転します。中心X・Y（回転と拡大の中心）、拡大率・X拡大率・Y拡大率、回転を決めます。

- 発展補間: cleanEdge で、線を塗りの上に残しながら斜めの線をなめらかに補間します（線の太さ・斜め補間・基準色・補間閾値は cleanEdge の Line Width・Slopes・Highest Color・Similar Threshold に当たります）
- 透明グリッド: 透明な所に、この大きさの市松模様を敷きます

## 元との違い

- 元の「ピクセル補正」はありません。znd のエフェクトの結果はいつも整数の画素に置かれます（レイヤーの位置を整数にするには PixelSnap_S の移植が使えます）。
- 元の PI とデバッグモードはありません。
