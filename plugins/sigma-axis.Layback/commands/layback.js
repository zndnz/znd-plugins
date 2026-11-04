// 最背面に配置（σ軸）の移植。
// 元のスクリプトは、それまでに描いた絵（フレームバッファ）の後ろにオブジェクトを描く。
// znd ではレイヤーの重なり順そのものを変えるので、選んだレイヤーをコンポのいちばん下（最背面）へ動かす。

function sendToBack() {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  var moving = [];
  c.layers = c.layers.filter(function (l) {
    if (ids.indexOf(l.id) >= 0) { moving.push(l); return false; }
    return true;
  });
  // 選んだレイヤー同士の上下関係は保つ
  c.layers = c.layers.concat(moving);
  host.status(moving.length + ' 個のレイヤーを最背面に置きました');
}

function bringToFront() {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  var moving = [];
  c.layers = c.layers.filter(function (l) {
    if (ids.indexOf(l.id) >= 0) { moving.push(l); return false; }
    return true;
  });
  c.layers = moving.concat(c.layers);
  host.status(moving.length + ' 個のレイヤーを最前面に置きました');
}
