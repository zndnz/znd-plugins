//! Glitch_T（sagami121）の移植: スライスのずれ・RGB のずれ・縦ずれ・色帯・砂嵐・走査線を重ねるグリッチ。元のシェーダーをそのまま移した。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

fn fract(v: f64) -> f64 {
    v - v.floor()
}
fn hash21(x: f64, y: f64) -> f64 {
    let (mut a, mut b, mut c) = (fract(x * 0.1031), fract(y * 0.1030), fract(x * 0.0973));
    let d = a * (b + 33.33) + b * (c + 33.33) + c * (a + 33.33);
    a += d;
    b += d;
    c += d;
    fract((a + b) * c)
}
fn hash11(p: f64) -> f64 {
    let mut p = fract(p * 0.1031);
    p *= p + 33.33;
    fract(p * (p + p))
}
fn step(e: f64, x: f64) -> f64 {
    if x >= e {
        1.0
    } else {
        0.0
    }
}
fn overlay(b: f64, l: f64) -> f64 {
    if b >= 0.5 {
        1.0 - 2.0 * (1.0 - b) * (1.0 - l)
    } else {
        2.0 * b * l
    }
}
fn blend(base: [f64; 3], col: [f64; 3], amt: f64, mode: usize) -> [f64; 3] {
    let mut o = base;
    for k in 0..3 {
        let (b, l) = (base[k], col[k]);
        o[k] = match mode {
            0 => b + (l - b) * amt,
            1 => (b + l * amt).clamp(0.0, 1.0),
            2 => b + (b * l - b) * amt,
            3 => b + (overlay(b, l) - b) * amt,
            _ => b + ((b - l).abs() - b) * amt,
        };
    }
    o
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let (w, h) = (f.width as f64, f.height as f64);
    let raw = (f.f(0) / 100.0).clamp(0.0, 1.0);
    let slice = (f.f(1) * s / h).max(1e-6);
    let rgb_n = f.f(2) * s / w;
    let speed = f.f(3) / 30.0;
    let v_n = f.f(4) * s / h;
    let mode = f.i(5);
    let th = f.f(6) / 100.0;
    let noise_a = f.f(7) / 100.0;
    let grain = (f.f(8) * s).max(1.0);
    let dens = (f.f(9) / 100.0 / s).max(0.0001);
    let contrast = f.f(10) / 100.0;
    let frame = f.frame() as f64;
    let seed = (frame * speed * 1.5).floor() + frame * speed * 0.05 + f.f(11) * 100.0;
    let it = (raw * (1.0 + (0.5 - th) * 1.5)).clamp(0.0, 1.0);
    let src = f.input();
    // 元のシェーダーはプリマルチプライドの表示値で計算する
    let disp = |p: [f32; 4]| -> [f64; 4] {
        let d = to_display(p);
        [(d[0] * d[3]) as f64, (d[1] * d[3]) as f64, (d[2] * d[3]) as f64, d[3] as f64]
    };
    let sample = |u: f64, v: f64| disp(src.sample_clamped((u * w) as f32, (v * h) as f32));
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
        let (u, v) = (px / w, py / h);
        let row = (v / slice).floor();
        let big = step(th * 0.8, hash21(row * 0.9, seed + 1.1) * it);
        let dx = (hash21(row * 0.4, seed) - 0.5) * 2.0 * rgb_n * 4.0 * big
            + (hash21(row * 3.3, seed + 2.2) - 0.5) * rgb_n * 0.4 * it
            + (v * 80.0 + seed * 25.0).sin() * rgb_n * 0.3 * it;
        let sv_on = step(th * 0.7, hash21(row * 2.5, seed + 3.7) * it);
        let slice_dy = (hash21(row * 4.1, seed + 4.9) - 0.5) * 3.0 * v_n * sv_on;
        let roll = seed * 0.08 * (hash11((seed * 0.15).floor()) - 0.5) * 2.0 * v_n * it;
        let v_on = step(th * 0.85, hash11(seed + 6.6) * it);
        let dy = (hash11(seed + 5.5) - 0.5) * 4.0 * v_n * v_on + (hash11(seed + 7.7) - 0.5) * v_n * 0.8 * it + slice_dy + roll;
        let shift = rgb_n * it;
        let vv = fract(v + dy);
        let r = sample(fract(u + dx + shift * 0.8), vv)[0];
        let g4 = sample(fract(u + dx), vv);
        let b = sample(fract(u + dx - shift * 0.8), vv)[2];
        let a = g4[3];
        let mut col = [r, g4[1], b];
        let band = (v * 12.0).floor();
        let band_on = step(1.0 - it * 0.45, hash21(band, seed * 19.3));
        let bc = [hash21(band * 1.3, seed * 11.1), hash21(band * 2.7, seed * 22.2), hash21(band * 4.1, seed * 33.3)].map(|c| (c * 2.0).clamp(0.0, 1.0));
        col = blend(col, bc, band_on * 0.65 * it * noise_a, mode);
        let gh = hash21((px / grain).floor() + seed * 17.3, (py / grain).floor() + seed * 31.7);
        let sg = gh + (step(1.0 - contrast, gh) - gh) * contrast;
        col = blend(col, [sg; 3], it * noise_a, mode);
        let ry = (py * dens).floor();
        let line_on = step(th * 0.8, hash21(ry, seed * 43.1) * it);
        let lb = hash21(ry * 3.3, seed * 91.7);
        col = blend(col, [lb; 3], line_on * 0.5 * noise_a, mode);
        if a <= 1e-6 {
            return [0.0; 4];
        }
        let st = [(col[0] / a) as f32, (col[1] / a) as f32, (col[2] / a) as f32];
        from_display([st[0].clamp(0.0, 1.0), st[1].clamp(0.0, 1.0), st[2].clamp(0.0, 1.0), a as f32])
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "sagami121（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "sagami121.Glitch_T", name: "Glitch_T", category: "スタイライズ", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"intensity", c"強さ", 50.0, 0.0, 100.0),
                float(c"slice_h", c"スライス高さ", 12.0, 1.0, 60.0),
                float(c"rgb_amt", c"RGBずれ", 30.0, 0.0, 150.0),
                float(c"speed", c"速度", 5.0, 0.0, 30.0),
                float(c"v_shift", c"縦ずれ量", 120.0, 0.0, 500.0),
                choice(c"blend_mode", c"合成モード", 0, c"通常|加算|乗算|オーバーレイ|差分"),
                float(c"threshold", c"しきい値", 50.0, 0.0, 100.0),
                float(c"noise_alpha", c"ノイズ不透明度", 80.0, 0.0, 100.0),
                float(c"grain_size", c"砂嵐粒子サイズ", 2.0, 1.0, 20.0),
                float(c"line_density", c"走査線密度", 20.0, 1.0, 100.0),
                float(c"noise_contrast", c"ノイズコントラスト", 60.0, 0.0, 100.0),
                float(c"seed_offset", c"シード値", 0.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}
