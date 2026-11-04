//! Exposure（こんにゃくねこ）の移植: 露出（倍率）→ ガンマ → オフセット。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let (ex, g, off) = (f.f(0) as f32, f.f(1) as f32, f.f(2) as f32);
    let mut img = f.input();
    aul::map_display(&mut img, |p| {
        let mut o = p;
        for c in 0..3 {
            o[c] = (p[c] * ex).clamp(0.0, 1.0).powf(1.0 / g) + off;
        }
        o
    });
    f.set(img);
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
