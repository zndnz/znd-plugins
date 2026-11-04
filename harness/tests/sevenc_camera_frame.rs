mod common;
use common::*;

#[test]
fn draws_corners_inside_margin() {
    load();
    // 100x100 のコンポ、余白 10 → 80x80 の枠。線の長さ 20・太さ 4
    let fx = fx("plugin:sevenc-nanashi.aviutl2-scripts.camera_frame-obj2", 100, 100).set_f("width", 10.0).set_f("height", 10.0).set_f("line_length", 20.0).set_f("line_thickness", 4.0);
    let buf = render(&solid_project(100, 100, 100, 100, vec![fx]), 0.0);
    assert!(buf.pixel(12, 11)[3] > 0.99 && buf.pixel(88, 88)[3] > 0.99, "隅の線");
    assert!(buf.pixel(50, 11)[3] < 0.01 && buf.pixel(50, 50)[3] < 0.01, "辺の中ほどと中心は空く");
}
