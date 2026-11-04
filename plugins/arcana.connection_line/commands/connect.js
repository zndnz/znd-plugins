// 選んだ 2 つのレイヤーを結ぶ「連結ライン」を作る。コンポの大きさの平面を足し、
// 始点・終点をそれぞれのレイヤーのアンカーポイントの位置（コンポ上）へエクスプレッションでつなぐ。

function pointExpr(name, dx, dy) {
  var L = 'thisComp.layer(' + JSON.stringify(name) + ')';
  return 'var q = ' + L + '.toComp(' + L + '.anchorPoint); thisLayer.fromComp([q[0] + ' + dx + ', q[1] + ' + dy + '])';
}

function connect() {
  var c = comp(ctx.comp);
  if (ctx.layers.length < 2) { host.status('つなぐレイヤーを 2 つ選択してください'); return; }
  var a = layer(c, ctx.layers[0]), b = layer(c, ctx.layers[1]);
  var line = addSolid(c, '連結ライン（' + a.name + ' — ' + b.name + '）');
  var fx = addEffect(line, 'plugin:arcana.connection_line', '連結ライン');
  fx.params.start.v.expression = { source: pointExpr(a.name, 0, 0), enabled: true };
  fx.params.end.v.expression = { source: pointExpr(b.name, 0, 0), enabled: true };
  host.select(line.id);
  host.status('「' + a.name + '」と「' + b.name + '」を線でつなぎました');
}
