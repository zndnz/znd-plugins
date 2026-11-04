# OpticsCompensation_s（光学補正）

レンズの歪みを再現・補正する。強さが正で樽型（中心が膨らむ）、負で糸巻き型。

- 元のパッケージ: [OpticsCompensation_s_for_AviUtl2](https://github.com/SagyouDarui900/OpticsCompensation_s_for_AviUtl2)（作者: Sagyou、ライセンス: MIT）
- カタログ ID: `Sagyou.OpticsCompensation_s_for_AviUtl2`

## znd での使い方

エフェクト「ディストーション › 光学補正（OpticsCompensation_s）」。中心はビューアでドラッグできる。

## 元との違い

- 中心は「中心 X/Y」と「中心点アンカー」の 2 つを足す方式ではなく、1 つの点で指定する。
- 計算は元のシェーダーと同じ（焦点距離 = 500 / tan(強さ × π/2)、樽型は tan、糸巻き型は atan）。焦点距離はレイヤー座標の px。
