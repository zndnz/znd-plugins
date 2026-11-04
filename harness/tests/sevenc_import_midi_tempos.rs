mod common;
use common::*;

fn vlq(mut v: u32) -> Vec<u8> {
    let mut out = vec![(v & 0x7f) as u8];
    v >>= 7;
    while v > 0 {
        out.insert(0, (v & 0x7f) as u8 | 0x80);
        v >>= 7;
    }
    out
}

#[test]
fn reads_tempo_signature_and_markers() {
    load();
    // 480 tpq、BPM 150（400000us）、3/4、1 拍目の後に BPM 100 とマーカー「サビ」
    let mut trk = vec![];
    trk.extend(vlq(0)); trk.extend([0xff, 0x51, 3, 0x06, 0x1a, 0x80]);
    trk.extend(vlq(0)); trk.extend([0xff, 0x58, 4, 3, 2, 24, 8]);
    trk.extend(vlq(0)); trk.extend([0x90, 60, 100]);
    trk.extend(vlq(480)); trk.extend([60, 0]);
    trk.extend(vlq(0)); trk.extend([0xff, 0x51, 3, 0x09, 0x27, 0xc0]);
    trk.extend(vlq(0)); trk.extend([0xff, 0x06, 6]); trk.extend("サビ".as_bytes());
    trk.extend(vlq(0)); trk.extend([0xff, 0x2f, 0]);
    let mut smf = b"MThd\0\0\0\x06\0\0\0\x01\x01\xe0MTrk".to_vec();
    smf.extend((trk.len() as u32).to_be_bytes());
    smf.extend(trk);
    let p = solid_project(10, 10, 100, 100, vec![]);
    let cmd = znd_render::plugins::commands().into_iter().find(|c| c.key == "sevenc-nanashi.import_midi_tempos-aux2/import").unwrap();
    let args = [serde_json::json!(smf), serde_json::json!("a.mid"), serde_json::json!({ "comp": "main" })];
    let run = znd_script::run_plugin(&p, &cmd.source, Some(("onMidi", &args)), &context("main", &[])).unwrap();
    let p = run.project.unwrap();
    let c = &p.compositions[0];
    let g = c.beat_grid.as_ref().unwrap();
    assert_eq!((g.bpm, g.beats_per_bar), (150.0, 3));
    let m: Vec<(f64, String)> = c.markers.iter().map(|m| (m.time.seconds(), m.comment.clone())).collect();
    assert!(m.iter().any(|(t, s)| (t - 0.4).abs() < 1e-6 && s == "サビ"), "{m:?}");
    assert!(m.iter().any(|(t, s)| (t - 0.4).abs() < 1e-6 && s == "BPM 100"), "{m:?}");
}
