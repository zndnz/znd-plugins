// import_midi_tempos.aux2（Nanashi.）の移植: MIDI ファイルのテンポ・拍子・マーカーを、コンポのビートグリッドとマーカーにする。
// znd のビートグリッドは 1 つの BPM なので、最初のテンポと拍子を使い、テンポが変わるところにはマーカーを置く。

/** SMF を読み、{ events: [{ tick, type, value }], tpq } を返す。 */
function parseSmf(b) {
  var p = 0;
  function u32() { var v = ((b[p] << 24) >>> 0) + (b[p + 1] << 16) + (b[p + 2] << 8) + b[p + 3]; p += 4; return v; }
  function u16() { var v = (b[p] << 8) + b[p + 1]; p += 2; return v; }
  function vlq() { var v = 0, c; do { c = b[p++]; v = v * 128 + (c & 0x7f); } while (c & 0x80 && p < b.length); return v; }
  function str(n) { var s = ''; for (var i = 0; i < n; i++) s += String.fromCharCode(b[p + i]); return s; }
  if (str(4) !== 'MThd') throw new Error('MIDI ファイルではありません');
  p += 4;
  var hlen = u32(), format = u16(), ntr = u16(), div = u16();
  p = 8 + hlen;
  if (format === 2) throw new Error('フォーマット 2 の MIDI には対応していません');
  if (div & 0x8000) throw new Error('SMPTE 時間の MIDI には対応していません');
  var events = [];
  for (var t = 0; t < ntr && p < b.length; t++) {
    if (str(4) !== 'MTrk') break;
    p += 4;
    var len = u32(), end = p + len, tick = 0, run = 0;
    while (p < end) {
      tick += vlq();
      var st = b[p];
      if (st === 0xff) {
        p++;
        var type = b[p++], n = vlq(), d = b.slice(p, p + n);
        if (type === 0x51 && n === 3) events.push({ tick: tick, type: 'tempo', value: (d[0] << 16) + (d[1] << 8) + d[2] });
        else if (type === 0x58 && n >= 1) events.push({ tick: tick, type: 'sig', value: d[0] });
        else if (type === 0x06) events.push({ tick: tick, type: 'marker', value: __utf8(d) });
        p += n;
      } else if (st === 0xf0 || st === 0xf7) {
        p++;
        p += vlq();
      } else {
        if (st & 0x80) { run = st; p++; }
        var hi = run & 0xf0;
        p += (hi === 0xc0 || hi === 0xd0) ? 1 : 2;
      }
    }
    p = end;
  }
  events.sort(function (a, c) { return a.tick - c.tick; });
  return { events: events, tpq: div };
}

function __utf8(d) {
  var s = '';
  for (var i = 0; i < d.length; i++) s += '%' + ('0' + d[i].toString(16)).slice(-2);
  try { return decodeURIComponent(s); } catch (e) { return String.fromCharCode.apply(null, d); }
}

/** テンポを考えて tick を秒にする関数を作る。 */
function tickToSeconds(events, tpq) {
  var tempos = events.filter(function (e) { return e.type === 'tempo'; });
  return function (tick) {
    var sec = 0, last = 0, us = 500000;
    for (var i = 0; i < tempos.length && tempos[i].tick <= tick; i++) {
      sec += (tempos[i].tick - last) / tpq * us / 1e6;
      last = tempos[i].tick;
      us = tempos[i].value;
    }
    return sec + (tick - last) / tpq * us / 1e6;
  };
}

function importMidi() {
  host.openFile({ extensions: ['mid', 'midi'], binary: true, title: 'MIDI ファイルを選択', data: { comp: ctx.comp } }, onMidi);
}

function onMidi(bytes, path, data) {
  var c = comp(data.comp);
  var smf;
  try { smf = parseSmf(bytes); } catch (e) { host.alert(String(e.message || e)); return; }
  var toSec = tickToSeconds(smf.events, smf.tpq);
  var tempos = smf.events.filter(function (e) { return e.type === 'tempo'; });
  var sig = smf.events.filter(function (e) { return e.type === 'sig'; })[0];
  var bpm = tempos.length ? 60e6 / tempos[0].value : 120;
  c.beat_grid = { bpm: Math.round(bpm * 1000) / 1000, beats_per_bar: sig ? sig.value : 4, subdivision: (c.beat_grid && c.beat_grid.subdivision) || 1, offset: 0 };
  var marks = [];
  smf.events.forEach(function (e) {
    if (e.type === 'marker') marks.push({ time: flicks(toSec(e.tick)), duration: 0, comment: e.value });
  });
  tempos.slice(1).forEach(function (e) { marks.push({ time: flicks(toSec(e.tick)), duration: 0, comment: 'BPM ' + Math.round(60e6 / e.value * 100) / 100 }); });
  if (marks.length) {
    c.markers = (c.markers || []).filter(function (m) { return false; }).concat(marks);
    c.markers.sort(function (a, b) { return a.time - b.time; });
  }
  host.status('BPM ' + c.beat_grid.bpm + ' とマーカー ' + marks.length + ' 個を読み込みました');
}

function resetGrid() {
  var c = comp(ctx.comp);
  c.beat_grid = { bpm: 120, beats_per_bar: 4, subdivision: 1, offset: 0 };
  host.status('BPM グリッドを 120 に戻しました');
}
