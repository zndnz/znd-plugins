# Pixelizer（ドット絵）

ドット絵風にする。ブロック化・色数を減らす・階調・順序ディザ（2x2 / 4x4）・パレット（GameBoy・16bit・4 階調グレー）・輪郭の強調。

- 元のパッケージ: [aviutl2_script_Pixelizer](https://github.com/nctype/aviutl2_script_Pixelizer)（作者: nctype、ライセンス: MIT）
- カタログ ID: `nctype.Pixelizer`

## znd での使い方

エフェクト「スタイライズ › Pixelizer（ドット絵）」。ピクセルサイズはレイヤー座標の px。

## 元との違い

- 「ドット固定: 画面固定」（画面の格子に揃える）は無い。znd のエフェクトはレイヤーの画像に掛かるので、格子はレイヤーに固定される。
  動くレイヤーで画面に揃えたいときは、調整レイヤーに掛ける。
- 色の計算は元と同じく表示値（sRGB）で行う。
