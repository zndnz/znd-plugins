//! Exposure（こんにゃくねこ）の移植: 露出（倍率）→ ガンマ → オフセット。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let (ex, g, off) = (f.f(0), f.f(1), f.f(2));
    for p in f.rgba.chunks_exact_mut(4) {
        let a = p[3];
        if a <= 0.0 {
            continue;
        }
        for c in 0..3 {
            let v = ((p[c] / a) as f64 * ex).clamp(0.0, 1.0).powf(1.0 / g) + off;
            p[c] = v as f32 * a;
        }
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Exposure", name: "Exposure", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"exposure", c"Exposure", 1.0, 0.0, 10.0),
                float(c"gamma", c"Gamma", 1.0, 0.0, 10.0),
                float(c"offset", c"Offset", 0.0, -1.0, 1.0),
            ],
            render: render,
        },
    ]
}
