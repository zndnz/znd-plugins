// フィルタ分離（Garech）の移植: レイヤーのエフェクトを調整レイヤーに分ける・上のレイヤーへ移す。

function __targets() {
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) host.status('レイヤーを選択してください');
  return ids;
}

/** エフェクト 1 つにつき 1 つの調整レイヤーを、同じ長さで上に作る（上から元の順）。 */
function splitFilters() {
  var c = comp(ctx.comp), n = 0;
  __targets().forEach(function (id) {
    var l = layer(c, id);
    if (!l || !(l.effects && l.effects.length)) return;
    var fx = l.effects;
    l.effects = [];
    // エフェクトは上から下へ順に掛かるので、最初のエフェクトをいちばん下（元のレイヤーの直上）に置く
    fx.slice().reverse().forEach(function (e) {
      var adj = JSON.parse(JSON.stringify(l));
      adj.id = newId('layer');
      adj.name = e.name || e.kind;
      adj.kind = { type: 'adjustment' };
      adj.effects = [e];
      adj.masks = [];
      adj.parent = null;
      delete adj.time_remap;
      delete adj.track_matte;
      c.layers.splice(c.layers.indexOf(l), 0, adj);
      n++;
    });
  });
  host.status(n + ' 個のエフェクトを分けました');
}

function __joinUp(onlyFirst) {
  var c = comp(ctx.comp), n = 0;
  __targets().forEach(function (id) {
    var l = layer(c, id);
    if (!l || !(l.effects && l.effects.length)) return;
    var up = c.layers[c.layers.indexOf(l) - 1];
    if (!up) { host.status('上にレイヤーがありません'); return; }
    var moved = onlyFirst ? l.effects.splice(0, 1) : l.effects.splice(0, l.effects.length);
    up.effects = (up.effects || []).concat(moved);
    n += moved.length;
  });
  if (n) host.status(n + ' 個のエフェクトを上のレイヤーへ移しました');
}
function joinUp() { __joinUp(false); }
function joinFirstUp() { __joinUp(true); }
