// time_control_assistant.tra2（sevenc-nanashi）の移植: 再生位置（タイムリマップ）を、範囲指定・繰り返し・往復で作る。
// レイヤーのタイムリマップのエクスプレッションから tcaTime(p, time) を呼ぶ。戻り値はソースの秒。

/**
 * p = { mode: 'absolute' | 'relative' | 'repeat' | 'forthback' | 'reverse',
 *       period（秒、繰り返しの周期）, offset（秒、周期のずれ）,
 *       a, b（absolute: ソースの開始・終了秒、relative: イン点・アウト点からのずれの秒）,
 *       inPoint, outPoint, base（イン点でのソースの秒）, grid: { bpm, beats, offset } | null（BPM グリッドに合わせるとき） }
 */
function tcaTime(p, time) {
  var t = time - p.inPoint, dur = Math.max(1e-6, p.outPoint - p.inPoint);
  if (p.mode === 'absolute') return p.a + (p.b - p.a) * t / dur;
  if (p.mode === 'relative') return (p.base + p.a) + ((p.base + dur + p.b) - (p.base + p.a)) * t / dur;
  var period = Math.max(1e-6, p.period), offset = p.offset;
  var pos;
  if (p.grid) {
    // BPM グリッド: 小節の中の位置で回す（コンポ時刻で数える）
    var measure = 60 * p.grid.beats / p.grid.bpm;
    var mpos = (time - p.grid.offset) / measure;
    var div = measure / p.grid.divisions;
    period = div * p.grid.units;
    pos = (mpos * p.grid.divisions - p.grid.divOffset) / p.grid.units;
    if (p.grid.fromMeasure) {
      var startM = (p.inPoint - p.grid.offset) / measure;
      pos -= (Math.ceil(startM - 1e-9) - startM) * p.grid.divisions / p.grid.units;
    }
  } else {
    pos = (t - offset) / period;
  }
  var u;
  if (p.mode === 'repeat') u = pos - Math.floor(pos);
  else {
    var q = pos - Math.floor(pos / 2) * 2;
    u = q > 1 ? 2 - q : q;
    if (p.mode === 'reverse') u = 1 - u;
  }
  return p.base + u * period;
}
