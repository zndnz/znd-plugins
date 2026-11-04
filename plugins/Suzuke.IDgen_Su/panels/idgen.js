// IDgen_Su のパネル: 選んだテキストレイヤーの本文を ID にする。

var DEF = { type: 0, seed: 0, perFrame: false, perObject: false };

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = DEF[k]; return s; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: 'IDgen_Su（ID を表示）' },
    { type: 'hint', text: 'テキストレイヤーを選んで「ID にする」。本文がエクスプレッションで ID に置き換わります（フォント・大きさ・色はテキストレイヤーの設定）。' },
    { type: 'choice', id: 'type', label: 'ID 種別', options: ['UUIDv4', 'NanoID', 'ULID', 'CUID', 'UUIDv7', 'ShortID', 'KSUID', 'Snowflake'], value: s.type },
    { type: 'number', id: 'seed', label: 'シード', value: s.seed, speed: 1 },
    { type: 'checkbox', id: 'perFrame', label: '毎フレーム更新', value: s.perFrame },
    { type: 'checkbox', id: 'perObject', label: 'レイヤーごとに変える', value: s.perObject },
    { type: 'row', items: [{ type: 'button', id: 'apply', label: 'ID にする' }, { type: 'button', id: 'clear', label: 'やめる' }] }
  ];
}

function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'apply' && id !== 'clear') { s[id] = value; return; }
  var c = comp(ctx.comp), n = 0;
  var ids = ctx.layers.length ? ctx.layers : (ctx.layer ? [ctx.layer] : []);
  ids.forEach(function (lid, i) {
    var l = layer(c, lid);
    if (l.kind.type !== 'text') return;
    n++;
    if (id === 'clear') { delete l.kind.text_expression; return; }
    var key = s.perObject ? (i + 1) : 0;
    l.kind.text_expression = { source: 'idgen(' + s.type + ', ' + Math.floor(s.seed) + ', time, ' + !!s.perFrame + ', ' + ctx.fps + ', ' + key + ')', enabled: true };
  });
  host.status(n ? n + ' 個のテキストレイヤーを' + (id === 'clear' ? '元に戻しました' : 'ID にしました') : 'テキストレイヤーを選択してください');
}
