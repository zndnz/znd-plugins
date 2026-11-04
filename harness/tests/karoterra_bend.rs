mod common;
use common::*;

#[test]
fn zero_curvature_keeps_image_and_curvature_bends() {
    load();
    let fx0 = fx("plugin:Karoterra.Bend.arc", 40, 40);
    let flat = render(&solid_project(40, 40, 100, 100, vec![fx0.clone()]), 0.0);
    // 曲率 0 なら元のまま（平面は (30,30)-(70,70)）
    assert!(flat.pixel(50, 50)[3] > 0.99);
    assert!(flat.pixel(32, 32)[3] > 0.99);
    assert!(flat.pixel(28, 50)[3] < 0.01);
    let bent = render(&solid_project(40, 40, 100, 100, vec![fx0.set_f("curvature", 100.0).set_f("sy", 20.0).set_f("ey", -20.0)]), 0.0);
    let moved = (0..100).flat_map(|y| (0..100).map(move |x| (x, y))).filter(|&(x, y)| (flat.pixel(x, y)[3] - bent.pixel(x, y)[3]).abs() > 0.5).count();
    assert!(moved > 50, "曲げると形が変わる {moved}");
    render(&solid_project(40, 40, 100, 100, vec![fx("plugin:Karoterra.Bend.bezier", 40, 40)]), 0.0);
}
