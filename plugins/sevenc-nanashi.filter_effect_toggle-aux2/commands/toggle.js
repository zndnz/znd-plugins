// filter_effect_toggle.aux2（Nanashi.）の移植: 「レイヤーに掛けたエフェクト」と「調整レイヤー（フィルタオブジェクト）」を切り替える。
// 普通のレイヤーを選ぶと、そのエフェクトを同じ長さの調整レイヤーに移して上に置く。
// 調整レイヤーを選ぶと、そのエフェクトをすぐ下のレイヤーに移して調整レイヤーを消す。

function toggleFilter() {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  var n = 0;
  ids.forEach(function (id) {
    var l = layer(c, id);
    if (!l) return;
    var i = c.layers.indexOf(l);
    if (l.kind.type === 'adjustment') {
      var below = c.layers[i + 1];
      if (!below) { host.status('調整レイヤーの下にレイヤーがありません'); return; }
      below.effects = (below.effects || []).concat(l.effects || []);
      c.layers.splice(i, 1);
      n++;
    } else {
      if (!(l.effects && l.effects.length)) return;
      var adj = JSON.parse(JSON.stringify(l));
      adj.id = newId('layer');
      adj.name = l.name + '（エフェクト）';
      adj.kind = { type: 'adjustment' };
      adj.masks = [];
      adj.parent = null;
      delete adj.time_remap;
      delete adj.track_matte;
      l.effects = [];
      c.layers.splice(i, 0, adj);
      n++;
    }
  });
  host.status(n ? n + ' 個切り替えました' : '切り替えるエフェクトがありません');
}
