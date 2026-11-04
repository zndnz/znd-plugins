mod common;
use common::*;

#[test]
fn characters_come_in_over_the_layer() {
    let p = text_project("かきくけこ", 40.0, 400, 120);
    let p = panel_event(&p, "NeAc.LyricMotioN", "lyricmotion", "apply", serde_json::Value::Null, context("main", &["t"]));
    assert!(p.validate().is_empty());
    let a = coverage(&render_expr(&p, 0.0));
    let b = coverage(&render_expr(&p, 3.9));
    assert!(a < b, "{a} {b}");
}
