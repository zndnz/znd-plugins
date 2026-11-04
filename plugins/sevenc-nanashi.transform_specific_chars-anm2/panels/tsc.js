// transform_specific_chars.anm2（sevenc-nanashi）の移植: テキストの中の特定の文字だけを動かす・回す・拡大する・色を変える。
// 選んだテキストレイヤーに、対象の文字だけを選ぶエクスプレッションセレクターのアニメーターを付ける。

var DEF = { chars: '', invert: false, regex: false, dx: 0, dy: 0, rz: 0, zoom: 100, sx: 100, sy: 100, transparency: 0, useColor: false, color: [1, 0, 0, 1] };
var NAME = 'transform_specific_chars';

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: '特定の文字を変形' },
    { type: 'text', id: 'chars', label: '対象文字', value: s.chars },
    { type: 'row', items: [{ type: 'checkbox', id: 'regex', label: '正規表現', value: s.regex }, { type: 'checkbox', id: 'invert', label: '対象文字の反転', value: s.invert }] },
    { type: 'row', items: [{ type: 'number', id: 'dx', label: 'X移動量', value: s.dx }, { type: 'number', id: 'dy', label: 'Y移動量', value: s.dy }] },
    { type: 'number', id: 'rz', label: '回転', value: s.rz },
    { type: 'row', items: [{ type: 'number', id: 'zoom', label: '拡大率', value: s.zoom }, { type: 'number', id: 'sx', label: 'X', value: s.sx }, { type: 'number', id: 'sy', label: 'Y', value: s.sy }] },
    { type: 'slider', id: 'transparency', label: '透明度', min: 0, max: 100, value: s.transparency },
    { type: 'row', items: [{ type: 'checkbox', id: 'useColor', label: '文字色を変える', value: s.useColor }, { type: 'color', id: 'color', label: '文字色', value: s.color }] },
    { type: 'hint', text: 'テキストを書き換えたら付け直してください（対象の文字の位置を付けたときに数えます）。' },
    { type: 'row', items: [{ type: 'button', id: 'apply', label: '付ける' }, { type: 'button', id: 'clear', label: '外す' }] }
  ];
}

function targets(text, s) {
  var chars = Array.from(text.replace(/\n/g, '')), out = [];
  var re = null;
  if (s.regex && s.chars) re = new RegExp(s.chars, 'g');
  var hit = chars.map(function () { return false; });
  if (re) {
    // 文字列の中で一致した範囲の文字を対象にする
    var flat = chars.join(''), m;
    while ((m = re.exec(flat)) !== null) {
      var start = Array.from(flat.substring(0, m.index)).length, len = Array.from(m[0]).length;
      for (var i = 0; i < len; i++) hit[start + i] = true;
      if (m[0].length === 0) re.lastIndex++;
    }
  } else {
    var set = Array.from(s.chars);
    chars.forEach(function (ch, i) { hit[i] = set.indexOf(ch) >= 0; });
  }
  hit.forEach(function (h, i) { if (h !== !!s.invert) out.push(i); });
  return out;
}

function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'apply' && id !== 'clear') { s[id] = value; return; }
  var c = comp(ctx.comp), n = 0;
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  ids.forEach(function (lid) {
    var l = layer(c, lid);
    if (l.kind.type !== 'text') return;
    n++;
    l.kind.animators = (l.kind.animators || []).filter(function (a) { return a.name !== NAME; });
    if (id === 'clear') return;
    var list = targets(l.kind.document.text, s);
    var props = {};
    if (s.dx || s.dy) props.position = { value: [s.dx, s.dy] };
    if (s.rz) props.rotation = { value: s.rz };
    if (s.zoom !== 100 || s.sx !== 100 || s.sy !== 100) props.scale = { value: [s.zoom * s.sx / 100, s.zoom * s.sy / 100] };
    if (s.transparency) props.opacity = { value: 100 - s.transparency };
    if (s.useColor) props.fill_color = { value: s.color };
    l.kind.animators.push({ name: NAME, props: props,
      selectors: [{ type: 'expression', unit: 'characters', source: JSON.stringify(list) + '.indexOf(textIndex - 1) >= 0 ? 100 : 0' }] });
  });
  host.status(n ? n + ' 個のテキストレイヤーに' + (id === 'clear' ? 'から外しました' : '付けました') : 'テキストレイヤーを選択してください');
}
