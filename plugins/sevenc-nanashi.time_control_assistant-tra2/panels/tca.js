// time_control_assistant のパネル: 選んだレイヤー（動画・プリコンポ）のタイムリマップにエクスプレッションを付ける。

var DEF = { mode: 0, unit: 1, period: 1, offset: 0, a: 0, b: 1, divisions: 4, units: 1, divOffset: 0, fromMeasure: false };
var MODES = ['範囲（絶対）', '範囲（相対）', '繰り返し', '往復', '逆往復'];
var KEYS = ['absolute', 'relative', 'repeat', 'forthback', 'reverse'];
var UNITS = ['フレーム', '秒', 'BPM（拍）', '周波数（Hz）', '回数', 'BPM グリッド'];

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  var items = [
    { type: 'heading', text: 'time_control_assistant' },
    { type: 'hint', text: '動画・プリコンポのレイヤーを選んで「付ける」。タイムリマップがエクスプレッションで作られます。' },
    { type: 'choice', id: 'mode', label: '種類', options: MODES, value: s.mode }
  ];
  if (s.mode <= 1) {
    items.push({ type: 'choice', id: 'unit', label: '単位', options: ['フレーム', '秒'], value: Math.min(s.unit, 1) });
    items.push({ type: 'number', id: 'a', label: s.mode === 0 ? '開始位置' : '開始位置シフト', value: s.a });
    items.push({ type: 'number', id: 'b', label: s.mode === 0 ? '終了位置' : '終了位置シフト', value: s.b });
  } else {
    items.push({ type: 'choice', id: 'unit', label: '周期の単位', options: UNITS, value: s.unit });
    if (s.unit === 5) {
      items.push({ type: 'number', id: 'divisions', label: '分割数', value: s.divisions, speed: 1 });
      items.push({ type: 'number', id: 'units', label: '分割後の個数', value: s.units, speed: 1 });
      items.push({ type: 'number', id: 'divOffset', label: '分割ずれ', value: s.divOffset });
      items.push({ type: 'checkbox', id: 'fromMeasure', label: '繰り返す位置を小節の先頭にする', value: s.fromMeasure });
    } else {
      items.push({ type: 'number', id: 'period', label: ['周期（フレーム）', '周期（秒）', '拍数（BPM はコンポのビートグリッド）', '周波数（Hz）', '回数'][s.unit], value: s.period });
      items.push({ type: 'number', id: 'offset', label: ['周期ずれ（フレーム）', '周期ずれ（秒）', '拍ずれ', '位相ずれ（°）', '周期ずれ（回）'][s.unit], value: s.offset });
    }
  }
  items.push({ type: 'row', items: [{ type: 'button', id: 'apply', label: '付ける' }, { type: 'button', id: 'clear', label: '外す' }] });
  return items;
}

function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'apply' && id !== 'clear') { s[id] = value; return; }
  var c = comp(ctx.comp), fps = ctx.fps, n = 0;
  var grid = c.beat_grid || null;
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  ids.forEach(function (lid) {
    var l = layer(c, lid);
    if (id === 'clear') { delete l.time_remap; n++; return; }
    var inP = seconds(l.in_point), outP = seconds(l.out_point), dur = outP - inP;
    var base = (inP - seconds(l.start_time)) * (l.speed || 1);
    var p = { mode: KEYS[s.mode], inPoint: inP, outPoint: outP, base: base, a: 0, b: 0, period: 1, offset: 0, grid: null };
    var conv = function (v) { return s.unit === 0 ? v / fps : v; };
    if (s.mode <= 1) { p.a = conv(s.a); p.b = conv(s.b); if (s.mode === 0 && s.unit === 0) { p.a = (s.a - 1) / fps; p.b = (s.b - 1) / fps; } }
    else if (s.unit === 5) {
      if (!grid) { host.status('BPM グリッドを使うには、コンポにビートグリッドを設定してください'); return; }
      p.grid = { bpm: grid.bpm, beats: grid.beats_per_bar || 4, offset: seconds(grid.offset || 0), divisions: Math.max(1, s.divisions), units: Math.max(1, s.units), divOffset: s.divOffset, fromMeasure: !!s.fromMeasure };
    } else {
      var bpm = grid ? grid.bpm : 120;
      var period = [s.period / fps, s.period, 60 / bpm * s.period, 1 / Math.max(1e-6, s.period), dur / Math.max(1e-6, s.period)][s.unit];
      var off = [s.offset / fps, s.offset, 60 / bpm * s.offset, period * s.offset / 360, s.offset * period][s.unit];
      p.period = period; p.offset = off;
    }
    l.time_remap = { value: base, expression: { source: 'tcaTime(' + JSON.stringify(p) + ', time)', enabled: true } };
    n++;
  });
  if (n) host.status(n + ' 個のレイヤーのタイムリマップを' + (id === 'clear' ? '外しました' : '設定しました'));
}
