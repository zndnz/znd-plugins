// 選んだレイヤーの位置に、ピクセルに合わせるエクスプレッションを付ける・外す。

var SRC = 'pixelSnap(value, anchorPoint, scale, rotation)';

function targets() {
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) host.status('レイヤーを選択してください');
  return ids;
}

function snap() {
  var c = comp(ctx.comp);
  var ids = targets();
  ids.forEach(function (id) { layer(c, id).transform.position.expression = { source: SRC, enabled: true }; });
  if (ids.length) host.status(ids.length + ' 個のレイヤーをピクセルに合わせました');
}

function unsnap() {
  var c = comp(ctx.comp);
  var ids = targets();
  ids.forEach(function (id) {
    var p = layer(c, id).transform.position;
    if (p.expression && p.expression.source === SRC) delete p.expression;
  });
}
