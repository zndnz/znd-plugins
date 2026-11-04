// 中心ずらし_A（azurite）の移植: アンカーポイント（回転・拡大の中心）を、絵の 9 か所のどこかへ移す。

function __apRect() {
  var r = thisLayer.sourceRectAtTime(time);
  return r;
}
function __apCell(coord) {
  var c = Math.min(Math.max(Math.round(coord), 1), 9) - 1;
  return [c % 3, Math.floor(c / 3)];
}

/** アンカーポイント: 絵の矩形の 9 か所（1 左上 … 5 中心 … 9 右下）＋オフセット。 */
function adjustPivotAnchor(coord, ox, oy) {
  var r = __apRect(), k = __apCell(coord);
  return [r.left + r.width * k[0] / 2 + (ox || 0), r.top + r.height * k[1] / 2 + (oy || 0)];
}

/** 位置: アンカーを中心から動かした分だけ位置も動かし、絵をその場に残す。 */
function adjustPivotPosition(value, coord, ox, oy) {
  var r = __apRect(), k = __apCell(coord);
  var dx = r.width * (k[0] - 1) / 2 + (ox || 0), dy = r.height * (k[1] - 1) / 2 + (oy || 0);
  var s = thisLayer.scale, sx = (s ? s[0] : 100) / 100, sy = (s ? s[1] : 100) / 100;
  var a = degreesToRadians(thisLayer.rotation || 0), c = Math.cos(a), n = Math.sin(a);
  dx *= sx; dy *= sy;
  return [value[0] + dx * c - dy * n, value[1] + dx * n + dy * c];
}
