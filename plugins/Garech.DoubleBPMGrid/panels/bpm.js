// BPMグリッド倍化（Garech）の移植: コンポのビートグリッドの BPM を N 倍・N 分の 1 にする、基準をずらす、タップで BPM を測る。

var DEF = { ratio: 2, taps: [] };
function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = JSON.parse(JSON.stringify(DEF[k])); return s; }

function __grid(c) {
  if (!c.beat_grid) c.beat_grid = { bpm: 120, beats_per_bar: 4, subdivision: 1, offset: 0 };
  return c.beat_grid;
}

function panel(s) {
  defaults(s);
  var c = comp(ctx.comp), g = c.beat_grid;
  var fps = c.frame_rate.num / c.frame_rate.den;
  var info = g ? 'BPM ' + g.bpm + ' / ' + g.beats_per_bar + ' 拍子 / オフセット ' + (seconds(g.offset || 0)).toFixed(3) + ' 秒（' + Math.round(seconds(g.offset || 0) * fps) + ' フレーム）' : 'ビートグリッドなし';
  var n = Math.round(s.ratio);
  var items = [
    { type: 'heading', text: 'BPMグリッド倍化' },
    { type: 'label', text: info },
    { type: 'slider', id: 'ratio', label: '変更倍率', min: 2, max: 8, step: 1, value: s.ratio },
    { type: 'row', items: [{ type: 'button', id: 'div', label: '÷' + n }, { type: 'button', id: 'mul', label: '×' + n }] },
    { type: 'row', items: [{ type: 'button', id: 'back', label: '＜ 1 フレーム' }, { type: 'button', id: 'fwd', label: '1 フレーム ＞' }, { type: 'button', id: 'here', label: '＋ ここを基準' }] },
    { type: 'button', id: 'tap', label: 'BPMを測定する（拍に合わせて押す）' }
  ];
  if (s.taps.length >= 2) items.push({ type: 'hint', text: '測定: ' + s.taps.length + ' 回 → BPM ' + __tapBpm(s.taps) });
  return items;
}

function __tapBpm(taps) {
  var n = taps.length - 1;
  return n > 0 ? Math.round(60000 * n / (taps[taps.length - 1] - taps[0])) : 0;
}

function onEvent(id, value, s) {
  defaults(s);
  var c = comp(ctx.comp);
  var fps = c.frame_rate.num / c.frame_rate.den, n = Math.round(s.ratio);
  if (id === 'ratio') { s.ratio = value; return; }
  var g = __grid(c);
  if (id === 'mul') g.bpm = Math.round(g.bpm * n * 1000) / 1000;
  else if (id === 'div') g.bpm = Math.round(g.bpm / n * 1000) / 1000;
  else if (id === 'back' || id === 'fwd') g.offset = (g.offset || 0) + (id === 'fwd' ? 1 : -1) * flicks(1 / fps);
  else if (id === 'here') g.offset = flicks(Math.round(ctx.time * fps) / fps);
  else if (id === 'tap') {
    var now = Date.now();
    // 2 秒あいたら測り直す
    if (s.taps.length && now - s.taps[s.taps.length - 1] > 2000) s.taps = [];
    s.taps.push(now);
    if (s.taps.length > 32) s.taps.shift();
    if (s.taps.length >= 2) { g.bpm = __tapBpm(s.taps); g.beats_per_bar = 4; }
  }
}
