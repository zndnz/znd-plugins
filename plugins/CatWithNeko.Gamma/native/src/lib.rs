//! Gamma（こんにゃくねこ）の移植: 全体と RGB ごとのガンマ補正。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let g = f.f(0);
    let gs = [g * f.f(1), g * f.f(2), g * f.f(3)];
    let mut img = f.input();
    aul::map_display(&mut img, |p| {
        let mut o = p;
        for c in 0..3 {
            o[c] = p[c].max(0.0).powf((1.0 / gs[c]) as f32);
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
