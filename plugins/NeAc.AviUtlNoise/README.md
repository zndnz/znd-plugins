# 無印ノイズ

AviUtl（無印、ExEdit 0.92）のフィルタ「ノイズ」の 6 種類（Type1〜Type6）を、同じ表と同じ整数演算で再現する。

- 元のパッケージ: [aviutl2_script_AviUtlNoise](https://github.com/nea-c/aviutl2_script_AviUtlNoise)（作者: 猫愛、ライセンス: MIT）
- カタログ ID: `NeAc.AviUtlNoise`

## znd での使い方

エフェクト「スタイライズ › 無印ノイズ」。合成モード「アルファ値と乗算」でノイズの形に抜き、「明るさと乗算」で暗くする。

## 元との違い

- 表は元のパッケージの `aviutl_noise_table.png` から取り出したもの（`native/src/table.bin`）。計算は元のシェーダーと同じ。
- 周期はレイヤー座標の 1px あたりの量なので、縮小プレビューでも同じ模様の大きさになる（模様の細部は 1/2 解像度で少し変わる）。
- 「仮想バッファ」は無い。速度は元と同じく一定の速さとして扱う。
