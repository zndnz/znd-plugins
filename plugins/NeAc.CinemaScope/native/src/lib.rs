//! シネマスコープ（猫愛）の移植: 画面の上下（任意の角度）に帯を描く。
//! 中心を通る線から、法線方向に「オフセット ± 幅」の範囲だけを残して外側を塗る。幅 0 なら全面を塗る。
//! レイヤーの絵を帯で置き換えるので、コンポの大きさの平面に掛けて使う。

use znd_plugin_sdk::*;

/// 幅・オフセットをレイヤー座標の px にする（割合はスクリーンの幅・高さの半分に対する %）。
fn to_px(mode: usize, v: f64, w: f64, h: f64) -> f64 {
    match mode {
        1 => v / 100.0 * w / 2.0,
        2 => v / 100.0 * h / 2.0,
        _ => v,
    }
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let (lw, lh) = (f.width as f64 / s, f.height as f64 / s);
    let center = f.point(0);
    let raw_width = f.f(3);
    let width = to_px(f.i(2), raw_width, lw, lh) * s;
    let offset = to_px(f.i(5), f.f(6), lw, lh) * s;
    let fill = color::premul(f.color(4), 1.0);
    let rad = (90.0 + f.f(1)).to_radians();
    let (dx, dy) = (rad.cos(), rad.sin());
    let wi = f.width;
    for (i, p) in f.rgba.chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i % wi) as f64 + 0.5 - center[0], (i / wi) as f64 + 0.5 - center[1]);
        let cov = if raw_width == 0.0 {
            1.0
        } else {
            let proj = x * dx + y * dy - offset;
            // 帯の境界までの距離で 1px のアンチエイリアス
            ((proj.abs() - width.abs()) + 0.5).clamp(0.0, 1.0) as f32
        };
        for k in 0..4 {
            p[k] = fill[k] * cov;
        }
    }
    Ok(())
}

plugin! {
    vendor: "猫愛（znd 移植: znd-plugins）", version: "r5", license: "MIT",
    effects: [
        {
            id: "NeAc.CinemaScope", name: "シネマスコープ", category: "描画", flags: THREAD_SAFE,
            params: [
                point(c"center", c"中心", 0.5, 0.5),
                float(c"angle", c"角度", 0.0, -10000.0, 10000.0),
                choice(c"width_mode", c"幅タイプ", 0, c"ピクセル|割合（スクリーン幅）|割合（スクリーン高さ）"),
                float(c"width", c"幅", 0.0, 0.0, 50000.0),
                color(c"color", c"色", [0.0, 0.0, 0.0, 1.0]),
                choice(c"offset_mode", c"オフセットタイプ", 0, c"ピクセル|割合（スクリーン幅）|割合（スクリーン高さ）"),
                float(c"offset", c"オフセット", 0.0, -10000.0, 10000.0),
            ],
            render: render,
        },
    ]
}
