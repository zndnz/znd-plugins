// SeiSa のパネル: 選んだテキストレイヤーを RTA タイマーにする。区間はそのレイヤーのマーカーで区切る。

var DEF = { odd: false, startFrames: 0, format: 0, digit: 0, frames: 0, frameDigit: 0,
  c1: [0.675, 0.675, 0.675, 1], c2: [0, 0.8, 0.21, 1], c3: [0.48, 0.48, 0.48, 1], c4: [0.086, 0.65, 1, 1] };
var PREFIX = 'SeiSa: ';

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: 'あとのせRTAタイマー（総タイム）' },
    { type: 'hint', text: 'テキストレイヤーを選んで「タイマーにする」。そのレイヤーにマーカーを打つと区間が分かれ、走行中・中断中が交互に切り替わります。' },
    { type: 'checkbox', id: 'odd', label: '奇数番目の区間でカウント', value: s.odd },
    { type: 'number', id: 'startFrames', label: '開始フレーム数', value: s.startFrames, speed: 1 },
    { type: 'choice', id: 'format', label: 'タイム書式', options: ['O:OO:OO.OOO', 'OhOOmOOsOOOms'], value: s.format },
    { type: 'choice', id: 'digit', label: 'タイム桁数', options: ['自動調整', '秒2桁以上', '分1桁以上', '分2桁以上', '時1桁以上', '時2桁以上', '時3桁以上', '時4桁以上'], value: s.digit },
    { type: 'choice', id: 'frames', label: 'フレーム番号', options: ['非表示', '表示（半角スペース後）', '表示（改行後）'], value: s.frames },
    { type: 'number', id: 'frameDigit', label: 'フレーム桁数（最低）', value: s.frameDigit, speed: 1 },
    { type: 'color', id: 'c1', label: '文字色（走行前）', value: s.c1 },
    { type: 'color', id: 'c2', label: '文字色（走行中）', value: s.c2 },
    { type: 'color', id: 'c3', label: '文字色（中断中）', value: s.c3 },
    { type: 'color', id: 'c4', label: '文字色（走行後）', value: s.c4 },
    { type: 'row', items: [{ type: 'button', id: 'apply', label: 'タイマーにする' }, { type: 'button', id: 'clear', label: 'やめる' }] }
  ];
}

function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'apply' && id !== 'clear') { s[id] = value; return; }
  var c = comp(ctx.comp);
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  var n = 0;
  ids.forEach(function (lid) {
    var l = layer(c, lid);
    if (l.kind.type !== 'text') return;
    n++;
    l.kind.animators = (l.kind.animators || []).filter(function (a) { return a.name.indexOf(PREFIX) !== 0; });
    if (id === 'clear') { delete l.kind.text_expression; return; }
    var p = JSON.stringify({ odd: !!s.odd, startFrames: Math.max(0, Math.floor(s.startFrames)), format: s.format, digit: s.digit,
      frames: s.frames, frameDigit: Math.max(0, Math.floor(s.frameDigit)), fps: ctx.fps });
    l.kind.text_expression = { source: 'seisaText(thisLayer, time, ' + p + ')', enabled: true };
    [s.c1, s.c2, s.c3, s.c4].forEach(function (col, k) {
      l.kind.animators.push({ name: PREFIX + ['走行前', '走行中', '中断中', '走行後'][k], props: { fill_color: { value: col } },
        selectors: [{ type: 'expression', unit: 'characters', source: 'seisaStateIs(thisLayer, time, ' + p + ', ' + (k + 1) + ')' }] });
    });
  });
  host.status(n ? n + ' 個のテキストレイヤーを' + (id === 'clear' ? '元に戻しました' : 'タイマーにしました') : 'テキストレイヤーを選択してください');
}
