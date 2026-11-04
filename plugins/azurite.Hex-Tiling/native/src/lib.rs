//! Hex-Tiling（azurite）の移植: 画像を、継ぎ目と繰り返しが目立たないよう六角形のタイルで敷き詰める。
//! 計算は Mikkelsen (2022) "Practical Real-Time Hex-Tiling"（MIT, mmikk）に基づく元のシェーダーと同じ。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

type V = (f64, f64);

fn fract(v: f64) -> f64 {
    v - v.floor()
}

fn triangle_grid(st: V, tile: f64) -> ([f64; 3], [(i64, i64); 3]) {
    let k = 2.0 * 3f64.sqrt() / tile;
    let st = (st.0 * k, st.1 * k);
    let sk = (st.0 - 0.57735027 * st.1, 1.15470054 * st.1);
    let base = (sk.0.floor() as i64, sk.1.floor() as i64);
    let (tx, ty) = (fract(sk.0), fract(sk.1));
    let tz = 1.0 - tx - ty;
    let s = if -tz >= 0.0 { 1.0 } else { 0.0 };
    let s2 = 2.0 * s - 1.0;
    let si = s as i64;
    ([-tz * s2, s - ty * s2, s - tx * s2], [(base.0 + si, base.1 + si), (base.0 + si, base.1 + 1 - si), (base.0 + 1 - si, base.1 + si)])
}

fn hash(p: (i64, i64)) -> V {
    let (x, y) = (p.0 as f64, p.1 as f64);
    (fract((127.1 * x + 311.7 * y).sin() * 43758.5453), fract((269.5 * x + 183.3 * y).sin() * 43758.5453))
}

fn center(v: (i64, i64)) -> V {
    let k = 2.0 * 3f64.sqrt();
    ((v.0 as f64 + 0.5 * v.1 as f64) / k, (v.1 as f64 / 1.15470054) / k)
}

fn rot(idx: (i64, i64), strength: f64) -> (f64, f64) {
    let mut a = ((idx.0 * idx.1).abs() + (idx.0 + idx.1).abs()) as f64 + PI;
    a = a.rem_euclid(2.0 * PI);
    if a > PI {
        a -= 2.0 * PI;
    }
    a *= strength;
    (a.cos(), a.sin())
}

fn hex_weights(w: [f64; 3], v1: (i64, i64), v3: (i64, i64)) -> [f64; 3] {
    let v = (v1.0 - v1.1).rem_euclid(3);
    let vh = if v < 2 { v + 1 } else { 0 };
    let vl = if v > 0 { v - 1 } else { 2 };
    let (a, b) = if v1.0 < v3.0 { (vl, vh) } else { (vh, vl) };
    std::array::from_fn(|i| if b == i as i64 { w[2] } else if a == i as i64 { w[1] } else { w[0] })
}

fn gain3(x: [f64; 3], r: f64) -> [f64; 3] {
    let k = (1.0 - r).ln() / 0.5f64.ln();
    let res: [f64; 3] = std::array::from_fn(|i| {
        let s = if x[i] >= 0.5 { 2.0 } else { 0.0 };
        let m = 2.0 * (1.0 - s);
        0.5 * s + 0.25 * m * (s + x[i] * m).max(0.0).powf(k)
    });
    let sum = res[0] + res[1] + res[2];
    res.map(|v| v / sum)
}

fn render(f: &mut Frame) -> FxResult {
    let size = f.f(0).max(1.0) / 100.0;
    let tile = f.f(1).max(1.0) / 100.0;
    let c = f.point(2);
    let strength = f.f(3) / 100.0;
    let r = f.f(4).clamp(50.0, 100.0) / 100.0;
    let show = f.b(5);
    let tex = if f.has_layer(6) { f.layer_image(6).unwrap_or_else(|| f.input()) } else { f.input() };
    let (w, h) = (f.width as f64, f.height as f64);
    let m = w.min(h);
    let sample = |u: f64, v: f64| tex.sample_wrapped((u * tex.w as f64) as f32, (v * tex.h as f64) as f32);
    let mut out = Image::new(f.width, f.height);
    out.map(|x, y, _| {
        let st = (((x as f64 + 0.5) - c[0]) / m / size, ((y as f64 + 0.5) - c[1]) / m / size);
        let (bw, vs) = triangle_grid(st, tile);
        let mut cs = [[0.0f32; 4]; 3];
        for i in 0..3 {
            let (co, si) = rot(vs[i], strength);
            let cen = center(vs[i]);
            let d = (st.0 - cen.0, st.1 - cen.1);
            let hh = hash(vs[i]);
            // mul(行ベクトル, 回転行列)
            let p = (d.0 * co + d.1 * si + cen.0 + hh.0, -d.0 * si + d.1 * co + cen.1 + hh.1);
            cs[i] = sample(p.0, p.1);
        }
        let lum = |c: [f32; 4]| -> f64 {
            let d = color::to_display(c);
            ((d[0] * 0.299 + d[1] * 0.587 + d[2] * 0.114) * d[3]) as f64
        };
        let mut wt: [f64; 3] = std::array::from_fn(|i| (1.0 + (lum(cs[i]) - 1.0) * 0.6) * bw[i].powi(7));
        let sum = (wt[0] + wt[1] + wt[2]).max(1e-12);
        wt = wt.map(|v| v / sum);
        if r != 0.5 {
            wt = gain3(wt, r);
        }
        if show {
            let hw = hex_weights(wt, vs[0], vs[2]);
            return color::from_display([hw[0] as f32, hw[1] as f32, hw[2] as f32, 1.0]);
        }
        std::array::from_fn(|k| (wt[0] * cs[0][k] as f64 + wt[1] * cs[1][k] as f64 + wt[2] * cs[2][k] as f64) as f32)
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "azurite（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "azurite.Hex-Tiling", name: "Hex-Tiling", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ", 100.0, 1.0, 4000.0),
                float(c"tile_size", c"タイルサイズ", 100.0, 50.0, 200.0),
                point(c"center", c"中心", 0.5, 0.5),
                float(c"rot_strength", c"回転強度", 0.0, -100.0, 100.0),
                float(c"r", c"コントラスト補正", 75.0, 50.0, 100.0),
                boolean(c"show_weights", c"重み表示", false),
                layer(c"texture", c"テクスチャのレイヤー"),
            ],
            render: render,
        },
    ]
}
