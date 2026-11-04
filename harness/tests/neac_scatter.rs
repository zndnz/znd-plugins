mod common;
use common::*;

#[test]
fn panel_applies_expressions_that_move_layers() {
    let p = solid_project(10, 10, 200, 200, vec![]);
    let p = panel_event(&p, "NeAc.Scatter", "scatter", "apply", serde_json::Value::Null, context("main", &["a"]));
    let l = &p.compositions[0].layers[0];
    assert!(l.effects.iter().any(|e| e.name == "Scatter"));
    // 元の位置（中央）から動いている
    let buf = render_expr(&p, 0.0);
    let moved = buf.pixel(100, 100)[3] < 0.5;
    let any = buf.data.chunks_exact(4).any(|px| px[3] > 0.5);
    assert!(any, "どこかに描かれている");
    assert!(moved, "中央から動いている");
}
