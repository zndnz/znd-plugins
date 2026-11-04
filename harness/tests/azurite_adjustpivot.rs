mod common;
use common::*;

#[test]
fn moves_anchor_to_top_left_and_keeps_image() {
    load();
    let p = solid_project(40, 20, 100, 100, vec![]);
    let before = render_expr(&p, 0.0);
    let p2 = panel_event(&p, "azurite.AdjustPivot_A", "pivot", "p1", serde_json::Value::Null, context("main", &["a"]));
    let t = &p2.compositions[0].layers[0].transform;
    assert!(t.anchor.expression.is_some() && t.position.expression.is_some(), "エクスプレッションが付く");
    let after = render_expr(&p2, 0.0);
    for (x, y) in [(31, 41), (69, 59), (50, 50), (29, 50)] {
        close(before.pixel(x, y)[3], after.pixel(x, y)[3], 1e-4, "絵はその場に残る");
    }
}
