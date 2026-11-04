// 日付と時刻.obj2（Garech）の移植: 選んだテキストレイヤーの文字を、dateTimeText のエクスプレッションにするパネル。

var DEF = { start: '2020/01/01 00:00:00', end: '2020/01/01 00:00:00', form: 0, custom: 'yyyy/mm/dd hh:nn:ss', sk: 0, zfield: 0, cap: false, ordinal: false, gannen: false,
  prog: 0, progUnit: 0, speed: 0, speedUnit: 0, tran: 0, elapsed: false, pfield: 2, base: 0, top: 0 };
var FORMS = ['yyyy年mm月dd日(aaa) hh時nn分ss秒', 'gggee年mm月dd日(aaa) hh時nn分ss秒', 'yyyy/mm/dd (aaa) hh:nn:ss', 'mm/dd/yyyy (ddd) hh:nn:ss', 'ddddd, mmmmmm dd, yyyy', 'hh:nn:ss a.m./p.m.', '午前/午後 hh時nn分ss秒', 'kkkdd日', 'カスタム'];
var UNITS = ['秒', '分', '時', '日', '週', '月', '年'];

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }
function __parse(t) { var m = String(t).match(/\d+/g) || []; var v = [2020, 1, 1, 0, 0, 0]; for (var i = 0; i < 6 && i < m.length; i++) v[i] = Number(m[i]); return v; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: '日付と時刻' },
    { type: 'text', id: 'start', label: '開始日時（年/月/日 時:分:秒）', value: s.start },
    { type: 'choice', id: 'form', label: '表示形式', options: FORMS, value: s.form },
    { type: 'text', id: 'custom', label: 'カスタム形式', value: s.custom },
    { type: 'row', items: [{ type: 'choice', id: 'sk', label: '数字', options: ['半角', '1桁のとき全角', '全角'], value: s.sk }, { type: 'choice', id: 'zfield', label: 'フィールド文字', options: ['0', 'スペース'], value: s.zfield }] },
    { type: 'row', items: [{ type: 'checkbox', id: 'cap', label: '大文字', value: s.cap }, { type: 'checkbox', id: 'ordinal', label: '序数', value: s.ordinal }, { type: 'checkbox', id: 'gannen', label: '元年表記', value: s.gannen }] },
    { type: 'row', items: [{ type: 'number', id: 'prog', label: '加算時間', value: s.prog }, { type: 'choice', id: 'progUnit', label: '単位', options: UNITS, value: s.progUnit }] },
    { type: 'row', items: [{ type: 'number', id: 'speed', label: '速度（/再生秒）', value: s.speed }, { type: 'choice', id: 'speedUnit', label: '単位', options: UNITS, value: s.speedUnit }] },
    { type: 'row', items: [{ type: 'slider', id: 'tran', label: '推移', min: 0, max: 100, value: s.tran }, { type: 'text', id: 'end', label: '終了日時', value: s.end }] },
    { type: 'row', items: [{ type: 'checkbox', id: 'elapsed', label: '経過時間モード', value: s.elapsed }, { type: 'number', id: 'pfield', label: 'フィールド幅', value: s.pfield }] },
    { type: 'row', items: [{ type: 'choice', id: 'base', label: '基点', options: ['最初のフレーム', '最後のフレーム', '開始', '終了'], value: s.base }, { type: 'choice', id: 'top', label: '最上位の単位', options: ['時間', '分', '秒', '日'], value: s.top }] },
    { type: 'row', items: [{ type: 'button', id: 'apply', label: '付ける' }, { type: 'button', id: 'clear', label: '外す' }] }
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
    if (!l || l.kind.type !== 'text') return;
    n++;
    if (id === 'clear') { if (l.kind.text_expression && l.kind.text_expression.source.indexOf('dateTimeText(') >= 0) delete l.kind.text_expression; return; }
    var o = { start: __parse(s.start), end: __parse(s.end), form: s.form === 8 ? String(s.custom) : s.form + 1, sk: s.sk + 1, zfield: s.zfield + 1,
      cap: !!s.cap, ordinal: !!s.ordinal, gannen: !!s.gannen, prog: Number(s.prog), progUnit: s.progUnit + 1, speed: Number(s.speed), speedUnit: s.speedUnit + 1,
      tran: Number(s.tran), elapsed: !!s.elapsed, pfield: Number(s.pfield), base: s.base + 1, top: [1, 2, 3, 4][s.top] };
    l.kind.text_expression = { source: 'dateTimeText(' + JSON.stringify(o) + ')', enabled: true };
  });
  host.status(n ? n + ' 個のテキストレイヤーに' + (id === 'clear' ? 'から外しました' : '付けました') : 'テキストレイヤーを選択してください');
}
