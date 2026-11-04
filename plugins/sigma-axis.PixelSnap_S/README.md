# PixelSnap_S

オブジェクトの位置を、ピクセルの格子にぴったり合うように丸める。小数の位置に置いた文字や画像がぼやけるのを防ぐ。

- 元のパッケージ: [aviutl2_script_PixelSnap_S](https://github.com/sigma-axis/aviutl2_script_PixelSnap_S)（作者: σ軸、ライセンス: MIT）
- カタログ ID: `sigma-axis.PixelSnap_S`

## znd での使い方

レイヤーを選び「プラグイン › ピクセルに合わせる（PixelSnap_S）」。位置に `pixelSnap(value, anchorPoint, scale, rotation)` のエクスプレッションが付く。
アニメーションしていても各フレームで丸められる。

## 元との違い

- 元はグループ制御・カメラの奥行き（z）まで含めて画面上の位置を計算する。znd では親レイヤー・3D は考えず、
  レイヤー自身の位置・アンカーポイント・拡大率から左上の位置を求めて丸める（2D の通常の使い方では同じ結果）。
- 斜めに回転しているときは、回転の中心だけを格子に合わせる。
