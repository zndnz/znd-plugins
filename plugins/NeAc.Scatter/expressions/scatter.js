// Scatter（猫愛）の移植: オブジェクトごとに乱数で位置・回転・拡大をばらけさせる。
// すべてのエクスプレッションから呼べる。

/** Lua の math.randomseed(seed); math.random(min, max) に相当する、seed で決まる整数の乱数 */
function scatterRandom(min, max, seed) {
  if (min > max) { var t = min; min = max; max = t; }
  min = Math.floor(min); max = Math.floor(max);
  var h = (seed | 0) ^ 0x9e3779b9;
  h = Math.imul(h ^ (h >>> 16), 0x7feb352d); h = Math.imul(h ^ (h >>> 15), 0x846ca68b); h ^= h >>> 16;
  return min + ((h >>> 0) % (max - min + 1));
}

/**
 * index 番目（0 始まり）のオブジェクトのずれ。progress は % （100 で範囲いっぱい）。
 * r = { x: [min, max], y: [...], z, rx, ry, rz, zoom（% 。100 で等倍） }
 * 戻り値 { x, y, z, rx, ry, rz, zoom（倍率の差分） }
 */
function scatter(progress, seed, r, i) {
  var k = progress / 100;
  var g = function (name, n, d) { var v = r[name] || [d, d]; return k * scatterRandom(v[0], v[1], i + seed + n); };
  return { x: g('x', 0, 0), y: g('y', 1, 0), z: g('z', 2, 0), rx: g('rx', 3, 0), ry: g('ry', 4, 0), rz: g('rz', 5, 0),
    zoom: k * scatterRandom((r.zoom || [100, 100])[0] - 100, (r.zoom || [100, 100])[1] - 100, i + seed + 6) / 100 };
}
