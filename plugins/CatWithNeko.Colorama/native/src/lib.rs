//! Colorama（こんにゃくねこ）の移植: 明るさを 6 色の巡回グラデーションに置き換え、元の絵と合成する簡易コロラマ。

use znd_plugin_sdk::*;

fn ycc(c: [f32; 3]) -> [f32; 3] {
    let (r, g, b) = (c[0] * 255.0, c[1] * 255.0, c[2] * 255.0);
    [
        (4918.0 * r + 354.0) / 1024.0 + (9655.0 * g + 585.0) / 1024.0 + (1875.0 * b + 523.0) / 1024.0,
        (-2775.0 * r + 240.0) / 1024.0 + (-5449.0 * g + 515.0) / 1024.0 + (8224.0 * b + 256.0) / 1024.0,
        (8224.0 * r + 256.0) / 1024.0 + (-6887.0 * g + 110.0) / 1024.0 + (-1337.0 * b + 646.0) / 1024.0,
    ]
}
fn rgb(c: [f32; 3]) -> [f32; 3] {
    let (y, cb, cr) = (c[0], c[1], c[2]);
    [
        (255.0 * y + (22881.0 * cr / 65536.0 + 3.0) * 1024.0) / 4096.0 / 255.0,
        (255.0 * y + ((-5616.0 * cb / 65536.0) + (-11655.0 * cr / 65536.0) + 3.0) * 1024.0) / 4096.0 / 255.0,
        (255.0 * y + (28919.0 * cb / 65536.0 + 3.0) * 1024.0) / 4096.0 / 255.0,
    ]
}

const MODES: [u32; 17] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 20, 21, 22, 25];

fn render(f: &mut Frame) -> FxResult {
    let shift = f.f(0) / 100.0;
    let cycle = f.f(1);
    let step = f.f(2).round().clamp(2.0, 6.0) as usize;
    let cols: Vec<[f32; 3]> = (0..6).map(|k| { let c = f.color(3 + k); [0, 1, 2].map(|i| color::linear_to_srgb(c[i])) }).collect();
    let wts = match f.i(9) {
        1 => [0.2126, 0.7152, 0.0722],
        2 => [0.2627, 0.6780, 0.0593],
        3 => [0.3333, 0.3333, 0.3333],
        _ => [0.299, 0.587, 0.114],
    };
    let inv = f.b(10);
    let mode = MODES[f.i(11).min(MODES.len() - 1)];
    let mix = (f.f(12) / 100.0) as f32;
    for p in f.rgba.chunks_exact_mut(4) {
        // 計算は元と同じく乗算済みの表示値で行う
        let d = color::to_display([p[0], p[1], p[2], p[3]]);
        let a0 = d[3];
        let t0 = [d[0] * a0, d[1] * a0, d[2] * a0, a0];
        let src = if inv { [(1.0 - d[0]) * a0, (1.0 - d[1]) * a0, (1.0 - d[2]) * a0] } else { [t0[0], t0[1], t0[2]] };
        let lum = (src[0] * wts[0] + src[1] * wts[1] + src[2] * wts[2]).clamp(0.0, 1.0) as f64;
        let ph = (lum * cycle + shift).rem_euclid(1.0) * step as f64;
        let seg = (ph.floor() as usize).min(step - 1);
        let fr = (ph - ph.floor()) as f32;
        let ca = cols[seg];
        let cb = cols[if seg + 1 < step { seg + 1 } else { 0 }];
        let c1 = [0, 1, 2].map(|k| (ca[k] + (cb[k] - ca[k]) * fr) * a0);
        let c0 = [t0[0], t0[1], t0[2]];
        let (y0, y1) = (ycc(c0), ycc(c1));
        let mut alpha = a0;
        let per = |g: fn(f32, f32) -> f32| [g(c0[0], c1[0]), g(c0[1], c1[1]), g(c0[2], c1[2])];
        let out = match mode {
            1 => per(|a, b| a + b),
            2 => per(|a, b| a - b),
            3 => per(|a, b| a * b),
            4 => per(|a, b| 1.0 - (1.0 - a) * (1.0 - b)),
            5 => per(|a, b| if b <= 0.5 { 1.0 - (1.0 - a) * (1.0 - b) } else { a * b }),
            6 => per(f32::max),
            7 => per(f32::min),
            8 => rgb([y1[0], y0[1], y0[2]]),
            9 => rgb([y0[0], y1[1], y1[2]]),
            10 => per(|a, b| a + b - 1.0),
            11 => per(|a, b| a + b * 2.0 - 1.0),
            12 => per(|a, b| (a - b).abs()),
            20 => { alpha = a0 + a0; per(|a, b| (a + b) / 2.0) }
            21 => per(|a, b| (a + b) / 2.0),
            22 => { alpha = 0.0; per(|a, b| (a + b) / 2.0) }
            _ => c1,
        };
        let r = [0, 1, 2].map(|k| t0[k] * (1.0 - mix) + out[k] * alpha * mix);
        let ra = (t0[3] * (1.0 - mix) + alpha * mix).clamp(0.0, 1.0);
        let o = if ra > 1e-6 { color::from_display([(r[0] / ra).clamp(0.0, 1.0), (r[1] / ra).clamp(0.0, 1.0), (r[2] / ra).clamp(0.0, 1.0), ra]) } else { [0.0; 4] };
        p.copy_from_slice(&o);
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Colorama", name: "Colorama", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"shift", c"Shift", 0.0, -5000.0, 5000.0),
                float(c"cycle", c"Cycle", 1.0, 1.0, 50.0),
                float(c"step", c"Step", 6.0, 2.0, 6.0),
                color_hex(c"c1", c"Color1", 0xff0000),
                color_hex(c"c2", c"Color2", 0xffff00),
                color_hex(c"c3", c"Color3", 0x00ff00),
                color_hex(c"c4", c"Color4", 0x00ffff),
                color_hex(c"c5", c"Color5", 0x0000ff),
                color_hex(c"c6", c"Color6", 0xff00ff),
                choice(c"lmode", c"Lum Mode", 0, c"bt-601|bt-709|bt-2020|mean"),
                boolean(c"inv", c"Invert", false),
                choice(c"bmode", c"Blend Mode", 0, c"none|add|sub|mul|screen|overlay|light|dark|brightness|chroma|shadow|light_dark|diff|alpha_add|alpha_max|alpha_sub|alpha_add2"),
                float(c"mix", c"mix", 100.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}
