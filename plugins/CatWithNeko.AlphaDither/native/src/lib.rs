//! AlphaDither（こんにゃくねこ）の移植: 半透明を、升目ごとの乱数で「不透明か透明か」に置き換えるディザ合成。

use znd_plugin_sdk::*;

fn rand(gx: i64, gy: i64, seed: f64) -> f64 {
    let v = ((gx as f64 * 12.9898 + gy as f64 * 78.233) + seed * 437.585).sin() * 43758.5453123;
    v - v.floor()
}

fn render(f: &mut Frame) -> FxResult {
    let alpha = f.f(0) / 100.0;
    let size = (f.f(1).max(1.0) * f.scale()).max(1e-3);
    let seed = f.f(2);
    let (w, h) = (f.width, f.height);
    let src = f.input();
    for y in 0..h {
        for x in 0..w {
            // 元と同じく中心から升目を数え、0 に向かって切り捨てる
            let gx = ((x as f64 + 0.5 - w as f64 / 2.0) / size).trunc() as i64;
            let gy = ((y as f64 + 0.5 - h as f64 / 2.0) / size).trunc() as i64;
            let sx = (gx as f64 * size + w as f64 / 2.0) as isize;
            let sy = (gy as f64 * size + h as f64 / 2.0) as isize;
            let a = src.get(sx, sy)[3] as f64 * alpha;
            let p = src.px(x, y);
            let o = if rand(gx, gy, seed) < a && p[3] > 0.0 { [p[0] / p[3], p[1] / p[3], p[2] / p[3], 1.0] } else { [0.0; 4] };
            let i = (y * w + x) * 4;
            f.rgba[i..i + 4].copy_from_slice(&o);
        }
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.AlphaDither", name: "AlphaDither_C", category: "抽出", flags: THREAD_SAFE,
            params: [
                float(c"alpha", c"Alpha", 100.0, 0.0, 100.0),
                float(c"size", c"Size", 1.0, 1.0, 1000.0),
                float(c"seed", c"Seed", 0.0, 0.0, 4000.0),
            ],
            render: render,
        },
    ]
}
