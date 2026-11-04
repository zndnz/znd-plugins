mod common;
use common::*;

#[test]
fn work_area_and_insert_actions() {
    let mut p = solid_project(10, 10, 100, 100, vec![]);
    p.compositions[0].layers[0].in_point = znd_core::Time::from_seconds(1.0);
    p.compositions[0].layers[0].out_point = znd_core::Time::from_seconds(2.0);
    let (_, a) = run_command(&p, "sevenc-nanashi.object_as_frame_selection-aux2/select", context("main", &["a"]));
    let w = a.iter().find(|x| x["type"] == "work_area").expect("ワークエリアの動作");
    assert_eq!((w["start"].as_i64(), w["end"].as_i64()), (Some(30), Some(60)));
    let (_, a) = run_command(&p, "sevenc-nanashi.insert-aux2/insert", context("main", &[]));
    assert!(a.iter().any(|x| x["type"] == "import"));
}
