// PixelSnap_S（σ軸）の移植: レイヤーの左上がピクセルの格子に乗るように位置を丸める（ぼやけ防止）。

/**
 * 位置 value を、アンカーポイントと拡大率を考えたうえで、レイヤーの左上が整数のピクセルに来るよう丸める。
 * 回転しているレイヤーでは、回転の中心（アンカーポイント）を整数か半整数のピクセルに合わせる。
 */
function pixelSnap(value, anchor, scale, rotation) {
  var sx = (scale ? scale[0] : 100) / 100, sy = (scale ? scale[1] : 100) / 100;
  var ax = anchor ? anchor[0] * sx : 0, ay = anchor ? anchor[1] * sy : 0;
  if (rotation && Math.abs(rotation % 90) > 1e-6) {
    // 斜めの回転ではどこを合わせても格子に乗らないので、中心だけ合わせる
    var fx = ax % 1, fy = ay % 1;
    return [Math.round(value[0] - fx) + fx, Math.round(value[1] - fy) + fy];
  }
  return [Math.round(value[0] - ax) + ax, Math.round(value[1] - ay) + ay];
}
