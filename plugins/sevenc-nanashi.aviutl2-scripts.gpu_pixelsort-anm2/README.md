# GPUピクセルソート.anm2

GPUを使用してピクセルソートを行うスクリプト

- 元のパッケージ: [GPUピクセルソート.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/GPU%E3%83%94%E3%82%AF%E3%82%BB%E3%83%AB%E3%82%BD%E3%83%BC%E3%83%88/README.md)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.aviutl2-scripts.gpu_pixelsort-anm2`

## znd での使い方

エフェクト「GPUピクセルソート」（カテゴリ「加工」）を掛けると、行（または列）ごとに、明るさが しきい値の最小〜最大 に入る画素が続く区間を、明るさの順に並べ替えます。方向は 横・縦 と、それぞれの反転（暗い順）です。

「ソート範囲可視化」を入れると、並べ替える区間を指定の色で塗って示します（塗りと元画像の不透明度を決められます）。

## 元との違い

- 元は GPU のバイトニックソートで、並べ替える長さが 4096px までですが、znd では CPU で区間ごとに並べ替えるので長さの制限はありません。
- 元の PI はありません。
