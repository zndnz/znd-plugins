// TextSplitter（なたでここ）の移植: テキストレイヤーを 1 文字ずつのレイヤーに分ける。
// 分けたレイヤーは元と同じ文章を持ち、「その文字以外を透明にする」アニメーターで 1 文字だけを見せる
// （文字の位置・フォント・行送りが元とまったく同じになる）。

var NAME = 'TextSplitter';

function splitText() {
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  var made = 0;
  ids.forEach(function (id) {
    var l = layer(c, id);
    if (!l || l.kind.type !== 'text') return;
    var chars = Array.from(l.kind.document.text.replace(/\n/g, ''));
    var at = c.layers.indexOf(l);
    var news = [];
    chars.forEach(function (ch, k) {
      if (/\s/.test(ch)) return;
      var n = JSON.parse(JSON.stringify(l));
      n.id = newId('layer');
      n.name = l.name + ' ' + (k + 1) + '「' + ch + '」';
      n.kind.animators = (n.kind.animators || []).concat([{ name: NAME, props: { opacity: { value: 0 } },
        selectors: [{ type: 'expression', unit: 'characters', source: 'textIndex - 1 === ' + k + ' ? 0 : 100' }] }]);
      news.push(n);
      c.layers.push(n); // newId が使った ID を数えるよう、いったん足しておく
    });
    news.forEach(function (n) { c.layers.splice(c.layers.indexOf(n), 1); });
    // 元のレイヤーの位置に、1 文字目が上になるよう並べ、元のレイヤーは消す
    c.layers.splice.apply(c.layers, [at, 1].concat(news));
    made += news.length;
  });
  host.status(made ? made + ' 文字に分けました' : 'テキストレイヤーを選択してください');
}
