// BezierPath のエクスプレッション用ライブラリ。ガイド（BezierPath エフェクトを持つレイヤー）の曲線に沿って並べる。

function __bpPoint(t, pts) {
  var n = pts.length, x = 0, y = 0, b = 1;
  for (var i = 0; i < n; i++) {
    var m = b * Math.pow(t, i) * Math.pow(1 - t, n - 1 - i);
    x += pts[i][0] * m; y += pts[i][1] * m;
    b = b * (n - 1 - i) / (i + 1);
  }
  return [x, y];
}

/** 長さの割合 u（0..1）に当たる曲線のパラメータ t */
function __bpParam(u, pts) {
  var N = 512, acc = [0], prev = __bpPoint(0, pts);
  for (var i = 1; i <= N; i++) {
    var q = __bpPoint(i / N, pts);
    acc.push(acc[i - 1] + Math.sqrt((q[0] - prev[0]) * (q[0] - prev[0]) + (q[1] - prev[1]) * (q[1] - prev[1])));
    prev = q;
  }
  var total = acc[N] || 1, target = u * total, lo = 0, hi = N;
  while (lo < hi) { var mid = (lo + hi) >> 1; if (acc[mid] < target) lo = mid + 1; else hi = mid; }
  if (lo === 0) return 0;
  var r = (target - acc[lo - 1]) / Math.max(acc[lo] - acc[lo - 1], 1e-9);
  return (lo - 1 + r) / N;
}

/**
 * thisComp と、ガイドのレイヤー名 guide の曲線に、count 個のうち index 番目（0 始まり）を置いたときの
 * { position: [x, y]（コンポ座標）, rotation: 度, visible: 真偽 }
 */
function bezierPath(thisComp, guide, index, count) {
  var L = thisComp.layer(guide);
  var fx = L.effect('BezierPath');
  var n = Math.round(Math.max(1, Math.min(9, fx('order')))) + 1;
  var pts = [];
  for (var i = 1; i <= n; i++) pts.push(L.toComp(fx('p' + i).value));
  var offset = fx('offset') / 100, space = fx('space') / 100 / 2;
  var u = offset - (1 - (index + 1) / Math.max(1, count)) * space;
  var t0 = __bpParam(Math.max(0, Math.min(1, u)), pts);
  var t1 = __bpParam(Math.max(0, Math.min(1, u - 0.01)), pts);
  var p = __bpPoint(t0, pts), q = __bpPoint(t1, pts);
  var rot = Math.atan2(p[1] - q[1], p[0] - q[0]) * 180 / Math.PI;
  if (Math.abs(p[0] - q[0]) + Math.abs(p[1] - q[1]) < 1e-9) rot = 0;
  return { position: p, rotation: rot + fx('rz'), visible: u >= 0 && u <= 1 };
}
