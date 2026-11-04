//! Unmult（mes51）の移植: 黒（または白）の背景を透明に変える（明るさをアルファに置き換える）。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let white = f.i(0) == 1;
    for p in f.rgba.chunks_exact_mut(4) {
        let mut c = to_display([p[0], p[1], p[2], p[3]]);
        if white {
            for v in &mut c[..3] {
                *v = 1.0 - *v;
            }
        }
        // 表示値で、色にアルファを掛けた値の最大を新しいアルファにする
        let pm = [c[0] * c[3], c[1] * c[3], c[2] * c[3]];
        let a = pm[0].max(pm[1]).max(pm[2]);
        let out = if a <= 0.0 {
            [0.0; 4]
        } else {
            let t = [pm[0] / a, pm[1] / a, pm[2] / a];
            let t = if white { [1.0 - t[0], 1.0 - t[1], 1.0 - t[2]] } else { t };
            from_display([t[0], t[1], t[2], a])
        };
        p.copy_from_slice(&out);
    }
    Ok(())
}

plugin! {
    vendor: "mes51（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "mes51.Unmult", name: "Unmult（背景を透明に）", category: "キーイング", flags: THREAD_SAFE,
            params: [choice(c"background", c"背景色", 0, c"黒|白")],
            render: render,
        },
    ]
}
