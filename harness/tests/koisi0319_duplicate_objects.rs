mod common;
use common::*;

#[test]
fn duplicates_with_gap() {
    load();
    let mut p = solid_project(10, 10, 100, 100, vec![]);
    p.compositions[0].layers[0].out_point = znd_core::Time::from_seconds(1.0);
    let kf = znd_core::Keyframe::linear(znd_core::Time::from_seconds(0.5), 50.0);
    p.compositions[0].layers[0].transform.opacity.keyframes = vec![kf];
    let pan = znd_render::plugins::panels().into_iter().find(|x| x.key == "koisi0319.Duplicate_objects/dup").unwrap();
    let src = format!("{}\n;function __go() {{ onEvent('apply', null, {{ gap: 15, mode: 0, count: 2 }}); }}", pan.source);
    let run = znd_script::run_plugin(&p, &src, Some(("__go", &[])), &context("main", &["a"])).unwrap();
    let p = run.project.unwrap();
    let mut starts: Vec<f64> = p.compositions[0].layers.iter().map(|l| l.in_point.seconds()).collect();
    starts.sort_by(f64::total_cmp);
    assert_eq!(starts.len(), 3);
    assert!((starts[1] - 1.5).abs() < 1e-6 && (starts[2] - 3.0).abs() < 1e-6, "{starts:?}");
    // 複製のキーフレームも一緒にずれる
    let last = p.compositions[0].layers.iter().max_by(|a, b| a.in_point.cmp(&b.in_point)).unwrap();
    assert!((last.transform.opacity.keyframes[0].time.seconds() - 3.5).abs() < 1e-6);
    assert!(p.validate().is_empty());
}
