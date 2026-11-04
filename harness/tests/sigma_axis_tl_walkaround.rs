mod common;
use common::*;
use znd_core::*;

fn ctx_at(frame: i64, layers: &[&str]) -> serde_json::Value {
    let mut c = context("main", layers);
    c["frame"] = serde_json::json!(frame);
    c["time"] = serde_json::json!(frame as f64 / 30.0);
    c
}

fn frame_of(actions: &[serde_json::Value]) -> Option<i64> {
    actions.iter().find(|a| a["type"] == "frame").and_then(|a| a["frame"].as_i64())
}

#[test]
fn moves_between_edit_points_markers_and_beats() {
    let mut p = solid_project(10, 10, 100, 100, vec![]);
    p.compositions[0].layers[0].in_point = Time::from_seconds(1.0);
    p.compositions[0].layers[0].out_point = Time::from_seconds(2.0);
    p.compositions[0].markers.push(Marker { time: Time::from_seconds(3.0), duration: Time::ZERO, comment: String::new() });
    p.compositions[0].beat_grid = Some(BeatGrid::new(120.0));
    let k = |f: &str| format!("sigma-axis.tl_walkaround2/{f}");
    let (_, a) = run_command(&p, &k("nextEdgeLayer"), ctx_at(0, &["a"]));
    assert_eq!(frame_of(&a), Some(30));
    let (_, a) = run_command(&p, &k("nextEdgeLayer"), ctx_at(30, &["a"]));
    assert_eq!(frame_of(&a), Some(60));
    let (_, a) = run_command(&p, &k("marker1"), ctx_at(0, &[]));
    assert_eq!(frame_of(&a), Some(90));
    let (_, a) = run_command(&p, &k("nextBeat"), ctx_at(0, &[]));
    assert_eq!(frame_of(&a), Some(15), "BPM 120 の 1 拍は 15 フレーム");
    let (p2, _) = run_command(&p, &k("toggleMarker"), ctx_at(10, &[]));
    assert_eq!(p2.compositions[0].markers.len(), 2);
}
