// TLショトカ移動2（σ軸）の移植: 編集点・マーカー・ビートグリッドへの移動と、レイヤーの選択・長さ調整のコマンド。

function __c() { return comp(ctx.comp); }
function __fps(c) { return c.frame_rate.num / c.frame_rate.den; }
function __frame(c, t) { return Math.round(seconds(t) * __fps(c)); }
function __goFrame(c, f) { host.setFrame(Math.max(0, Math.min(f, __frame(c, c.duration)))); }
function __sel(c) { return ctx.layer ? layer(c, ctx.layer) : null; }

function __keys(o, out) {
  if (!o || typeof o !== 'object') return;
  if (Array.isArray(o.keyframes)) o.keyframes.forEach(function (k) { if (typeof k.time === 'number') out.push(k.time); });
  for (var k in o) if (k !== 'keyframes') __keys(o[k], out);
}

/** 編集点（フレーム）の一覧。withKeys でキーフレームも含める。 */
function __points(c, layers, withKeys) {
  var out = [];
  layers.forEach(function (l) {
    out.push(__frame(c, l.in_point), __frame(c, l.out_point));
    if (withKeys) { var ks = []; __keys(l.transform, ks); __keys(l.effects, ks); __keys(l.kind, ks); ks.forEach(function (t) { out.push(__frame(c, t)); }); }
  });
  return out;
}

function __step(points, dir) {
  var f = ctx.frame, best = null;
  points.forEach(function (p) {
    if ((dir > 0 && p > f && (best === null || p < best)) || (dir < 0 && p < f && (best === null || p > best))) best = p;
  });
  return best;
}

function __move(layerOnly, withKeys, dir) {
  var c = __c(), s = __sel(c);
  var ls = layerOnly ? (s ? [s] : []) : c.layers;
  if (!ls.length) { host.status('レイヤーを選択してください'); return; }
  var p = __step(__points(c, ls, withKeys), dir);
  if (p !== null) __goFrame(c, p);
}
function nextPointLayer() { __move(true, true, 1); }
function prevPointLayer() { __move(true, true, -1); }
function nextEdgeLayer() { __move(true, false, 1); }
function prevEdgeLayer() { __move(true, false, -1); }
function nextPointComp() { __move(false, true, 1); }
function prevPointComp() { __move(false, true, -1); }
function nextEdgeComp() { __move(false, false, 1); }
function prevEdgeComp() { __move(false, false, -1); }

function __marks(c) { return (c.markers || []).map(function (m) { return __frame(c, m.time); }).sort(function (a, b) { return a - b; }); }
function nextMarker() { var c = __c(), p = __step(__marks(c), 1); if (p !== null) __goFrame(c, p); }
function prevMarker() { var c = __c(), p = __step(__marks(c), -1); if (p !== null) __goFrame(c, p); }
function __gotoMarker(n) { var c = __c(), m = __marks(c); if (m[n - 1] !== undefined) __goFrame(c, m[n - 1]); else host.status(n + ' 番目のマーカーがありません'); }
function marker1() { __gotoMarker(1); } function marker2() { __gotoMarker(2); } function marker3() { __gotoMarker(3); }
function marker4() { __gotoMarker(4); } function marker5() { __gotoMarker(5); } function marker6() { __gotoMarker(6); }
function marker7() { __gotoMarker(7); } function marker8() { __gotoMarker(8); } function marker9() { __gotoMarker(9); }
function marker10() { __gotoMarker(10); }

/** 再生ヘッドを、選んだレイヤーが見えるいちばん近い位置へ。 */
function gotoSelected() {
  var c = __c(), l = __sel(c);
  if (!l) { host.status('レイヤーを選択してください'); return; }
  var a = __frame(c, l.in_point), b = __frame(c, l.out_point) - 1;
  if (ctx.frame < a) __goFrame(c, a); else if (ctx.frame > b) __goFrame(c, b);
}

// ---- ビートグリッド ----
function __grid(c) { var g = c.beat_grid; return g && g.bpm > 0 ? g : null; }
function __beatMove(div, dir) {
  var c = __c(), g = __grid(c);
  if (!g) { host.status('コンポにビートグリッドがありません'); return; }
  var fps = __fps(c), step = 60 / g.bpm / div, t0 = seconds(g.offset || 0);
  var t = ctx.frame / fps, k = (t - t0) / step;
  var n = dir > 0 ? Math.floor(k + 1e-6) + 1 : Math.ceil(k - 1e-6) - 1;
  var f = Math.round((t0 + n * step) * fps);
  if (f === ctx.frame) f += dir;
  __goFrame(c, f);
}
function nextBar() { var g = __grid(__c()); __beatMove(g ? 1 / (g.beats_per_bar || 4) : 1, 1); }
function prevBar() { var g = __grid(__c()); __beatMove(g ? 1 / (g.beats_per_bar || 4) : 1, -1); }
function nextBeat() { __beatMove(1, 1); }
function prevBeat() { __beatMove(1, -1); }
function nextSubBeat() { var g = __grid(__c()); __beatMove(g ? Math.max(1, g.subdivision || 4) : 4, 1); }
function prevSubBeat() { var g = __grid(__c()); __beatMove(g ? Math.max(1, g.subdivision || 4) : 4, -1); }
function __shiftGrid(df) { var c = __c(), g = __grid(c); if (!g) { host.status('コンポにビートグリッドがありません'); return; } g.offset = (g.offset || 0) + flicks(df / __fps(c)); }
function gridOriginLeft() { __shiftGrid(-1); }
function gridOriginRight() { __shiftGrid(1); }
function gridOriginHere() { var c = __c(), g = __grid(c); if (!g) return; g.offset = flicks(ctx.frame / __fps(c)); }
/** いちばん近い小節線が再生ヘッドに来るよう、グリッドの基準をずらす。 */
function nearestBarHere() {
  var c = __c(), g = __grid(c);
  if (!g) return;
  var bar = 60 / g.bpm * (g.beats_per_bar || 4), t = ctx.frame / __fps(c), t0 = seconds(g.offset || 0);
  var n = Math.round((t - t0) / bar);
  g.offset = flicks(t - n * bar);
}

// ---- レイヤー ----
function __neighbor(dir) {
  var c = __c(), l = __sel(c);
  var i = l ? c.layers.indexOf(l) + dir : (dir > 0 ? 0 : c.layers.length - 1);
  var n = c.layers[Math.max(0, Math.min(c.layers.length - 1, i))];
  if (n) host.select(n.id);
}
function selectAbove() { __neighbor(-1); }
function selectBelow() { __neighbor(1); }
function extendStartHere() { var c = __c(), l = __sel(c); if (l && flicks(ctx.time) < l.out_point) l.in_point = flicks(ctx.frame / __fps(c)); }
function extendEndHere() { var c = __c(), l = __sel(c); if (l && flicks(ctx.time) > l.in_point) l.out_point = flicks((ctx.frame + 1) / __fps(c)); }
function toggleVisible() { var c = __c(), l = __sel(c); if (l) l.visible = !l.visible; }
function toggleMarker() {
  var c = __c(), f = ctx.frame;
  var before = (c.markers || []).length;
  c.markers = (c.markers || []).filter(function (m) { return __frame(c, m.time) !== f; });
  if (c.markers.length === before) c.markers.push({ time: flicks(f / __fps(c)), duration: 0, comment: '' });
  c.markers.sort(function (a, b) { return a.time - b.time; });
}
