//! Leave Color（こんにゃくねこ）の移植: 指定色の向きに近い色だけ残し、ほかを灰色にする色抜き。

use znd_plugin_sdk::*;

fn norm(c: [f32; 3]) -> [f32; 3] {
    let l = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
    if l < 1e-9 {
        [f32::NAN; 3]
    } else {
        c.map(|v| v / l)
    }
}

fn render(f: &mut Frame) -> FxResult {
    let th = (f.f(0) / 100.0) as f32;
    let pc = f.color(1);
    let col = norm([0, 1, 2].map(|i| color::linear_to_srgb(pc[i])));
    let wts = match f.i(2) {
        0 => [0.299, 0.587, 0.114],
        2 => [0.2627, 0.6780, 0.0593],
        3 => [0.3333, 0.3333, 0.3333],
        _ => [0.2126, 0.7152, 0.0722],
    };
    let inv = f.b(3);
    let mut img = f.input();
    aul::map_display(&mut img, |p| {
        let n = norm([p[0], p[1], p[2]]);
        let sim = n[0] * col[0] + n[1] * col[1] + n[2] * col[2];
        // 元のシェーダーと同じく、NaN（黒など）の比較は偽になる
        if (!inv && sim < th) || (inv && sim > th) {
            let v = p[0] * wts[0] + p[1] * wts[1] + p[2] * wts[2];
            [v, v, v, p[3]]
        } else {
            p
        }
    });
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.LeaveColor", name: "Leave Color", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"th", c"Threshold", 50.0, 0.0, 100.0),
                color_hex(c"col", c"Color", 0xffffff),
                choice(c"lum_mode", c"Lum Mode", 1, c"BT.601|BT.709|BT.2020|Mean"),
                boolean(c"inv", c"Invert", false),
            ],
            render: render,
        },
    ]
}
