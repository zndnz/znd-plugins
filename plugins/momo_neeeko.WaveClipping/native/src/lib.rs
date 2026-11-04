//! ウェーブクリッピング（もも猫）の移植: 波の形で斜めに切り取る。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

const TAU: f64 = 2.0 * PI;
fn fract(v: f64) -> f64 {
    v - v.floor()
}
fn rand(x: f64) -> f64 {
    fract((x * 12.9898).sin() * 43758.5453)
}
fn noise1(x: f64) -> f64 {
    let i = x.floor();
    let t = x - i;
    let fd = t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    rand(i) + (rand(i + 1.0) - rand(i)) * fd
}
fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let amp = f.f(1) * sc;
    let mut olen = f.f(0) * sc;
    if olen == 0.0 {
        olen = if amp == 0.0 { 1.0 } else { (amp * 4.0).abs() };
    }
    let len = TAU / olen;
    let pha = f.f(2) / 360.0 * TAU;
    let kind = f.i(3);
    let (w, h) = (f.width as f64, f.height as f64);
    let cx = -w / 2.0 - f.f(4) * sc;
    let cy = -h / 2.0 - f.f(5) * sc;
    let (sr, cr) = (-f.f(6)).to_radians().sin_cos();
    let method = f.i(7);
    let gap = f.f(8) * sc;
    let g = 1.0 / TAU;
    let wd = f.width;
    for y in 0..f.height {
        for x in 0..wd {
            let px = x as f64 + 0.5 + cx;
            let py = y as f64 + 0.5 + cy;
            let dx = px * cr - py * sr;
            let dy = px * sr + py * cr;
            let dir = dx * len + pha;
            let (range, smooth) = match kind {
                1 => ((1.0 - (fract(dir * g + 0.25) - 0.5).abs() * 4.0) * amp, ((amp * 0.5).abs() - 1.0).min((amp / olen) * 4.0).abs() + 0.5),
                2 => (if fract(dir * g) < 0.5 { amp } else { -amp }, 0.5),
                3 => ((1.0 - (fract(dir * g + 0.5) - 1.0).abs() * 2.0) * amp, ((amp * 0.5).abs() - 1.0).min((amp / olen) * 2.0).abs() + 0.5),
                4 => (dir.sin().abs() * amp, ((amp * 0.5).abs() - 1.0).min((len * amp * dir.cos()).abs()).abs() + 0.5),
                5 => ((noise1(dir) * 2.0 - 1.0) * amp, 1.0),
                _ => (dir.sin() * amp, ((amp * 0.5).abs() - 1.0).min((len * amp * dir.cos()).abs()).abs() + 0.5),
            };
            let ran = -dy - range;
            let band = || smoothstep(-smooth, smooth, ran + gap) * (1.0 - smoothstep(-smooth, smooth, ran - gap));
            let a = match method {
                1 => band(),
                2 => 1.0 - band(),
                _ => smoothstep(-smooth, smooth, ran),
            } as f32;
            let i = (y * wd + x) * 4;
            for k in 0..4 {
                f.rgba[i + k] *= a;
            }
        }
    }
    Ok(())
}

plugin! {
    vendor: "もも猫（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "momo_neeeko.WaveClipping", name: "ウェーブクリッピング", category: "クリッピング", flags: THREAD_SAFE,
            params: [
                float(c"len", c"波長", 240.0, 0.0, 10000.0),
                float(c"amp", c"振幅", 30.0, -5000.0, 5000.0),
                float(c"pha", c"位相", 0.0, -36000.0, 36000.0),
                choice(c"type", c"波形", 0, c"正弦波|三角波|矩形波|ノコギリ波|正弦波の絶対値|ランダム"),
                float(c"cx", c"中心X", 0.0, -10000.0, 10000.0),
                float(c"cy", c"中心Y", 0.0, -10000.0, 10000.0),
                float(c"rotate", c"角度", 0.0, -36000.0, 36000.0),
                choice(c"method", c"クリッピング方法", 0, c"一方向|双方向|双方向(反転)"),
                float(c"gap", c"幅", 100.0, 0.0, 10000.0),
            ],
            render: render,
        },
    ]
}
