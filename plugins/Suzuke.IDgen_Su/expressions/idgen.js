// IDgen_Su（Suzuke）の移植: UUID・NanoID・ULID などの「それらしい ID」をテキストの本文にする。
// テキストの本文のエクスプレッションから idgen(…) を呼ぶ。乱数はシードで決まる（本物の一意な ID ではない、映像用の飾り）。

var __IDA = {
  URL64: 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-',
  CROCKFORD32: '0123456789ABCDEFGHJKMNPQRSTVWXYZ',
  BASE36: '0123456789abcdefghijklmnopqrstuvwxyz',
  BASE62: '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'
};

function __idHash(a, b) {
  var h = Math.imul(a | 0, 0x9e3779b1) ^ Math.imul((b | 0) + 0x632be5ab, 0x85ebca6b);
  h = Math.imul(h ^ (h >>> 16), 0x7feb352d); h = Math.imul(h ^ (h >>> 15), 0x846ca68b); h ^= h >>> 16;
  return (h >>> 0) / 4294967296;
}
function __idEnc(num, a, len) {
  var r = '';
  for (var i = 0; i < len; i++) { r = a.charAt(num % a.length) + r; num = Math.floor(num / a.length); }
  return r;
}
function __idChars(rf, a, off, n) { var r = ''; for (var i = 1; i <= n; i++) r += a.charAt(Math.floor(rf(off + i) * a.length)); return r; }
function __idHex(b) { return (b < 16 ? '0' : '') + b.toString(16); }
function __idUuid(b, v) {
  b[6] = (b[6] & 0x0f) + v * 0x10; b[8] = (b[8] & 0x3f) + 0x80;
  var h = b.map(__idHex).join('');
  return h.substr(0, 8) + '-' + h.substr(8, 4) + '-' + h.substr(12, 4) + '-' + h.substr(16, 4) + '-' + h.substr(20, 12);
}

/**
 * type: 0 UUIDv4 / 1 NanoID / 2 ULID / 3 CUID / 4 UUIDv7 / 5 ShortID / 6 KSUID / 7 Snowflake
 * seed: シード、t: 時刻（秒）、perFrame: 毎フレーム変える、fps: フレームレート、key: レイヤーごとに変える値（0 で全レイヤー共通）
 */
function idgen(type, seed, t, perFrame, fps, key) {
  var frame = perFrame ? Math.floor(t * fps + 1e-6) : 1;
  var rf = function (off) { return __idHash((key | 0) * 0x10001 + seed * 256 + off, frame); };
  var rb = function (off) { return Math.floor(rf(off) * 256); };
  var ms = perFrame ? Math.floor(t * 1000) * 2 : 0;
  switch (type) {
    case 1: return __idChars(rf, __IDA.URL64, 100, 21);
    case 2: return __idEnc(ms % 281474976710656, __IDA.CROCKFORD32, 10) + __idChars(rf, __IDA.CROCKFORD32, 200, 16);
    case 3: return 'c' + __idEnc(ms, __IDA.BASE36, 8) + __idEnc(1, __IDA.BASE36, 4) + __idEnc(seed, __IDA.BASE36, 4) + __idChars(rf, __IDA.BASE36, 300, 8);
    case 4: {
      var ts = ms % 281474976710656, b = [];
      for (var i = 5; i >= 0; i--) { b[i] = ts % 256; ts = Math.floor(ts / 256); }
      for (var j = 6; j < 16; j++) b[j] = rb(1001 + j);
      return __idUuid(b, 7);
    }
    case 5: return __idChars(rf, __IDA.URL64, 500, 11);
    case 6: {
      var secs = Math.max(0, (perFrame ? Math.floor(t) * 2 : 0) - 1400000000) % 4294967296;
      var bytes = [];
      for (var k = 3; k >= 0; k--) { bytes[k] = secs % 256; secs = Math.floor(secs / 256); }
      for (var m = 1; m <= 16; m++) bytes[3 + m] = rb(6000 + m);
      var parts = [];
      for (var w = 0; w < 5; w++) parts.push(((bytes[w * 4] * 256 + bytes[w * 4 + 1]) * 256 + bytes[w * 4 + 2]) * 256 + bytes[w * 4 + 3]);
      var out = '';
      while (parts.length) {
        var q = [], rem = 0;
        parts.forEach(function (pv) { var v = pv + rem * 4294967296; var d = Math.floor(v / 62); rem = v % 62; if (d !== 0 || q.length) q.push(d); });
        out = __IDA.BASE62.charAt(rem) + out; parts = q;
      }
      while (out.length < 27) out = '0' + out;
      return out;
    }
    case 7: {
      var tsf = Math.max(0, ms - 1577836800000) % 4398046511104;
      return String(tsf * 1024 + Math.floor(rf(800) * 1024));
    }
    default: {
      var bb = [];
      for (var n = 1; n <= 16; n++) bb.push(rb(n));
      return __idUuid(bb, 4);
    }
  }
}
