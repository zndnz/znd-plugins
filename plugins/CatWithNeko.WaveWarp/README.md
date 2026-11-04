# Wave Warp

角度と波形を指定できるラスター

- 元のパッケージ: [Wave Warp](https://github.com/CatWithNeko/aviutl2-scripts#wave-warp)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.WaveWarp`

## znd での使い方

エフェクト「Wave Warp」（カテゴリ「変形」）を掛けると、Angle の向きに進む波で、波と直角の向きに絵を最大 Amp px ずらします。Length は波長（px）、Phase は位相（度）、speed は 1 秒あたりに進む位相（度）、Type は波形（sin / saw / tri / square）です。Expand を入れると、ずれる分だけ周りを広げます。

## 元との違い

- speed の時間はレイヤーの中の時間（レイヤーの始まりが 0 秒）です。
