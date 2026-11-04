mod common;
use common::*;

#[test]
fn characters_appear_one_after_another() {
    let p = text_project("あいうえお", 40.0, 400, 120);
    let p = panel_event(&p, "korarei.AutoLyricAnimation_K", "ala", "apply", serde_json::Value::Null, context("main", &["t"]));
    assert!(p.validate().is_empty(), "{:?}", p.validate());
    let at0 = coverage(&render_expr(&p, 0.0));
    let mid = coverage(&render_expr(&p, 0.5));
    let late = coverage(&render_expr(&p, 2.0));
    // 最初はほとんど出ておらず、後になるほど出そろう
    assert!(at0 < mid && mid <= late + 1.0, "{at0} {mid} {late}");
    assert!(late > 100.0);
}
