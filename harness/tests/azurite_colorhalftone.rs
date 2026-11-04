mod common;
use common::*;

#[test]
fn white_stays_white_and_black_becomes_dots() {
    load();
    let fx = fx("plugin:azurite.ColorHalftone", 40, 40);
    // 白い平面は網点が無く、背景の白のまま
    let buf = render(&colored_project(40, 40, 100, 100, [1.0, 1.0, 1.0], vec![fx.clone()]), 0.0);
    let p = buf.pixel(50, 50);
    assert!(p[0] > 0.99 && p[1] > 0.99 && p[2] > 0.99 && p[3] > 0.99, "{p:?}");
    // 黒い平面は 3 色が重なって暗くなる
    let buf = render(&colored_project(40, 40, 100, 100, [0.0, 0.0, 0.0], vec![fx.clone()]), 0.0);
    let mut dark = 0;
    for y in 32..68 {
        for x in 32..68 {
            if buf.pixel(x, y)[0] < 0.2 {
                dark += 1;
            }
        }
    }
    assert!(dark > 36 * 36 / 2, "暗い画素 {dark}");
    // 平面の外は透明（元のアルファを維持）
    assert!(buf.pixel(20, 50)[3] < 0.01);
    // 形状をすべて通す
    for s in 0..8 {
        render(&colored_project(40, 40, 100, 100, [0.5, 0.2, 0.8], vec![fx.clone().set_i("shape", s)]), 0.0);
    }
}
