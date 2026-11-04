// 目標指定拡大縮小（Nanashi.）の移植: 絵の大きさが目標のピクセル数になる拡大率を返す（縦横比はそのまま）。

/**
 * mode: 0 下限指定（範囲を埋める・cover）/ 1 上限指定（範囲に収める・contain）/ 2 幅指定 / 3 高さ指定。
 * value は今の拡大率 [x, y]（%）。絵の大きさは thisLayer.sourceRectAtTime() に今の拡大率を掛けたもの。
 */
function targetedScale(value, targetWidth, targetHeight, mode) {
  var r = thisLayer.sourceRectAtTime(time);
  var sx = value[0] / 100, sy = value[1] / 100;
  var cw = r.width * sx, ch = r.height * sy;
  if (!(cw > 0) || !(ch > 0)) return value;
  var aspect = cw / ch, w, h;
  if (mode === 0 || mode === 1) {
    var byWidth = mode === 0 ? targetWidth / aspect >= targetHeight : targetWidth / aspect <= targetHeight;
    if (byWidth) { w = targetWidth; h = targetWidth / aspect; } else { w = targetHeight * aspect; h = targetHeight; }
  } else if (mode === 2) { w = targetWidth; h = targetWidth / aspect; }
  else { w = targetHeight * aspect; h = targetHeight; }
  return [value[0] * w / cw, value[1] * h / ch];
}
