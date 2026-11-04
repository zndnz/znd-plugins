// Scatter のパネル: 選んだレイヤーの位置・回転・スケールに、レイヤーごとに違うずれを付けるエクスプレッションを入れる。
// ずらす量は、各レイヤーに足す「Scatter」スライダー（%）で動かせる（キーフレームを打てる）。

var FIELDS = [['x', 'x', -160, 160], ['y', 'y', -90, 90], ['rz', '回転', 0, 0], ['zoom', '拡大（%）', 100, 100]];

function defaults(s) {
  if (s.seed == null) s.seed = 0;
  FIELDS.forEach(function (f) { if (s[f[0] + '_min'] == null) { s[f[0] + '_min'] = f[2]; s[f[0] + '_max'] = f[3]; } });
  return s;
}

function panel(state) {
  defaults(state);
  var items = [{ type: 'heading', text: 'Scatter（ばらけさせる）' },
    { type: 'hint', text: '選んだレイヤーごとに、乱数で位置・回転・拡大をずらします。ずれの量は各レイヤーの「Scatter」スライダー（%）で動かします。' },
    { type: 'number', id: 'seed', label: 'シード', value: state.seed, speed: 1 }];
  FIELDS.forEach(function (f) {
    items.push({ type: 'row', items: [
      { type: 'number', id: f[0] + '_min', label: f[1] + '(min)', value: state[f[0] + '_min'] },
      { type: 'number', id: f[0] + '_max', label: f[1] + '(max)', value: state[f[0] + '_max'] }] });
  });
  items.push({ type: 'row', items: [{ type: 'button', id: 'apply', label: '選択レイヤーに付ける' }, { type: 'button', id: 'clear', label: '外す' }] });
  return items;
}

function rangeText(s) {
  return '{x:[' + s.x_min + ',' + s.x_max + '],y:[' + s.y_min + ',' + s.y_max + '],rz:[' + s.rz_min + ',' + s.rz_max + '],zoom:[' + s.zoom_min + ',' + s.zoom_max + ']}';
}

function onEvent(id, value, state) {
  defaults(state);
  if (id !== 'apply' && id !== 'clear') { state[id] = value; return; }
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  ids.forEach(function (lid, i) {
    var l = layer(c, lid);
    var tr = l.transform;
    if (id === 'clear') {
      delete tr.position.expression; delete tr.rotation.expression; delete tr.scale.expression;
      l.effects = (l.effects || []).filter(function (e) { return e.name !== 'Scatter'; });
      return;
    }
    if (!findEffect(l, 'Scatter')) setParam(addEffect(l, 'slider_control', 'Scatter'), 'slider', 100);
    // 選んだ順番を obj.index の代わりにする
    var call = 'scatter(effect("Scatter")("slider"), ' + state.seed + ', ' + rangeText(state) + ', ' + i + ')';
    tr.position.expression = { source: 'var s = ' + call + '; [value[0] + s.x, value[1] + s.y]', enabled: true };
    tr.rotation.expression = { source: 'value + ' + call + '.rz', enabled: true };
    tr.scale.expression = { source: 'var z = Math.max(0, 1 + ' + call + '.zoom); [value[0] * z, value[1] * z]', enabled: true };
  });
  host.status(ids.length + ' 個のレイヤーを' + (id === 'clear' ? '元に戻しました' : 'ばらけさせました'));
}
