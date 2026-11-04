// リップル削除（JobsJoestar）の移植: 選んだレイヤーの「先頭から再生ヘッドの手前まで」を取り除き、取り除いた長さだけ詰める。
// レイヤーの中身（素材の再生位置）はずれないよう、開始時刻も一緒に動かす。
// 後ろにあるレイヤー（元の終わり以降に始まるもの）も同じだけ左に詰める。

/** レイヤーのキーフレーム（コンポ時刻）をすべて dt だけずらす。 */
function __shiftKeys(o, dt) {
  if (!o || typeof o !== 'object') return;
  if (Array.isArray(o.keyframes)) o.keyframes.forEach(function (k) { if (typeof k.time === 'number') k.time += dt; });
  for (var k in o) if (k !== 'keyframes') __shiftKeys(o[k], dt);
}

function rippleDelete() {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  if (!ids.length) { host.status('レイヤーを選択してください'); return; }
  var t = flicks(ctx.time);
  var done = [], firstIn = null;
  ids.forEach(function (id) {
    var l = layer(c, id);
    if (!l || l.locked || !(l.in_point < t && t < l.out_point)) return;
    var cut = t - l.in_point, oldOut = l.out_point;
    // 先頭を切り、残りを元の開始位置へ寄せる（中身は切ったところから続く）
    l.start_time -= cut;
    l.out_point -= cut;
    (l.markers || []).forEach(function (m) { m.time -= cut; });
    __shiftKeys(l.kind, -cut); __shiftKeys(l.transform, -cut); __shiftKeys(l.effects, -cut); __shiftKeys(l.masks, -cut); __shiftKeys(l.time_remap, -cut); __shiftKeys(l.audio, -cut);
    c.layers.forEach(function (o) {
      if (o === l || o.locked || ids.indexOf(o.id) >= 0) return;
      if (o.in_point >= oldOut) {
        o.in_point -= cut;
        o.out_point -= cut;
        o.start_time -= cut;
        (o.markers || []).forEach(function (m) { m.time -= cut; });
        __shiftKeys(o.kind, -cut); __shiftKeys(o.transform, -cut); __shiftKeys(o.effects, -cut); __shiftKeys(o.masks, -cut); __shiftKeys(o.time_remap, -cut); __shiftKeys(o.audio, -cut);
      }
    });
    done.push(l.id);
    if (firstIn === null || l.in_point < firstIn) firstIn = l.in_point;
  });
  if (!done.length) { host.status('再生ヘッドが選んだレイヤーの範囲の中にありません'); return; }
  host.setTime(seconds(firstIn));
  host.select(done[0]);
  host.status('リップル削除しました');
}
