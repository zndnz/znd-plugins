# time_control_assistant.tra2

再生位置を、範囲の指定（絶対・相対）・繰り返し・往復で作る。周期はフレーム・秒・BPM の拍・周波数・回数・BPM グリッドの分割で指定する。

- 元のパッケージ: [time_control_assistant.tra2](https://github.com/sevenc-nanashi/time_control_assistant.tra2)（作者: sevenc-nanashi、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.time_control_assistant-tra2`

## znd での使い方

動画・プリコンポのレイヤーを選び、「プラグイン › time_control_assistant」パネルで種類と周期を決めて「付ける」。
レイヤーのタイムリマップが `tcaTime(…)` のエクスプレッションになる。BPM の拍・BPM グリッドは、コンポのビートグリッドの BPM と拍子を使う。

## 元との違い

- 元は AviUtl2 の「時間制御」オブジェクトの移動方法（トラックバーの動き方）として働く。znd には時間制御オブジェクトが無いので、レイヤーのタイムリマップとして作る。
  周期の中の動きは直線（元はトラックバーの値と移動方法の曲線を組み合わせる）。
- 「往復」は順再生 → 逆再生、「逆往復」は逆再生から始まる往復。
- BPM グリッドはコンポの 1 つのビートグリッドだけを使う（元は BPM の変化を含む複数のグリッドを辿る）。
