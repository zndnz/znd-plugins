# MovingMosaic_S（動くモザイク）

マスの中のランダムな位置から色を拾うモザイク。シードをキーフレームやエクスプレッション（`time * 10` など）で動かすと、モザイクがちらちら動いて見える。

- 元のパッケージ: [aviutl2_script_MovingMosaic_S](https://github.com/sigma-axis/aviutl2_script_MovingMosaic_S)（作者: σ軸、ライセンス: MIT）
- カタログ ID: `sigma-axis.MovingMosaic_S`

## znd での使い方

エフェクト「スタイライズ › 動くモザイク（MovingMosaic_S）」。サイズはレイヤー座標の px、縦横比は正で縦長・負で横長。
「マスの基準位置」でマスの格子の原点をずらせる（ビューアでドラッグできる）。

## 元との違い

- 乱数は元と同じ IbukiHash（Andante, CC0 1.0）。シードが 0 以上のときにレイヤーごとに乱数を変える混ぜ方は、AviUtl のオブジェクト ID の代わりに znd のレイヤーの乱数の種を使う。
- `PI`（パラメータ注入）は無い。
