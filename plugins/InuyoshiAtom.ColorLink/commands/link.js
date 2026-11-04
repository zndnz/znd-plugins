// カラーリンク: 選んだレイヤーに「カラーリンク」エフェクトを付け、その色を、
// 自分より上にある最も近い「カラーマスター」の色 A/B/C にエクスプレッションでつなぐ。

var MASTER = 'plugin:InuyoshiAtom.ColorLink.master';
var LINK = 'plugin:InuyoshiAtom.ColorLink.link';

function linkTo(group) {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  var param = { A: 'color_a', B: 'color_b', C: 'color_c' }[group];
  var linked = 0, missing = 0;
  ids.forEach(function (id) {
    var i = c.layers.findIndex(function (l) { return l.id === id; });
    // 上（番号の小さい側）へ向かって最も近いカラーマスター
    var master = null, masterFx = null;
    for (var j = i - 1; j >= 0 && !master; j--) {
      var m = (c.layers[j].effects || []).filter(function (e) { return e.kind === MASTER; })[0];
      if (m) { master = c.layers[j]; masterFx = m; }
    }
    if (!master) { missing++; return; }
    var l = c.layers[i];
    var fx = (l.effects || []).filter(function (e) { return e.kind === LINK; })[0] || addEffect(l, LINK, 'カラーリンク');
    fx.params.color.v.expression = {
      source: 'thisComp.layer(' + JSON.stringify(master.name) + ').effect(' + JSON.stringify(masterFx.name) + ')(' + JSON.stringify(param) + ')',
      enabled: true
    };
    linked++;
  });
  host.status(linked + ' 個のレイヤーを色' + group + 'につなぎました' + (missing ? '（' + missing + ' 個は上にカラーマスターがありません）' : ''));
}

function linkA() { linkTo('A'); }
function linkB() { linkTo('B'); }
function linkC() { linkTo('C'); }

function addMaster() {
  var c = comp(ctx.comp);
  var l = ctx.layer ? layer(c, ctx.layer) : null;
  if (!l) { host.status('カラーマスターを付けるレイヤーを選択してください'); return; }
  if ((l.effects || []).some(function (e) { return e.kind === MASTER; })) { host.status('すでにカラーマスターがあります'); return; }
  addEffect(l, MASTER, 'カラーマスター');
  host.status('「' + l.name + '」にカラーマスターを付けました');
}
