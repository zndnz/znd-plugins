mod common;
use common::*;

#[test]
fn crops_to_opaque_bounds_and_pads() {
    load();
    // 40x30 の平面の左上 10x10 だけを残す（マスクの代わりに、先に 余白除去 で負の余白を使って中身を作る）
    let fx = fx("plugin:sigma-axis.AutoClipping_S", 40, 30)
        .set_b("enable_u", false)
        .set_b("enable_d", false)
        .set_b("enable_l", false)
        .set_b("enable_r", false)
        .set_f("pad_r", -30.0)
        .set_f("pad_d", -20.0);
    let p = solid_project(40, 30, 100, 100, vec![fx.clone()]);
    let buf = render(&p, 0.0);
    // 平面は (30, 35) から 40x30。左上 10x10 だけが残る
    assert!(buf.pixel(35, 40)[3] > 0.99);
    assert!(buf.pixel(45, 40)[3] < 0.01);
    assert!(buf.pixel(35, 50)[3] < 0.01);
    // 余白を足すと広がる（左に 5px）
    let grow = fx.set_f("pad_l", 5.0);
    let buf = render(&solid_project(40, 30, 100, 100, vec![grow]), 0.0);
    assert!(buf.pixel(27, 40)[3] < 0.01, "足した余白は透明");
    assert!(buf.pixel(31, 40)[3] > 0.99);
}
