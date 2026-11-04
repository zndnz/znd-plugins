// AutoLyricAnimation_K のパネル: 選んだテキストレイヤーに、文字ごとのアニメーター（エクスプレッションセレクター）を付ける。

var DEF = { maxDist: 500, spread: 50, speed: 4, interval: 8, moveX: true, moveY: true, omni: true, inv: false, type: 0,
  entryScale: 200, entryFlip: false, exitScale: 200, exitFlip: false, seed: -1, ease: 2 };
var PREFIX = 'AutoLyricAnimation_K: ';

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: 'AutoLyricAnimation_K' },
    { type: 'hint', text: 'テキストレイヤーを選んで「付ける」。文字ごとに少しずつずれて、ランダムな方向から入って（出て）いきます。レイヤーの長さを変えたら付け直してください。' },
    { type: 'choice', id: 'type', label: '種類', options: ['In', 'Out', 'In/Out'], value: s.type },
    { type: 'choice', id: 'ease', label: '動き', options: ['直線', '二次', '三次', '指数', 'バック'], value: s.ease },
    { type: 'slider', id: 'maxDist', label: '最大距離', min: 0, max: 2000, value: s.maxDist },
    { type: 'slider', id: 'spread', label: 'ばらつき（%）', min: 0, max: 100, value: s.spread },
    { type: 'slider', id: 'speed', label: '速さ', min: 0.1, max: 100, value: s.speed },
    { type: 'slider', id: 'interval', label: '間隔', min: 0, max: 100, value: s.interval },
    { type: 'row', items: [{ type: 'checkbox', id: 'moveX', label: 'X に動く', value: s.moveX }, { type: 'checkbox', id: 'moveY', label: 'Y に動く', value: s.moveY }] },
    { type: 'row', items: [{ type: 'checkbox', id: 'omni', label: '両方向', value: s.omni }, { type: 'checkbox', id: 'inv', label: '向きを反転', value: s.inv }] },
    { type: 'row', items: [{ type: 'number', id: 'entryScale', label: '入りの拡大（%）', value: s.entryScale }, { type: 'checkbox', id: 'entryFlip', label: 'ランダムに反転', value: s.entryFlip }] },
    { type: 'row', items: [{ type: 'number', id: 'exitScale', label: '出の拡大（%）', value: s.exitScale }, { type: 'checkbox', id: 'exitFlip', label: 'ランダムに反転', value: s.exitFlip }] },
    { type: 'number', id: 'seed', label: 'シード', value: s.seed, speed: 1 },
    { type: 'row', items: [{ type: 'button', id: 'apply', label: '付ける' }, { type: 'button', id: 'clear', label: '外す' }] }
  ];
}

function animator(name, props, src) {
  return { name: PREFIX + name, selectors: [{ type: 'expression', source: src, unit: 'characters' }], props: props };
}

function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'apply' && id !== 'clear') { s[id] = value; return; }
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  var done = 0;
  ids.forEach(function (lid) {
    var l = layer(c, lid);
    if (l.kind.type !== 'text') return;
    l.kind.animators = (l.kind.animators || []).filter(function (a) { return a.name.indexOf(PREFIX) !== 0; });
    done++;
    if (id === 'clear') return;
    var p = {};
    for (var k in DEF) p[k] = s[k];
    p.inPoint = seconds(l.in_point); p.outPoint = seconds(l.out_point); p.fps = ctx.fps;
    var call = function (part) { return 'alaAmount(' + JSON.stringify(p) + ', "' + part + '", textIndex, textTotal, time)'; };
    var d = s.maxDist;
    l.kind.animators.push(
      animator('+X', { position: { value: [d, 0] } }, call('px')),
      animator('-X', { position: { value: [-d, 0] } }, call('nx')),
      animator('+Y', { position: { value: [0, d] } }, call('py')),
      animator('-Y', { position: { value: [0, -d] } }, call('ny')),
      animator('入りの拡大', { scale: { value: [s.entryScale, s.entryScale] } }, call('entry')),
      animator('出の拡大', { scale: { value: [s.exitScale, s.exitScale] } }, call('exit')),
      animator('反転', { scale: { value: [-100, 100] } }, call('flip')),
      animator('非表示', { opacity: { value: 0 } }, call('hide')));
  });
  host.status(done ? done + ' 個のテキストレイヤーに' + (id === 'clear' ? 'から外しました' : '付けました') : 'テキストレイヤーを選択してください');
}
