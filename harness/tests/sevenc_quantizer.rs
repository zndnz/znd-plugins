mod common;
use common::*;
use znd_core::*;

#[test]
fn finds_and_fixes_off_grid_points() {
    load();
    let mut p = solid_project(10, 10, 100, 100, vec![]);
    p.compositions[0].beat_grid = Some(BeatGrid { bpm: 120.0, beats_per_bar: 4, subdivision: 1, offset: Time::ZERO });
    // 0.5 秒ごとのグリッド。開始を 1 フレームずらす
    p.compositions[0].layers[0].in_point = Time::from_seconds(1.0 / 30.0);
    p.compositions[0].layers[0].out_point = Time::from_seconds(2.0);
    let pan = znd_render::plugins::panels().into_iter().find(|x| x.key == "sevenc-nanashi.quantizer-aux2/quantizer").unwrap();
    let src = format!("{}\n;function __go() {{ var s = {{}}; onEvent('fixall', null, s); }}", pan.source);
    let run = znd_script::run_plugin(&p, &src, Some(("__go", &[])), &context("main", &[])).unwrap();
    let p = run.project.unwrap();
    assert_eq!(p.compositions[0].layers[0].in_point, Time::ZERO);
}
