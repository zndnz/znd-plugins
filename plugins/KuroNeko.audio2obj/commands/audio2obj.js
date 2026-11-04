// audio2obj（KuroNeko）の移植: 音声レイヤーのファイル名から、同じ長さのテキスト（セリフ）レイヤーを作る。
// 「キャラクターID_セリフ」の形のファイル名なら、区切りより後ろをセリフにできる。

var SPLIT = '_';

function __basename(p) { var n = String(p || '').split(/[\\/]/).pop(); return n.replace(/\.[^.]+$/, ''); }

function __text(path, split) {
  var t = __basename(path);
  if (split) {
    var parts = t.split(SPLIT).filter(function (s) { return s.length; });
    if (parts.length > 1) t = parts.slice(1).join(' ');
  }
  return t;
}

function __make(split) {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  var n = 0;
  ids.forEach(function (id) {
    var l = layer(c, id);
    var k = l && l.kind;
    if (!k || !(k.type === 'audio' || k.type === 'footage')) return;
    var fi = project.footage.filter(function (f) { return f.id === k.footage; })[0];
    if (!fi) return;
    var t = addText(c, __basename(fi.path), __text(fi.path, split), 64);
    t.in_point = l.in_point;
    t.out_point = l.out_point;
    // 音声レイヤーのすぐ上に置く
    c.layers.splice(c.layers.indexOf(t), 1);
    c.layers.splice(c.layers.indexOf(l), 0, t);
    n++;
  });
  host.status(n ? n + ' 個のセリフを作りました' : '音声のレイヤーを選んでください');
}

function fromFileName() { __make(false); }
function fromFileNameWithId() { __make(true); }
