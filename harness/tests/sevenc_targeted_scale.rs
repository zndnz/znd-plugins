mod common;
use common::*;

#[test]
fn scales_to_target_size() {
    load();
    let mut p = solid_project(40, 20, 200, 200, vec![]);
    p.compositions[0].layers[0].transform.scale.expression = Some(znd_core::Expression { source: "targetedScale(value, 100, 100, 1)".into(), enabled: true });
    let buf = render_expr(&p, 0.0);
    // 40x20 を 100x100 に収める → 100x50
    assert!(buf.pixel(52, 100)[3] > 0.99 && buf.pixel(148, 100)[3] > 0.99);
    assert!(buf.pixel(48, 100)[3] < 0.01 && buf.pixel(100, 72)[3] < 0.01);
}
