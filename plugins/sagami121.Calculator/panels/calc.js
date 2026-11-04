// Calculator（sagami121）の移植: 四則演算の電卓パネル（計算の決まりは元と同じ）。

function defaults(s) {
  if (s.display == null) { s.display = '0'; s.equation = ''; s.last = 0; s.op = null; s.wait = false; }
  return s;
}
function num(s) { var v = parseFloat(s.display); return isNaN(v) ? 0 : v; }
function fmt(v) { return String(Math.round(v * 1e12) / 1e12); }

function digit(s, d) {
  if (s.display === '0' || s.wait) { s.display = d; s.wait = false; } else s.display += d;
}
function inner(s) {
  if (!s.op) return;
  var c = num(s), r = s.op === '+' ? s.last + c : s.op === '-' ? s.last - c : s.op === '×' ? s.last * c : s.op === '÷' ? (c !== 0 ? s.last / c : 0) : c;
  s.display = fmt(r);
  s.last = r;
  s.wait = true;
}
function setOp(s, op) {
  if (s.op && !s.wait) inner(s);
  s.last = num(s);
  s.op = op;
  s.equation = fmt(s.last) + ' ' + op;
  s.wait = true;
}
function equals(s) {
  if (!s.op) return;
  var prev = s.last, c = num(s), op = s.op;
  inner(s);
  s.equation = fmt(prev) + ' ' + op + ' ' + fmt(c) + ' =';
  s.op = null;
}

function panel(s) {
  defaults(s);
  var b = function (l) { return { type: 'button', id: 'k' + l, label: l }; };
  return [
    { type: 'hint', text: s.equation || ' ' },
    { type: 'heading', text: s.display },
    { type: 'row', items: [b('%'), b('CE'), b('C'), b('←')] },
    { type: 'row', items: [b('7'), b('8'), b('9'), b('÷')] },
    { type: 'row', items: [b('4'), b('5'), b('6'), b('×')] },
    { type: 'row', items: [b('1'), b('2'), b('3'), b('-')] },
    { type: 'row', items: [b('±'), b('0'), b('.'), b('+')] },
    { type: 'row', items: [b('='), { type: 'button', id: 'copy', label: '結果をコピー' }] }
  ];
}

function onEvent(id, value, s) {
  defaults(s);
  if (id === 'copy') { host.copyText(s.display); host.status('コピーしました: ' + s.display); return; }
  var k = id.substring(1);
  if (/^[0-9]$/.test(k)) digit(s, k);
  else if (k === '.') { if (s.wait) { s.display = '0.'; s.wait = false; } else if (s.display.indexOf('.') < 0) s.display += '.'; }
  else if (k === '+' || k === '-' || k === '×' || k === '÷') setOp(s, k);
  else if (k === '=') equals(s);
  else if (k === '%') s.display = fmt(num(s) / 100);
  else if (k === '±') s.display = fmt(-num(s));
  else if (k === 'CE') s.display = '0';
  else if (k === 'C') { s.display = '0'; s.equation = ''; s.last = 0; s.op = null; s.wait = false; }
  else if (k === '←') { if (!s.wait && s.display !== '0') { s.display = s.display.slice(0, -1); if (s.display === '' || s.display === '-') s.display = '0'; } }
}
