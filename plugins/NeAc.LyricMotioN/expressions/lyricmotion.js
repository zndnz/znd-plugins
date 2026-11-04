// LyricMotioN（猫愛）の移植: 文字ごとに順番にずれて、上下左右のどれかの方向から入る（出る）歌詞モーション。
// テキストのアニメーターのエクスプレッションセレクターから呼ばれる（textIndex は 1 始まり）。

function __lmHash(a, b) {
  var h = Math.imul(a | 0, 0x9e3779b1) ^ Math.imul((b | 0) + 0x632be5ab, 0x85ebca6b);
  h = Math.imul(h ^ (h >>> 16), 0x7feb352d); h = Math.imul(h ^ (h >>> 15), 0x846ca68b); h ^= h >>> 16;
  return h >>> 0;
}

function __lmEase(kind, t) {
  t = Math.max(0, Math.min(1, t));
  switch (kind) {
    case 1: return 1 - (1 - t) * (1 - t);
    case 2: return 1 - Math.pow(1 - t, 3);
    case 3: return t >= 1 ? 1 : 1 - Math.pow(2, -10 * t);
    case 4: { var c1 = 1.70158, c3 = c1 + 1; return 1 + c3 * Math.pow(t - 1, 3) + c1 * Math.pow(t - 1, 2); }
    default: return t;
  }
}

/** i 番目（0 始まり）の文字の向き（'x' '-x' 'y' '-y'）。連続して同じ向きにならないようにできる */
function __lmDirection(p, i) {
  var dirs = p.dirs;
  if (!dirs.length) return '';
  var prev = null, cur = null;
  for (var k = 0; k <= i; k++) {
    var r = __lmHash(p.seed, k) % dirs.length;
    cur = dirs[r];
    if (k > 0 && cur === prev && !p.allowSame) cur = dirs[(r + 1) % dirs.length];
    prev = cur;
  }
  return cur;
}

/** 順序の番号 */
function __lmOrder(p, i, num) {
  switch (p.order) {
    case 1: return num - 1 - i;
    case 2: return Math.abs(num - i * 2 - 1);
    case 3: return num - Math.abs(num - i * 2 - 1);
    case 4: {
      // シードで決まる並べ替え: 自分のハッシュより小さい文字の数
      var mine = __lmHash(p.seedOrder, i), rank = 0;
      for (var k = 0; k < num; k++) { var h = __lmHash(p.seedOrder, k); if (h < mine || (h === mine && k < i)) rank++; }
      return rank;
    }
    default: return i;
  }
}

/** 文字の動き { vin, vout, dir }（vin・vout は 0..1） */
function __lmMotion(p, textIndex, textTotal, time) {
  var i = textIndex - 1, num = Math.max(1, textTotal);
  var frame = Math.floor((time - p.inPoint) * p.fps + 1e-6);
  var total = Math.max(2, Math.round((p.outPoint - p.inPoint) * p.fps));
  var anim = frame / (total - 1);
  var n = num - 1, idx = __lmOrder(p, i, num);
  var seq = Math.max(0, Math.min(1, anim * (1 + p.delay * n) - p.delay * idx));
  var m = __lmEase(p.ease, seq);
  return { vin: p.type === 0 ? 1 - m : 0, vout: p.type === 1 ? m : 0, dir: __lmDirection(p, i) };
}

/** アニメーターの部位 part（'x' '-x' 'y' '-y' 'zoom' 'alpha'）に対する量（%） */
function lyricMotionAmount(p, part, textIndex, textTotal, time) {
  var m = __lmMotion(p, textIndex, textTotal, time);
  var va = 1 - (1 - m.vin) * (1 - m.vout);
  if (part === 'zoom') return (va * m.vin + m.vout) * 100;
  if (part === 'alpha') {
    if (p.fade) return Math.abs(va) * 100;
    return p.hide && Math.ceil(Math.abs(va) * 1000) / 1000 >= 1 ? 100 : 0;
  }
  var dir = m.dir;
  if (p.vector < 0) dir = dir.charAt(0) === '-' ? dir.substring(1) : '-' + dir;
  return dir === part ? (m.vin + m.vout) * 100 : 0;
}
