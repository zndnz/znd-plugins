mod common;
use common::*;
use znd_core::shape::{BezPath, Mask, MaskMode};

/// 40x40 の平面に、横一直線の開いたパス（モードなし）を置く。
fn project(fx: EffectInstance) -> znd_core::Project {
    let mut p = solid_project(40, 40, 100, 100, vec![fx]);
    let mut path = BezPath::polygon(&[[5.0, 20.0], [35.0, 20.0]]);
    path.closed = false;
    let mut m = Mask::new(path);
    m.mode = MaskMode::None;
    p.compositions[0].layers[0].masks.push(m);
    p
}

#[test]
fn draws_mask_path_as_line() {
    load();
    let fx = fx("plugin:ar-ca-na.waveform-line", 40, 40).set_f("thick", 4.0);
    let buf = render(&project(fx.clone()), 0.0);
    // 平面は (30, 30) から。線は y=50、x=35..65
    assert!(buf.pixel(50, 50)[3] > 0.99, "線の上");
    assert!(buf.pixel(50, 44)[3] < 0.01, "線の外（平面は消える）");
    assert!(buf.pixel(33, 50)[3] > 0.5, "丸い端");
    // 描く割合 50% なら右半分は無い
    let half = render(&project(fx.set_f("grow", 50.0)), 0.0);
    assert!(half.pixel(45, 50)[3] > 0.99);
    assert!(half.pixel(60, 50)[3] < 0.01);
}
