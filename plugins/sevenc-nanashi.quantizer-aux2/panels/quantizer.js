// quantizer.aux2（Nanashi.）の移植: BPM グリッドから少しずれている開始・終了・キーフレームを探して、グリッドに合わせる。
// グリッドはコンポのビートグリッド（BPM・分割数・オフセット）。

var DEF = { distance: 2, start: true, key: true, end: true, compEnd: false, sort: 0, found: null };

function defaults(s) { for (var k in DEF) if (s[k] === undefined) s[k] = DEF[k]; return s; }

function __grid(c) {
  var g = c.beat_grid;
  if (!g || !(g.bpm > 0)) return null;
  return { step: flicks(60 / g.bpm / Math.max(1, g.subdivision || 1)), offset: g.offset || 0 };
}
function __near(g, t) { return g.offset + Math.round((t - g.offset) / g.step) * g.step; }

/** キーフレームを持つプロパティ（{ path, prop }）をすべて集める。 */
function __props(l) {
  var out = [];
  function walk(o, path) {
    if (!o || typeof o !== 'object') return;
    if (Array.isArray(o.keyframes) && o.keyframes.length) { out.push({ path: path, prop: o }); return; }
    for (var k in o) if (k !== 'keyframes') walk(o[k], path ? path + '.' + k : k);
  }
  walk(l.transform, 'transform');
  (l.effects || []).forEach(function (fx, i) { walk(fx.params, 'effects.' + i + '(' + (fx.name || fx.kind) + ')'); });
  return out;
}

/** グリッドから 1〜distance フレームずれている点の一覧。 */
function findGaps(c, s) {
  var g = __grid(c);
  if (!g) return null;
  var frame = flicks(c.frame_rate.den / c.frame_rate.num);
  var tol = Math.round(Number(s.distance)) * frame;
  var out = [];
  function check(l, kind, t, extra) {
    var n = __near(g, t), d = t - n;
    if (d !== 0 && Math.abs(d) <= tol + 1) out.push({ layer: l.id, name: l.name, kind: kind, time: t, offset: Math.round(d / frame), target: n, extra: extra || null });
  }
  c.layers.forEach(function (l) {
    if (l.locked) return;
    if (s.start) check(l, '開始', l.in_point);
    if (s.end && (s.compEnd || l.out_point !== c.duration)) check(l, '終了', l.out_point);
    if (s.key) __props(l).forEach(function (p) { p.prop.keyframes.forEach(function (k, i) { check(l, 'キー ' + p.path, l.start_time + k.time, { path: p.path, index: i }); }); });
  });
  out.sort(s.sort === 1 ? function (a, b) { return a.time - b.time; } : function (a, b) { return c.layers.findIndex(function (l) { return l.id === a.layer; }) - c.layers.findIndex(function (l) { return l.id === b.layer; }) || a.time - b.time; });
  return out;
}

function fixGap(c, gap) {
  var l = layer(c, gap.layer);
  if (!l) return;
  var d = gap.target - gap.time;
  if (gap.kind === '開始') { l.in_point += d; }
  else if (gap.kind === '終了') { l.out_point += d; }
  else {
    var p = __props(l).filter(function (q) { return q.path === gap.extra.path; })[0];
    if (p && p.prop.keyframes[gap.extra.index]) { p.prop.keyframes[gap.extra.index].time += d; p.prop.keyframes.sort(function (a, b) { return a.time - b.time; }); }
  }
}

function panel(s) {
  defaults(s);
  var items = [
    { type: 'heading', text: 'quantizer' },
    { type: 'number', id: 'distance', label: 'フレーム数（このフレーム以内のずれを探す）', value: s.distance },
    { type: 'row', items: [
      { type: 'checkbox', id: 'start', label: '開始位置', value: s.start },
      { type: 'checkbox', id: 'key', label: 'キーフレーム', value: s.key },
      { type: 'checkbox', id: 'end', label: '終了位置', value: s.end },
      { type: 'checkbox', id: 'compEnd', label: 'コンポの終わり', value: s.compEnd }] },
    { type: 'choice', id: 'sort', label: 'ソート', options: ['レイヤー順', 'フレーム順'], value: s.sort },
    { type: 'row', items: [{ type: 'button', id: 'find', label: '探す' }, { type: 'button', id: 'fixall', label: 'すべて補正' }] }
  ];
  if (s.found === 'nogrid') items.push({ type: 'hint', text: 'コンポにビートグリッドがありません（コンポの設定で BPM を決めてください）。' });
  else if (s.found) {
    items.push({ type: 'label', text: s.found.length + ' 個のずれ' });
    s.found.slice(0, 100).forEach(function (g, i) {
      items.push({ type: 'row', items: [
        { type: 'label', text: g.name + ' / ' + g.kind + ' / ずれ ' + (g.offset > 0 ? '+' : '') + g.offset + 'f' },
        { type: 'button', id: 'jump:' + i, label: '移動' },
        { type: 'button', id: 'fix:' + i, label: '補正' }] });
    });
  }
  return items;
}

function onEvent(id, value, s) {
  defaults(s);
  var c = comp(ctx.comp);
  var refind = function () { var f = findGaps(c, s); s.found = f === null ? 'nogrid' : f; };
  if (id === 'find') { refind(); return; }
  if (id === 'fixall') {
    var f = findGaps(c, s);
    if (f === null) { s.found = 'nogrid'; return; }
    f.forEach(function (g) { fixGap(c, g); });
    host.status(f.length + ' 個補正しました');
    refind();
    return;
  }
  var m = /^(jump|fix):(\d+)$/.exec(id);
  if (m && Array.isArray(s.found)) {
    var g = s.found[Number(m[2])];
    if (!g) return;
    if (m[1] === 'jump') { host.select(g.layer); host.setTime(seconds(g.time)); return; }
    fixGap(c, g);
    refind();
    return;
  }
  s[id] = value;
}
