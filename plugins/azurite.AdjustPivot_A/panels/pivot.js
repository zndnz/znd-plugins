// 中心ずらし_A（azurite）の移植: 選んだレイヤーのアンカーポイントを 9 か所のどこかへ移すパネル。

var NAMES = ['左上', '上', '右上', '左', '中心', '右', '左下', '下', '右下'];
var DEF = { ox: 0, oy: 0, move: false };

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  var rows = [];
  for (var y = 0; y < 3; y++) {
    var items = [];
    for (var x = 0; x < 3; x++) items.push({ type: 'button', id: 'p' + (y * 3 + x + 1), label: NAMES[y * 3 + x] });
    rows.push({ type: 'row', items: items });
  }
  return [{ type: 'heading', text: '中心ずらし' }].concat(rows, [
    { type: 'row', items: [{ type: 'number', id: 'ox', label: 'オフセットX', value: s.ox }, { type: 'number', id: 'oy', label: 'オフセットY', value: s.oy }] },
    { type: 'checkbox', id: 'move', label: 'オブジェクトを移動', value: s.move },
    { type: 'hint', text: 'ボタンを押すと、選んだレイヤーのアンカーポイント（と位置）にエクスプレッションを付けます。' },
    { type: 'button', id: 'clear', label: '外す' }
  ]);
}

function isOurs(p, fn) { return p && p.expression && p.expression.source.indexOf(fn + '(') >= 0; }

function onEvent(id, value, s) {
  defaults(s);
  if (id === 'ox' || id === 'oy' || id === 'move') { s[id] = value; return; }
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  ids.forEach(function (lid) {
    var t = layer(c, lid).transform;
    if (id === 'clear') {
      if (isOurs(t.anchor, 'adjustPivotAnchor')) delete t.anchor.expression;
      if (isOurs(t.position, 'adjustPivotPosition')) delete t.position.expression;
      return;
    }
    var coord = Number(id.substring(1)), args = coord + ', ' + Number(s.ox) + ', ' + Number(s.oy);
    t.anchor.expression = { source: 'adjustPivotAnchor(' + args + ')', enabled: true };
    if (s.move) {
      if (isOurs(t.position, 'adjustPivotPosition')) delete t.position.expression;
    } else {
      t.position.expression = { source: 'adjustPivotPosition(value, ' + args + ')', enabled: true };
    }
  });
  host.status(id === 'clear' ? '中心ずらしを外しました' : '中心を「' + NAMES[Number(id.substring(1)) - 1] + '」に移しました');
}
