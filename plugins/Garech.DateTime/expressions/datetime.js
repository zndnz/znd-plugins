// 日付と時刻.obj2（Garech、元は連結歯車氏）の移植: 時間とともに変わる日付と時刻の文字を作る。
// 表示形式の記号（yyyy・mm・dd・aaa・ggg・ee・kk・hh・nn・ss・午前/午後 など）は元と同じ。

var __DT = {
  forms: ['yyyy年mm月dd日(aaa) hh時nn分ss秒', 'gggee年mm月dd日(aaa) hh時nn分ss秒', 'yyyy/mm/dd (aaa) hh:nn:ss', 'mm/dd/yyyy (ddd) hh:nn:ss',
    'ddddd, mmmmmm dd, yyyy', 'hh:nn:ss a.m./p.m.', '午前/午後 hh時nn分ss秒', 'kkkdd日'],
  scale: [1, 60, 3600, 86400, 604800, 2629743.84, 31556926.08],
  epoch: 2440588
};

function __dtJdn(y, m, d) {
  var a = Math.floor((14 - m) / 12), yy = y + 4800 - a, mm = m + 12 * a - 3;
  return d + Math.floor((153 * mm + 2) / 5) + 365 * yy + Math.floor(yy / 4) - Math.floor(yy / 100) + Math.floor(yy / 400) - 32045;
}
function __dtFromJdn(j) {
  var a = j + 32044, b = Math.floor((4 * a + 3) / 146097), c = a - Math.floor(146097 * b / 4);
  var d = Math.floor((4 * c + 3) / 1461), e = c - Math.floor(1461 * d / 4), m = Math.floor((5 * e + 2) / 153);
  return [100 * b + d - 4800 + Math.floor(m / 10), m + 3 - 12 * Math.floor(m / 10), e - Math.floor((153 * m + 2) / 5) + 1];
}
function __dtSec(t) { return (__dtJdn(t[0], t[1], t[2]) - __DT.epoch) * 86400 + t[3] * 3600 + t[4] * 60 + t[5]; }
function __dtDate(sec) {
  var days = Math.floor(sec / 86400), s = sec - days * 86400;
  var ymd = __dtFromJdn(days + __DT.epoch);
  return { year: ymd[0], month: ymd[1], day: ymd[2], hour: Math.floor(s / 3600), min: Math.floor((s % 3600) / 60), sec: Math.floor(s % 60), wday: ((days + 4) % 7 + 7) % 7 + 1 };
}
function __dtGengo(d) {
  var map = [[5, 2019, 5, 1], [4, 1989, 1, 8], [3, 1926, 12, 25], [2, 1912, 7, 30], [1, 1868, 1, 25]];
  for (var i = 0; i < map.length; i++) {
    var v = map[i];
    if (d.year > v[1] || (d.year === v[1] && d.month > v[2]) || (d.year === v[1] && d.month === v[2] && d.day >= v[3])) return [v[0], d.year - v[1] + 1];
  }
  return [0, d.year];
}
function __dtZen(s) { return String(s).replace(/[0-9]/g, function (c) { return String.fromCharCode(c.charCodeAt(0) + 0xfee0); }).replace(/-/g, '－').replace(/ /g, '　'); }
function __dtPad(v, w, space) { var s = String(Math.abs(v)); while (s.length < w) s = (space ? ' ' : '0') + s; return (v < 0 ? '-' : '') + s; }

/**
 * 日付と時刻の文字。o の項目（省略可）:
 * start [年,月,日,時,分,秒]・end [...]・form（1〜8 か書式の文字列）・sk（1 半角 / 2 1桁のとき全角 / 3 全角）・
 * zfield（1 0 / 2 スペース）・cap・ordinal・gannen・prog・progUnit（1 秒〜7 年）・speed・speedUnit・tran（0〜100）・
 * elapsed（経過時間モード）・pfield・base（1 最初 / 2 最後 / 3 開始 / 4 終了）・top（3 秒 / 2 分 / 1 時間 / 4 日）
 */
function dateTimeText(o) {
  o = o || {};
  var st = o.start || [2020, 1, 1, 0, 0, 0], en = o.end || st;
  var form = typeof o.form === 'string' ? o.form : (__DT.forms[(o.form || 1) - 1] || __DT.forms[0]);
  var sk = o.sk || 1, space = o.zfield === 2;
  var lt = time - thisLayer.inPoint, dur = thisLayer.outPoint - thisLayer.inPoint;
  var tran = (o.tran || 0) / 100, ta = __dtSec(st), tb = __dtSec(en);
  var at = function (t) {
    var s = ta * (1 - tran) + tb * tran;
    s += (o.prog || 0) * __DT.scale[(o.progUnit || 1) - 1];
    s += (o.speed || 0) * t * __DT.scale[(o.speedUnit || 1) - 1];
    return s;
  };
  var now = at(lt), d, top = null, neg = false;
  if (o.elapsed) {
    var base = o.base || 1;
    var tp = base === 3 ? ta : base === 4 ? tb : base === 2 ? at(dur) : at(0);
    var diff = now - tp;
    neg = diff < 0;
    var a = Math.abs(diff), days = Math.floor(a / 86400), t = a % 86400;
    d = { year: 0, month: 0, day: days, hour: Math.floor(t / 3600), min: Math.floor((t % 3600) / 60), sec: Math.floor(t % 60), wday: 8 };
    var u = o.top || 1;
    if (u === 1) { d.hour += d.day * 24; d.day = 0; top = 'h'; }
    else if (u === 2) { d.min += (d.hour + d.day * 24) * 60; d.day = d.hour = 0; top = 'n'; }
    else if (u === 3) { d.sec += (d.min + (d.hour + d.day * 24) * 60) * 60; d.day = d.hour = d.min = 0; top = 's'; }
    else top = 'd';
  } else d = __dtDate(Math.floor(now));
  var caps = o.cap ? function (s) { return s.toUpperCase(); } : function (s) { return s; };
  var ord = '';
  if (o.ordinal) ord = caps([1, 21, 31].indexOf(d.day) >= 0 ? 'st' : [2, 22].indexOf(d.day) >= 0 ? 'nd' : [3, 23].indexOf(d.day) >= 0 ? 'rd' : 'th');
  form = form.replace(/\\n/g, '\n').replace(/\\t/g, '\t');
  var ampm = ['午前/午後', 'AM/PM', 'A.M./P.M.', 'am/pm', 'a.m./p.m.', 'A/P', 'A./P.', 'a/p', 'a./p.'], am = '', pm = '';
  for (var i = 0; i < ampm.length; i++) if (form.indexOf(ampm[i]) >= 0) { am = ampm[i].split('/')[0]; pm = ampm[i].split('/')[1]; form = form.split(ampm[i]).join('||||'); break; }
  var hd = d.hour, ap = '';
  if (!o.elapsed && am) {
    if (d.hour <= 11) ap = am; else { ap = pm; hd = d.hour - 12; }
    if (am !== '午前' && (d.hour === 0 || d.hour === 12)) hd = 12;
  }
  var num = function (v, w, letter) {
    var isTop = o.elapsed && letter === top;
    if (isTop) w = Math.max(w, o.pfield || 2);
    var s = __dtPad(v, w, space);
    if (isTop && neg) s = '-' + s;
    if (sk === 3 || (sk === 2 && Math.abs(v) < 10 && w <= 1)) return __dtZen(s);
    if (sk === 2 && Math.abs(v) < 10) return __dtZen(String(v));
    return s;
  };
  var M = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];
  var D = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
  var K = ['睦月', '如月', '弥生', '卯月', '皐月', '水無月', '文月', '葉月', '長月', '神無月', '霜月', '師走'];
  var fmt = function (f) {
    var c = f.charAt(0), n = f.length, g;
    switch (c) {
      case 'y': { var y = d.year; if (y <= 0 && !o.elapsed) y = -y + 1; var ys = String(y); while (ys.length < n) ys = (space ? ' ' : '0') + ys; ys = ys.slice(-n); return sk === 3 ? __dtZen(ys) : ys; }
      case 'g': g = __dtGengo(d)[0]; return g ? [['M', 'T', 'S', 'H', 'R'], ['明', '大', '昭', '平', '令'], ['明治', '大正', '昭和', '平成', '令和']][Math.min(n, 3) - 1][g - 1] : 'N/A';
      case 'e': g = __dtGengo(d)[1]; return o.gannen && g === 1 ? '元' : num(g, n === 1 ? 1 : 2, 'e');
      case 'm':
        if (n <= 2) return num(d.month, n, 'm');
        if (n === 3) return caps((M[d.month - 1] || 'N/A').slice(0, 3));
        if (n === 4) return caps(M[d.month - 1] || 'N/A');
        if (n === 5) return (M[d.month - 1] || 'N/A').charAt(0);
        var mm = caps(M[d.month - 1] || 'N/A'); while (mm.length < 9) mm = ' ' + mm; return mm;
      case 'k': { var k = K[d.month - 1]; if (!k) return 'N/A'; if (n === 1) return k.charAt(0); if (n === 2) return k; return k.length < 3 ? '　' + k : k; }
      case 'd':
        if (n <= 2) return num(d.day, n, 'd') + ord;
        var dn = D[d.wday - 1] || 'N/A';
        if (n === 3) return caps(dn.slice(0, 3));
        if (n === 4) return caps(dn);
        var dd = caps(dn); while (dd.length < 9) dd = ' ' + dd; return dd;
      case 'a': { var j = ['日', '月', '火', '水', '木', '金', '土'][d.wday - 1]; if (!j) return 'N/A'; return n >= 4 ? j + '曜日' : j; }
      case 'h': return num(hd, n, 'h');
      case 'n': return num(d.min, n, 'n');
      case 's': return num(d.sec, n, 's');
      case '|': return ap || 'N/A';
    }
    return f;
  };
  var max = { y: 4, g: 3, e: 2, m: 6, k: 3, d: 5, a: 4, h: 2, n: 2, s: 2, '|': 4 };
  var chars = Array.from(form), out = '', i2 = 0;
  while (i2 < chars.length) {
    var ch = chars[i2];
    if (ch === '$') { out += chars[i2 + 1] || ''; i2 += 2; continue; }
    if (max[ch]) {
      var len = 1;
      while (len < max[ch] && chars[i2 + len] === ch) len++;
      out += fmt(new Array(len + 1).join(ch));
      i2 += len;
      continue;
    }
    out += ch;
    i2++;
  }
  return out;
}
