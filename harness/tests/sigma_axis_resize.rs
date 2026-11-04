mod common;
use common::*;

#[test]
fn resize_changes_size_around_center() {
    load();
    for up in 0..6 {
        let fx = fx("plugin:sigma-axis.Resize_S", 20, 20).set_f("zoom", 200.0).set_i("upscale", up);
        let buf = render(&solid_project(20, 20, 100, 100, vec![fx]), 0.0);
        // 平面は (40,40)-(60,60)。2 倍で (30,30)-(70,70)
        assert!(buf.pixel(32, 50)[3] > 0.99, "up {up}");
        assert!(buf.pixel(28, 50)[3] < 0.01, "up {up}");
    }
    for dn in 0..6 {
        let fx = fx("plugin:sigma-axis.Resize_S", 20, 20).set_f("zoom", 50.0).set_i("downscale", dn);
        let buf = render(&solid_project(20, 20, 100, 100, vec![fx]), 0.0);
        // 半分で (45,45)-(55,55)
        assert!(buf.pixel(50, 50)[3] > 0.99, "dn {dn}");
        assert!(buf.pixel(43, 50)[3] < 0.01, "dn {dn}");
    }
}

#[test]
fn box_fits_and_pads() {
    load();
    // 40x20 を 20x20 の箱に内接 → 20x10、上下に余白で 20x20
    let fx = fx("plugin:sigma-axis.Resize_S.box", 40, 20).set_f("width", 20.0).set_f("height", 20.0);
    let buf = render(&solid_project(40, 20, 100, 100, vec![fx.clone()]), 0.0);
    assert!(buf.pixel(50, 50)[3] > 0.99);
    assert!(buf.pixel(50, 43)[3] < 0.01, "上の余白は透明");
    assert!(buf.pixel(38, 50)[3] < 0.01);
    // 外接最小 → 40x20 を 20x20 で覆う: 縦が合うので 40x20 のまま切って 20x20
    let cov = fx.set_i("mode", 1);
    let buf = render(&solid_project(40, 20, 100, 100, vec![cov]), 0.0);
    assert!(buf.pixel(42, 50)[3] > 0.99);
    assert!(buf.pixel(35, 50)[3] < 0.01, "はみ出た分は切る");
}
