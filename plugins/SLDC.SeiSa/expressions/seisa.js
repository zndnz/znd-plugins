// あとのせRTAタイマー（総タイム）SeiSa（SLDC）の移植。
// テキストレイヤーのマーカーで区間を区切り、走行中の区間だけ時間を数えたタイムを本文にする。

/** 区間の開始フレーム（レイヤーのイン点からのフレーム番号）。先頭は 0 */
function __seisaSections(L, fps) {
  var starts = [0], m = L.marker;
  for (var i = 1; i <= m.numKeys; i++) {
    var f = Math.round((m.key(i).time - L.inPoint) * fps);
    if (f > starts[starts.length - 1]) starts.push(f);
  }
  return starts;
}

/** 区間ごとの状態: 1 走行前 / 2 走行中 / 3 中断中 / 4 走行後 */
function __seisaStates(n, odd) {
  var st = [];
  for (var i = 1; i <= n; i++) st.push((i % 2 === 1) === odd ? 2 : 3);
  if (st[n - 1] !== 2) st[n - 1] = 4;
  if (st[0] !== 2) st[0] = 1;
  return st;
}

function __seisaSectionNo(starts, frame) {
  var no = 0;
  while (no + 1 < starts.length && frame >= starts[no + 1]) no++;
  return no;
}

function __seisaElapsed(p, starts, states, frame) {
  var n = p.startFrames, sec = __seisaSectionNo(starts, frame);
  for (var s = 0; s < sec; s++) if (states[s] === 2) n += starts[s + 1] - starts[s];
  if (states[sec] === 2) n += frame - starts[sec];
  return n;
}

function __pad(v, n) { var s = String(v); while (s.length < n) s = '0' + s; return s; }

/** 本文のエクスプレッションから呼ぶ: seisaText(thisLayer, time, 設定) */
function seisaText(L, t, p) {
  var fps = p.fps;
  var starts = __seisaSections(L, fps);
  var states = __seisaStates(starts.length, p.odd);
  var total = Math.round((L.outPoint - L.inPoint) * fps);
  var frame = Math.floor((t - L.inPoint) * fps + 1e-6);
  var e = __seisaElapsed(p, starts, states, frame);
  var max = __seisaElapsed(p, starts, states, total);
  var sep = p.format === 1 ? ['h', 'm', 's', 'ms'] : [':', ':', '.', ''];
  var ms = Math.floor((e % fps) / fps * 1000 + 0.5), s = Math.floor(e / fps) % 60, m = Math.floor(e / (fps * 60)) % 60, h = Math.floor(e / (fps * 3600));
  var str = __pad(h, 4) + sep[0] + __pad(m, 2) + sep[1] + __pad(s, 2) + sep[2] + __pad(ms, 3) + sep[3];
  // 桁数: 全体の最大のタイムに合わせて先頭を削る（「自動調整」）。timedigit で最低限残す桁を決める
  var cuts = [[36000000 * fps, 1], [360000 * fps, 1], [36000 * fps, 1], [3600 * fps, 1 + sep[0].length], [600 * fps, 1], [60 * fps, 1 + sep[1].length], [10 * fps, 1]];
  var cut = 0;
  for (var i = 0; i < cuts.length; i++) {
    if (p.digit === cuts.length - i) break;
    if (max < cuts[i][0]) cut += cuts[i][1]; else break;
  }
  str = str.substring(cut);
  if (p.frames !== 0) {
    var digits = Math.max(String(Math.max(1, max)).length, p.frameDigit);
    str += (p.frames === 2 ? '\n' : ' ') + '(' + __pad(e, digits) + ' / ' + fps + ')';
  }
  return str;
}

/** 文字色のアニメーター用: 今の区間の状態が state（1〜4）なら 100 */
function seisaStateIs(L, t, p, state) {
  var fps = p.fps, starts = __seisaSections(L, fps), states = __seisaStates(starts.length, p.odd);
  return states[__seisaSectionNo(starts, Math.floor((t - L.inPoint) * fps + 1e-6))] === state ? 100 : 0;
}
