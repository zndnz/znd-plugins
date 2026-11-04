//! ラスターM（もも猫）の移植: 6 種の波形と向きを選べるラスター。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

const TAU: f64 = 2.0 * PI;
fn fract(v: f64) -> f64 {
    v - v.floor()
}
fn rand(x: f64) -> f64 {
    fract((x * 12.9898).sin() * 43758.5453)
}
/// 元の 1 次元ノイズ（滑らかにつないだ乱数）。
pub fn noise1(x: f64) -> f64 {
    let i = x.floor();
    let t = x - i;
    let fd = t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    rand(i) + (rand(i + 1.0) - rand(i)) * fd
}

/// 位相 ph（ラジアン）での波（-1～1、正弦波の絶対値は 0～1）。
pub fn wave(kind: usize, ph: f64) -> f64 {
    match kind {
        1 => 1.0 - (fract(ph / TAU + 0.25) - 0.5).abs() * 4.0,
        2 => if fract(ph / TAU) < 0.5 { 1.0 } else { -1.0 },
        3 => 1.0 - (fract(ph / TAU + 0.5) - 1.0).abs() * 2.0,
        4 => ph.sin().abs(),
        5 => noise1(ph) * 2.0 - 1.0,
        _ => ph.sin(),
    }
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let amp = f.f(1) * sc;
    let mut len = f.f(0) * sc;
    if len == 0.0 {
        len = if amp == 0.0 { 1.0 } else { (amp * 4.0).abs() };
    }
    let pha = (-f.f(2)).to_radians();
    let rot = (-f.f(3)).to_radians();
    let kind = f.i(4);
    let ex = (amp * f.f(3).to_radians().sin()).abs().ceil() as usize;
    let ey = (amp * f.f(3).to_radians().cos()).abs().ceil() as usize;
    let src = f.input().padded(ex, ey, ex, ey);
    let (w2, h2) = (src.w as f64 / 2.0, src.h as f64 / 2.0);
    let (rs, rc) = rot.sin_cos();
    let mut out = Image::new(src.w, src.h);
    for y in 0..src.h {
        for x in 0..src.w {
            let n = [x as f64 + 0.5 - w2, y as f64 + 0.5 - h2];
            let xx = n[0] * rc - n[1] * rs;
            let wv = wave(kind, xx / len * TAU + pha) * amp;
            out.set(x, y, src.get((n[0] + w2 + rs * wv).floor() as isize, (n[1] + h2 + rc * wv).floor() as isize));
        }
    }
    f.output(out, -(ex as i32), -(ey as i32));
    Ok(())
}

plugin! {
    vendor: "もも猫（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "momo_neeeko.RasterM", name: "ラスターM", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"len", c"波長", 240.0, 0.0, 5000.0),
                float(c"amp", c"振幅", 30.0, -5000.0, 5000.0),
                float(c"pha", c"位相", 0.0, -36000.0, 36000.0),
                float(c"rotate", c"角度", 0.0, -7200.0, 7200.0),
                choice(c"type", c"波形", 0, c"正弦波|三角波|矩形波|ノコギリ波|正弦波の絶対値|ランダム"),
            ],
            render: render,
        },
    ]
}
