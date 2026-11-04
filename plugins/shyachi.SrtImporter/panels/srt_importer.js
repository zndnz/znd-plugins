// SRT Importer（shyachi）の移植: 位置・サイズ・フォント・色を決めてから、SRT の字幕をテキストレイヤーとして読み込むパネル。

var DEF = { x: 0, y: 0, size: 40, font: '', color: [1, 1, 1, 1], outline: false, outlineColor: [0, 0, 0, 1], outlineWidth: 4 };

function defaults(s) { for (var k in DEF) if (s[k] == null) s[k] = JSON.parse(JSON.stringify(DEF[k])); return s; }

function panel(s) {
  defaults(s);
  return [
    { type: 'heading', text: 'SRT Importer' },
    { type: 'row', items: [{ type: 'number', id: 'x', label: 'X', value: s.x }, { type: 'number', id: 'y', label: 'Y', value: s.y }] },
    { type: 'number', id: 'size', label: 'サイズ', value: s.size },
    { type: 'text', id: 'font', label: 'フォント', value: s.font },
    { type: 'color', id: 'color', label: '文字色', value: s.color },
    { type: 'row', items: [{ type: 'checkbox', id: 'outline', label: '縁取りを有効', value: s.outline }, { type: 'color', id: 'outlineColor', label: '縁色', value: s.outlineColor }] },
    { type: 'number', id: 'outlineWidth', label: '縁の太さ', value: s.outlineWidth },
    { type: 'hint', text: '字幕は再生ヘッドの位置を 0 秒として並べます（時刻はフレームに切り捨て）。' },
    { type: 'button', id: 'import', label: 'Import SRT...' }
  ];
}

function onEvent(id, value, s) {
  defaults(s);
  if (id !== 'import') { s[id] = value; return; }
  host.openFile({ extensions: ['srt'], title: 'SRT ファイルを選択', data: { comp: ctx.comp, time: ctx.time || 0, s: s } }, onSrt);
}

function __ms(h, m, sec, ms) { return ((Number(h) * 60 + Number(m)) * 60 + Number(sec)) * 1000 + Number(ms); }

/** SRT の文字列を [{start, end, text}]（秒）にする。 */
function parseSrt(src) {
  var out = [];
  String(src).replace(/^﻿/, '').replace(/\r\n?/g, '\n').split(/\n\s*\n/).forEach(function (b) {
    var lines = b.split('\n');
    var t = -1;
    for (var i = 0; i < lines.length; i++) if (lines[i].indexOf('-->') >= 0) { t = i; break; }
    if (t < 0) return;
    var m = lines[t].match(/(\d+):(\d+):(\d+)[,.](\d+)\s*-->\s*(\d+):(\d+):(\d+)[,.](\d+)/);
    if (!m) return;
    var text = lines.slice(t + 1).join('\n').replace(/\n+$/, '');
    out.push({ start: __ms(m[1], m[2], m[3], m[4]) / 1000, end: __ms(m[5], m[6], m[7], m[8]) / 1000, text: text });
  });
  out.sort(function (a, b) { return a.start - b.start; });
  return out;
}

function onSrt(content, path, data) {
  var c = comp(data.comp), s = defaults(data.s || {});
  var subs = parseSrt(content);
  if (!subs.length) { host.alert('SRT の内容が空か、読み込みに失敗しました。'); return; }
  var fps = c.frame_rate.num / c.frame_rate.den;
  var fl = function (sec) { return flicks(Math.floor(sec * fps) / fps); };
  var base = flicks(data.time), n = 0;
  // addText は一番上に足すので、後ろの字幕から足して上から時刻順に並べる
  for (var i = subs.length - 1; i >= 0; i--) {
    var sub = subs[i], a = base + fl(sub.start), b = base + fl(sub.end);
    if (b <= a) continue;
    var l = addText(c, (sub.text.split('\n')[0] || '字幕').slice(0, 40), sub.text, s.size);
    l.in_point = a;
    l.out_point = b;
    var d = l.kind.document;
    if (s.font) d.font_family = s.font;
    d.fill = s.color;
    d.stroke = s.outline ? { color: s.outlineColor, width: Number(s.outlineWidth) } : null;
    var p = l.transform.position.value;
    l.transform.position.value = [p[0] + Number(s.x), p[1] + Number(s.y)];
    n++;
  }
  host.status(n + ' 個の字幕を読み込みました');
}
