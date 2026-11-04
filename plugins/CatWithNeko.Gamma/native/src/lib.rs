//! Gamma（こんにゃくねこ）の移植: 全体と RGB ごとのガンマ補正。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let g = f.f(0);
    let gs = [g * f.f(1), g * f.f(2), g * f.f(3)];
    for p in f.rgba.chunks_exact_mut(4) {
        let a = p[3];
        if a <= 0.0 {
            continue;
        }
        for c in 0..3 {
            let v = (p[c] / a).max(0.0) as f64;
            p[c] = (v.powf(1.0 / gs[c]) as f32) * a;
        }
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Gamma", name: "Gamma", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"gamma", c"Gamma", 1.0, 0.0, 10.0),
                float(c"r", c"R", 1.0, 0.0, 10.0),
                float(c"g", c"G", 1.0, 0.0, 10.0),
                float(c"b", c"B", 1.0, 0.0, 10.0),
            ],
            render: render,
        },
    ]
}
