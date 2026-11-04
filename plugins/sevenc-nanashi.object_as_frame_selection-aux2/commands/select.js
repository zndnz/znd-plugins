// object_as_frame_selection.aux2（Nanashi.）の移植: 選んだレイヤーの範囲をワークエリアにする。
function objectAsFrameSelection() {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  var a = Infinity, b = -Infinity;
  ids.forEach(function (id) { var l = layer(c, id); if (l) { a = Math.min(a, l.in_point); b = Math.max(b, l.out_point); } });
  var fps = c.frame_rate.num / c.frame_rate.den;
  var fa = Math.round(seconds(a) * fps), fb = Math.round(seconds(b) * fps);
  host.setWorkArea(fa, fb);
  host.status('ワークエリアを ' + fa + '〜' + (fb - 1) + ' フレームにしました');
}
