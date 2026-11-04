// Palette History（てるよし）の移植: プロジェクトで使った色を、最近変えた順に並べるパネル。
// 色を押すと、選んだレイヤーの色（平面の色・テキストの塗り・シェイプの塗り）にします。「コピー」で #RRGGBB をクリップボードへ。

var MAX = 24;

function __hex(c) {
  function h(v) { v = Math.max(0, Math.min(255, Math.round(v * 255))); return ('0' + v.toString(16)).slice(-2); }
  return '#' + h(c[0]) + h(c[1]) + h(c[2]);
}

/** レイヤーの色（sRGB 0〜1）を集める。 */
function __colors(l) {
  var out = [], k = l.kind || {};
  if (k.type === 'solid' && k.color) out.push(k.color);
  if (k.type === 'text' && k.document && k.document.fill) out.push(k.document.fill);
  if (k.type === 'shape') (k.items || []).forEach(function (it) { if (it.color && it.color.value) out.push(it.color.value); });
  (l.effects || []).forEach(function (fx) { for (var n in fx.params) { var p = fx.params[n]; if (p && p.t === 'color' && p.v && p.v.value) out.push(p.v.value); } });
  return out;
}

function __scan(s) {
  // 前回までの履歴に、今のプロジェクトで初めて見る色を先頭に積む（最近使った順）
  var seen = {};
  (s.history || []).forEach(function (h) { seen[h] = true; });
  var now = [];
  project.compositions.forEach(function (c) { c.layers.forEach(function (l) { __colors(l).forEach(function (col) { now.push(__hex(col)); }); }); });
  var fresh = now.filter(function (h, i) { return !seen[h] && now.indexOf(h) === i; });
  s.history = fresh.concat(s.history || []).slice(0, MAX);
}

function panel(s) {
  if (!s.history) s.history = [];
  __scan(s);
  var items = [{ type: 'heading', text: '色の履歴' }];
  for (var i = 0; i < s.history.length; i += 4) {
    var row = [];
    for (var j = i; j < Math.min(i + 4, s.history.length); j++) row.push({ type: 'button', id: 'use:' + j, label: s.history[j] });
    items.push({ type: 'row', items: row });
  }
  items.push({ type: 'hint', text: '色を押すと、選んだレイヤーの色にします。' });
  items.push({ type: 'row', items: [{ type: 'button', id: 'copy', label: '先頭の色をコピー' }, { type: 'button', id: 'clear', label: '履歴を消す' }] });
  return items;
}

function onEvent(id, value, s) {
  if (!s.history) s.history = [];
  if (id === 'clear') { s.history = []; return; }
  if (id === 'copy') { if (s.history[0]) host.copyText(s.history[0]); return; }
  var m = /^use:(\d+)$/.exec(id);
  if (!m) return;
  var hex = s.history[Number(m[1])];
  if (!hex) return;
  var col = [1, 3, 5].map(function (i) { return parseInt(hex.substr(i, 2), 16) / 255; });
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  ids.forEach(function (lid) {
    var l = layer(c, lid), k = l && l.kind;
    if (!k) return;
    if (k.type === 'solid') k.color = col;
    else if (k.type === 'text') k.document.fill = col.concat([k.document.fill ? k.document.fill[3] : 1]);
    else if (k.type === 'shape') (k.items || []).forEach(function (it) { if (it.color && it.color.value) it.color.value = col.concat([it.color.value[3]]); });
  });
  // 使った色を先頭へ
  s.history = [hex].concat(s.history.filter(function (h) { return h !== hex; }));
}
