// AutoLyricAnimation_K（korarei）の移植: 文字ごとにずれて、ランダムな方向から入って・出ていく歌詞アニメーション。
// テキストのアニメーターのエクスプレッションセレクターから呼ばれる（textIndex は 1 始まり）。

function __alaHash(a, b, c) {
  var h = Math.imul(a | 0, 0x9e3779b1) ^ Math.imul((b | 0) + 0x632be5ab, 0x85ebca6b) ^ Math.imul((c | 0) + 0x5bd1e995, 0xc2b2ae35);
  h = Math.imul(h ^ (h >>> 16), 0x7feb352d); h = Math.imul(h ^ (h >>> 15), 0x846ca68b); h ^= h >>> 16;
  return h >>> 0;
}
/** obj.rand(st, ed, seed, frame) に相当: st 以上 ed 以下の整数 */
function __alaRand(st, ed, seed, k) { return st + __alaHash(seed, k, 17) % (ed - st + 1); }

/** イージング（0..1）。0: 直線, 1: 二次, 2: 三次, 3: 指数, 4: バック */
function __alaEase(kind, t) {
  t = Math.max(0, Math.min(1, t));
  switch (kind) {
    case 1: return 1 - (1 - t) * (1 - t);
    case 2: return 1 - Math.pow(1 - t, 3);
    case 3: return t >= 1 ? 1 : 1 - Math.pow(2, -10 * t);
    case 4: { var c1 = 1.70158, c3 = c1 + 1; return 1 + c3 * Math.pow(t - 1, 3) + c1 * Math.pow(t - 1, 2); }
    default: return t;
  }
}

/**
 * 文字 textIndex の、アニメーターの部位 part に対する量（%）。
 * part: 'px' 'nx' 'py' 'ny'（その向きへのずれ）/ 'hide'（非表示）/ 'entry'・'exit'（出入りの瞬間の拡大）/ 'flip'（左右反転）
 */
function alaAmount(p, part, textIndex, textTotal, time) {
  var i = textIndex - 1, num = Math.max(1, textTotal);
  var frame = Math.floor((time - p.inPoint) * p.fps + 1e-6);
  var totalframe = Math.round((p.outPoint - p.inPoint) * p.fps);
  var DELTA = 100;
  var r = function (st, ed, slot) { return __alaRand(st, ed, p.seed, i + slot * num); };
  var delta = 0, hide = false, entry = false, exit = false;
  var spread = r(100 - p.spread, 100, 1) / 100;
  var progIn = p.speed * frame - i * p.interval;
  var inPart = function () {
    if (progIn < 0) hide = true; else if (progIn < p.speed) entry = true;
    return (1 - __alaEase(p.ease, Math.max(0, Math.min(DELTA, progIn)) / DELTA)) * p.maxDist * spread;
  };
  var outPart = function (invMotion) {
    var st = p.speed * totalframe - (DELTA + p.interval * (num - 1)) - 1;
    var prog = progIn - st;
    if (prog > DELTA) hide = true; else if (prog > DELTA - p.speed) exit = true;
    var t = Math.max(0, Math.min(DELTA, prog)) / DELTA;
    return (invMotion ? 1 - __alaEase(p.ease, 1 - t) : __alaEase(p.ease, t)) * p.maxDist * spread;
  };
  if (p.type === 0) delta = inPart();
  else if (p.type === 1) delta = outPart(false);
  else delta = inPart() + outPart(true);
  if (p.omni && r(0, 1, 2) === 0) delta = -delta;
  if (p.inv) delta = -delta;
  var axis = p.moveX && p.moveY ? (r(0, 1, 3) === 0 ? 'x' : 'y') : p.moveX ? 'x' : p.moveY ? 'y' : '';
  var flip = (entry && p.entryFlip || exit && p.exitFlip) && r(0, 1, 4) === 0;
  var amt = p.maxDist > 0 ? Math.abs(delta) / p.maxDist * 100 : 0;
  switch (part) {
    case 'px': return axis === 'x' && delta > 0 ? amt : 0;
    case 'nx': return axis === 'x' && delta < 0 ? amt : 0;
    case 'py': return axis === 'y' && delta > 0 ? amt : 0;
    case 'ny': return axis === 'y' && delta < 0 ? amt : 0;
    case 'hide': return hide ? 100 : 0;
    case 'entry': return entry ? 100 : 0;
    case 'exit': return exit ? 100 : 0;
    case 'flip': return flip ? 100 : 0;
  }
  return 0;
}
