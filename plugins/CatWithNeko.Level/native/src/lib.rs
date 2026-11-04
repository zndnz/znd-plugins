//! Level（こんにゃくねこ）の移植: Min～Max を 0～1 に引き伸ばすレベル補正。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let (lo, hi) = (f.f(0) as f32, f.f(1) as f32);
    for p in f.rgba.chunks_exact_mut(4) {
        let a = p[3];
        if a <= 0.0 {
            continue;
        }
        for c in 0..3 {
            let v = ((p[c] / a - lo) / (hi - lo)).clamp(0.0, 1.0);
            p[c] = if v.is_nan() { 0.0 } else { v * a };
        }
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Level", name: "Level", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"min", c"Min", 0.0, 0.0, 1.0),
                float(c"max", c"Max", 1.0, 0.0, 1.0),
            ],
            render: render,
        },
    ]
}
