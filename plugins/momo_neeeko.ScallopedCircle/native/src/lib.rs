//! スカラップ円（もも猫）の移植: 円周を波形で波打たせた図形を描くカスタムオブジェクト。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

const TAU: f64 = 2.0 * PI;
fn fract(v: f64) -> f64 {
    v - v.floor()
}

fn render(f: &mut Frame) -> FxResult {
    let kind = f.i(0);
    let size = f.f(1).max(0.0) * f.scale();
    let ampp = f.f(2) / 100.0;
    let nf = f.f(3).round().max(1.0);
    let col = f.color(4);
    let cor = f.b(5);
    let r = size / 2.0;
    let wv = if kind == 4 && ampp < 0.0 { size } else { size + (size * ampp).abs() };
    let wi = wv.round().max(1.0) as usize;
    let c = wv / 2.0;
    let amp = size / 2.0 * ampp;
    let fq = if cor && kind == 2 { nf * 2.0 } else { nf };
    let inside = |px: f64, py: f64| -> bool {
        let (dx, dy) = (px - c, py - c);
        let a = dy.atan2(dx);
        if !cor {
            let range = match kind {
                0 => (a * fq).cos(),
                1 => 1.0 - (fract((a * fq) / TAU - 0.5) - 0.5).abs() * 4.0,
                2 => if fract((a * fq) / TAU + 0.25) < 0.5 { 1.0 } else { -1.0 },
                3 => 1.0 - (fract(a * fq / TAU + 0.5) - 1.0).abs() * 2.0,
                _ => (a * fq / 2.0).cos().abs(),
            };
            return dx * dx + dy * dy <= (r + amp * range).powi(2);
        }
        let dir = (a / (TAU / fq)).round() * (TAU / fq);
        let x = dx * (-dir).sin() + dy * (-dir).cos();
        let xmax = (r - amp) * (PI / fq).sin();
        let range = match kind {
            0 => -((x.abs() / xmax).clamp(0.0, 1.0) * PI).cos(),
            1 => 2.0 * (x.abs() / xmax).clamp(0.0, 1.0) - 1.0,
            2 => if ((dir / (TAU / fq)).round() as i64).rem_euclid(2) == 0 { -1.0 } else { 1.0 },
            3 => ((-x + amp * (PI / fq).sin()) / (r * (PI / fq).sin())).clamp(-1.0, 1.0),
            _ => -((x.abs() / (r * (PI / fq).sin())).clamp(0.0, 1.0) * PI / 2.0).cos().abs(),
        };
        if kind == 4 {
            return dx.hypot(dy) <= r + amp * -range;
        }
        let limit = (r + amp) * (PI / fq).cos() + ((r - amp) * (PI / fq).cos() - (r + amp) * (PI / fq).cos()) * (range + 1.0) * 0.5;
        dx * (-dir).cos() - dy * (-dir).sin() <= limit
    };
    const SS: usize = 4;
    let img = Image::generate(wi, wi, |x, y| {
        let mut n = 0;
        for j in 0..SS {
            for i in 0..SS {
                if inside(x as f64 + (i as f64 + 0.5) / SS as f64, y as f64 + (j as f64 + 0.5) / SS as f64) {
                    n += 1;
                }
            }
        }
        color::premul(col, n as f32 / (SS * SS) as f32)
    });
    let ox = (f.width as i32 - wi as i32) / 2;
    let oy = (f.height as i32 - wi as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "もも猫（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "momo_neeeko.ScallopedCircle", name: "スカラップ円", category: "描画", flags: THREAD_SAFE,
            params: [
                choice(c"type", c"波形", 0, c"正弦波|三角波|矩形波|ノコギリ波|正弦波の絶対値"),
                float(c"size", c"サイズ", 100.0, 0.0, 4000.0),
                float(c"amp", c"振幅%", 10.0, -100.0, 100.0),
                float(c"f", c"山の数", 10.0, 1.0, 128.0),
                color_hex(c"col", c"色", 0xffffff),
                boolean(c"cor", c"歪み補正", false),
            ],
            render: render,
        },
    ]
}
