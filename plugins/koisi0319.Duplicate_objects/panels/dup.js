// オブジェクト行複製（koisi0319）の移植: 選んだレイヤーを、すき間（フレーム）を空けて後ろへ何回も複製する。
// 回数で決めるか、「このフレームまで」で決めるかを選べる。

var DEF = { gap: 0, mode: 0, count: 1, limit: 300 };

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: 'オブジェクト行複製' },
    { type: 'number', id: 'gap', label: 'すき間（フレーム）', value: s.gap },
    { type: 'choice', id: 'mode', label: '複製の数', options: ['回数で指定', 'フレームまで'], value: s.mode },
    s.mode === 0 ? { type: 'number', id: 'count', label: '回数', value: s.count } : { type: 'number', id: 'limit', label: '開始がこのフレームまで', value: s.limit },
    { type: 'button', id: 'apply', label: '複製する' }
  ];
}

/** レイヤーのキーフレーム（コンポ時刻）をすべて dt だけずらす。 */
function __shiftKeys(o, dt) {
  if (!o || typeof o !== 'object') return;
  if (Array.isArray(o.keyframes)) o.keyframes.forEach(function (k) { if (typeof k.time === 'number') k.time += dt; });
  for (var k in o) if (k !== 'keyframes') __shiftKeys(o[k], dt);
}

function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'apply') { s[id] = value; return; }
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  var frame = flicks(c.frame_rate.den / c.frame_rate.num);
  var made = 0;
  ids.forEach(function (id) {
    var src = layer(c, id);
    var len = src.out_point - src.in_point;
    if (len <= 0) return;
    var step = len + Math.round(Number(s.gap)) * frame;
    var limit = Math.round(Number(s.limit)) * frame;
    var n = s.mode === 0 ? Math.max(0, Math.round(Number(s.count))) : 1e6;
    for (var k = 1; k <= n; k++) {
      var start = src.in_point + step * k;
      if (s.mode === 1 && start > limit) break;
      if (start >= c.duration) break;
      var d = JSON.parse(JSON.stringify(src));
      d.id = newId('layer');
      d.name = src.name + ' ' + (k + 1);
      d.in_point = start;
      d.out_point = start + len;
      d.start_time = src.start_time + step * k;
      (d.markers || []).forEach(function (m) { m.time += step * k; });
      __shiftKeys(d.kind, step * k); __shiftKeys(d.transform, step * k); __shiftKeys(d.effects, step * k); __shiftKeys(d.masks, step * k); __shiftKeys(d.time_remap, step * k); __shiftKeys(d.audio, step * k);
      c.layers.splice(c.layers.indexOf(src), 0, d);
      made++;
    }
  });
  host.status(made + ' 個複製しました');
}
