// LyricMotioN のパネル: 選んだテキストレイヤーに、文字ごとのアニメーター（エクスプレッションセレクター）を付ける。

var DEF = { type: 0, order: 0, seedOrder: 0, delay: 10, seed: 0, vector: 100, hide: true,
  dx: true, dmx: true, dy: true, dmy: true, allowSame: false, fade: false, zoom: 100, ease: 2 };
var PREFIX = 'LyricMotioN: ';

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: 'LyricMotioN' },
    { type: 'hint', text: 'テキストレイヤーを選んで「付ける」。レイヤーの長さ全体で 1 文字ずつ入って（出て）いきます。長さを変えたら付け直してください。' },
    { type: 'choice', id: 'type', label: 'タイプ', options: ['In', 'Out'], value: s.type },
    { type: 'choice', id: 'order', label: '順序', options: ['先頭から', '末尾から', '中央から', '外側から', 'ランダム'], value: s.order },
    { type: 'number', id: 'seedOrder', label: '順序用シード', value: s.seedOrder, speed: 1 },
    { type: 'choice', id: 'ease', label: '動き', options: ['直線', '二次', '三次', '指数', 'バック'], value: s.ease },
    { type: 'slider', id: 'delay', label: 'ズレ（%）', min: 0, max: 100, value: s.delay },
    { type: 'number', id: 'vector', label: '移動距離', value: s.vector },
    { type: 'number', id: 'seed', label: 'シード', value: s.seed, speed: 1 },
    { type: 'row', items: [{ type: 'checkbox', id: 'dx', label: 'X', value: s.dx }, { type: 'checkbox', id: 'dmx', label: '-X', value: s.dmx },
      { type: 'checkbox', id: 'dy', label: 'Y', value: s.dy }, { type: 'checkbox', id: 'dmy', label: '-Y', value: s.dmy }] },
    { type: 'row', items: [{ type: 'checkbox', id: 'allowSame', label: '被りあり', value: s.allowSame }, { type: 'checkbox', id: 'fade', label: 'フェード', value: s.fade },
      { type: 'checkbox', id: 'hide', label: 'In/Out 非表示', value: s.hide }] },
    { type: 'number', id: 'zoom', label: '拡大率（%）', value: s.zoom },
    { type: 'row', items: [{ type: 'button', id: 'apply', label: '付ける' }, { type: 'button', id: 'clear', label: '外す' }] }
  ];
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
    var dirs = [];
    if (s.dx) dirs.push('x'); if (s.dmx) dirs.push('-x'); if (s.dy) dirs.push('y'); if (s.dmy) dirs.push('-y');
    var p = { type: s.type, order: s.order, seedOrder: s.seedOrder, delay: s.delay / 100, seed: s.seed, vector: s.vector,
      hide: s.hide, fade: s.fade, allowSame: s.allowSame, ease: s.ease, dirs: dirs,
      inPoint: seconds(l.in_point), outPoint: seconds(l.out_point), fps: ctx.fps };
    var add = function (name, props, part) {
      l.kind.animators.push({ name: PREFIX + name, props: props,
        selectors: [{ type: 'expression', unit: 'characters', source: 'lyricMotionAmount(' + JSON.stringify(p) + ', "' + part + '", textIndex, textTotal, time)' }] });
    };
    var d = Math.abs(s.vector);
    add('X', { position: { value: [d, 0] } }, 'x');
    add('-X', { position: { value: [-d, 0] } }, '-x');
    add('Y', { position: { value: [0, d] } }, 'y');
    add('-Y', { position: { value: [0, -d] } }, '-y');
    if (s.zoom !== 100) add('拡大', { scale: { value: [s.zoom, s.zoom] } }, 'zoom');
    add('不透明度', { opacity: { value: 0 } }, 'alpha');
  });
  host.status(done ? done + ' 個のテキストレイヤーに' + (id === 'clear' ? 'から外しました' : '付けました') : 'テキストレイヤーを選択してください');
}
