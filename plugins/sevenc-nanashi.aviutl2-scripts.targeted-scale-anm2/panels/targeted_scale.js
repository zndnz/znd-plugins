// 目標指定拡大縮小（Nanashi.）の移植: 選んだレイヤーの拡大率に targetedScale のエクスプレッションを付けるパネル。
var DEF = { w: 100, h: 100, mode: 0 };
function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }
function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: '目標指定拡大縮小' },
    { type: 'row', items: [{ type: 'number', id: 'w', label: '目標幅', value: s.w }, { type: 'number', id: 'h', label: '目標高さ', value: s.h }] },
    { type: 'choice', id: 'mode', label: 'モード', options: ['下限指定', '上限指定', '幅指定', '高さ指定'], value: s.mode },
    { type: 'row', items: [{ type: 'button', id: 'apply', label: '付ける' }, { type: 'button', id: 'clear', label: '外す' }] }
  ];
}
function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'apply' && id !== 'clear') { s[id] = value; return; }
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  ids.forEach(function (lid) {
    var p = layer(c, lid).transform.scale;
    if (id === 'clear') { if (p.expression && p.expression.source.indexOf('targetedScale(') >= 0) delete p.expression; return; }
    p.expression = { source: 'targetedScale(value, ' + Number(s.w) + ', ' + Number(s.h) + ', ' + Math.round(s.mode) + ')', enabled: true };
  });
}
