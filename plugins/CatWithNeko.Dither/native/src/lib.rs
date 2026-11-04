//! Dither（こんにゃくねこ）の移植: 閾値を升目ごとの乱数で揺らす二値化。

use znd_plugin_sdk::*;

fn rand(x: f64, y: f64, seed: i64) -> f64 {
    let v = ((x * 12.9898 + y * 78.233) + seed as f64).sin() * 43758.5453;
    v - v.floor()
}

fn render(f: &mut Frame) -> FxResult {
    let th = f.f(0);
    let st = f.f(1);
    let seed = f.f(2) as i64;
    let size = (f.f(3).max(1.0) * f.scale()).max(1e-3);
    let ca = f.color(4);
    let cb = f.color(5);
    let w = f.width;
    for y in 0..f.height {
        for x in 0..w {
            let i = (y * w + x) * 4;
            let p = color::to_display([f.rgba[i], f.rgba[i + 1], f.rgba[i + 2], f.rgba[i + 3]]);
            let t = th + (rand(((x as f64 + 0.5) / size).floor(), ((y as f64 + 0.5) / size).floor(), seed) - 0.5) * st;
            let lum = 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
            let c = if lum >= t { ca } else { cb };
            let a = f.rgba[i + 3];
            f.rgba[i..i + 4].copy_from_slice(&[c[0] * a, c[1] * a, c[2] * a, a]);
        }
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Dither", name: "Dither", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"th", c"threshold", 0.5, 0.0, 1.0),
                float(c"str", c"strength", 0.5, 0.0, 1.0),
                float(c"seed", c"seed", 0.0, 0.0, 2000.0),
                float(c"size", c"size", 1.0, 1.0, 100.0),
                color_hex(c"ca", c"Color1", 0xffffff),
                color_hex(c"cb", c"Color2", 0x000000),
            ],
            render: render,
        },
    ]
}
