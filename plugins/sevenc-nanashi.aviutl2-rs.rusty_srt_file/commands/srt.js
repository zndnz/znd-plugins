// Rusty SRT File Plugin（Nanashi.）の移植: SRT（字幕）ファイルの読み込みと書き出し。
// 読み込み: 選んだテキストレイヤーをひな形にして、字幕 1 つにつき 1 つのテキストレイヤーを作る（ひな形は消す）。
// 書き出し: 選んだテキストレイヤー（選んでいなければコンポのすべてのテキストレイヤー）を時刻順に SRT にする。

function __ms(h, m, s, ms) { return ((Number(h) * 60 + Number(m)) * 60 + Number(s)) * 1000 + Number(ms); }

/** SRT の文字列を [{start, end, text}]（秒）にする。 */
function parseSrt(src) {
  var out = [];
  var blocks = String(src).replace(/^﻿/, '').replace(/\r\n?/g, '\n').split(/\n\s*\n/);
  blocks.forEach(function (b) {
    var lines = b.split('\n').filter(function (l, i, a) { return !(i === a.length - 1 && l === ''); });
    var t = -1;
    for (var i = 0; i < lines.length; i++) if (lines[i].indexOf('-->') >= 0) { t = i; break; }
    if (t < 0) return;
    var m = lines[t].match(/(\d+):(\d+):(\d+)[,.](\d+)\s*-->\s*(\d+):(\d+):(\d+)[,.](\d+)/);
    if (!m) return;
    out.push({ start: __ms(m[1], m[2], m[3], m[4]) / 1000, end: __ms(m[5], m[6], m[7], m[8]) / 1000, text: lines.slice(t + 1).join('\n') });
  });
  out.sort(function (a, b) { return a.start - b.start || a.end - b.end; });
  return out;
}

function __ts(sec) {
  var ms = Math.max(0, Math.round(sec * 1000));
  function p(n, w) { n = String(n); while (n.length < w) n = '0' + n; return n; }
  return p(Math.floor(ms / 3600000), 2) + ':' + p(Math.floor(ms / 60000) % 60, 2) + ':' + p(Math.floor(ms / 1000) % 60, 2) + ',' + p(ms % 1000, 3);
}

/** [{start, end, text}] を SRT の文字列にする。 */
function formatSrt(subs) {
  return subs.map(function (s, i) { return (i + 1) + '\n' + __ts(s.start) + ' --> ' + __ts(s.end) + '\n' + s.text + '\n'; }).join('\n');
}

function importSrt() {
  var c = comp(ctx.comp);
  var l = ctx.layer ? layer(c, ctx.layer) : null;
  if (!l || l.kind.type !== 'text') { host.alert('ひな形にするテキストレイヤーを 1 つ選んでください。'); return; }
  host.openFile({ extensions: ['srt'], title: 'SRT ファイルを選択', data: { comp: c.id, layer: l.id } }, onSrt);
}

function onSrt(content, path, data) {
  var c = comp(data.comp), tpl = layer(c, data.layer);
  if (!tpl) return;
  var subs = parseSrt(content);
  if (!subs.length) { host.alert('SRT ファイルに字幕が含まれていません。'); return; }
  var fps = c.frame_rate ? c.frame_rate.num / c.frame_rate.den : (ctx.fps || 30);
  var snap = function (sec) { return flicks(Math.round(sec * fps) / fps); };
  var base = tpl.in_point, idx = c.layers.indexOf(tpl), made = [], next = base;
  subs.forEach(function (s, k) {
    var a = base + snap(s.start), b = base + snap(s.end);
    if (a < next) a = next;
    if (b <= a) return;
    var n = JSON.parse(JSON.stringify(tpl));
    n.id = newId('srt');
    n.name = (s.text.split('\n')[0] || ('字幕 ' + (k + 1))).slice(0, 40);
    n.in_point = a;
    n.out_point = b;
    n.kind.document.text = s.text;
    c.layers.splice(idx + made.length + 1, 0, n);
    made.push(n.id);
    next = b;
  });
  c.layers.splice(idx, 1);
  host.status(made.length + ' 個の字幕を読み込みました');
}

function exportSrt() {
  var c = comp(ctx.comp);
  var ids = ctx.layers && ctx.layers.length ? ctx.layers : null;
  var ls = c.layers.filter(function (l) { return l.kind.type === 'text' && (!ids || ids.indexOf(l.id) >= 0); });
  if (!ls.length) { host.alert('テキストレイヤーがありません。'); return; }
  var subs = ls.map(function (l) { return { start: seconds(l.in_point), end: seconds(l.out_point), text: l.kind.document.text }; });
  subs.sort(function (a, b) { return a.start - b.start || a.end - b.end; });
  host.saveFile((c.name || 'subtitles') + '.srt', formatSrt(subs), { extensions: ['srt'] });
}
