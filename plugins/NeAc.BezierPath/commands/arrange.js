// 選んだレイヤーを、BezierPath のガイドの曲線に沿って並べる（ガイドが無ければ作る）。

function arrange() {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('並べるレイヤーを選択してください'); return; }
  var guide = null;
  c.layers.forEach(function (l) { if (!guide && findEffect(l, 'BezierPath')) guide = l; });
  if (!guide) {
    guide = addSolid(c, 'BezierPath ガイド');
    addEffect(guide, 'plugin:NeAc.BezierPath', 'BezierPath');
  }
  var name = JSON.stringify(guide.name);
  ids.forEach(function (id, i) {
    if (id === guide.id) return;
    var tr = layer(c, id).transform;
    var call = 'bezierPath(thisComp, ' + name + ', ' + i + ', ' + ids.length + ')';
    tr.position.expression = { source: call + '.position', enabled: true };
    tr.rotation.expression = { source: 'thisComp.layer(' + name + ').effect("BezierPath")("fix") ? value + ' + call + '.rotation : value', enabled: true };
    tr.opacity.expression = { source: call + '.visible ? value : 0', enabled: true };
  });
  host.status(ids.length + ' 個のレイヤーを「' + guide.name + '」の曲線に並べました');
}
