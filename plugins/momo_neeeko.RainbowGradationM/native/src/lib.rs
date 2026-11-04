//! 虹色グラデーションM（もも猫）の移植: 色相が巡るグラデーションを、合成モードを選んで重ねる。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

fn fract(v: f64) -> f64 {
    v - v.floor()
}
fn hue2rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 0.5 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}
fn hsl2rgb(h: f64, s: f64, l: f64) -> [f64; 3] {
    if s < 1e-6 {
        return [l, l, l];
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    [hue2rgb(p, q, h + 1.0 / 3.0), hue2rgb(p, q, h), hue2rgb(p, q, h - 1.0 / 3.0)]
}
/// 元の「HSV(補正)」: 色相の進み方を、黄・水色・紫付近で詰めて見た目の幅をそろえる。
fn hsl2_to_hsl(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let hh = h * 360.0;
    let lp = ((l * 2.0 - 1.0).powi(5) + 1.0) / 2.0 * 100.0;
    let kaku = lp / 100.0 * 44.0;
    let m120 = ((hh + 60.0) % 120.0 + 120.0) % 120.0;
    if (m120 - 60.0).abs() > 60.0 - kaku {
        let kijun = (((hh + 60.0 - kaku) / 120.0).floor() * 2.0 + 1.0) * 60.0;
        let k = if kaku.abs() < 1e-4 { 1e-4 } else { kaku };
        (fract((kijun + (hh - kijun) / k * 10.0) / 360.0), s, l)
    } else {
        let kijun = (((hh + 60.0 - kaku) / 120.0).floor() * 2.0) * 60.0;
        let d = if (60.0 - kaku).abs() < 1e-4 { 1e-4 } else { 60.0 - kaku };
        (fract((kijun + (hh - kijun) / d * 50.0) / 360.0), s, l)
    }
}
/// OKLCH → リニア RGB。
fn oklch(h: f64, c: f64, l: f64) -> [f64; 3] {
    let an = h * 2.0 * PI;
    let (a, b) = (c * an.cos(), c * an.sin());
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
    let (l3, m3, s3) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
    [
        4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3,
        -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3,
        -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3,
    ]
}
fn rgb2hsl(c: [f32; 3]) -> (f64, f64, f64) {
    let (h, s, l) = color::rgb_to_hsl(c[0], c[1], c[2]);
    (h as f64, s as f64, l as f64)
}
fn lin2oklch(c: [f32; 4]) -> (f64, f64, f64) {
    let (r, g, b) = (c[0] as f64, c[1] as f64, c[2] as f64);
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).max(0.0).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).max(0.0).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).max(0.0).cbrt();
    let ll = 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s;
    let a = 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s;
    let bb = -0.6616965574 * l + 1.6913997184 * m - 1.0297031610 * s;
    let ch = a.hypot(bb);
    let h = if ch > 1e-5 { (bb.atan2(a) / (2.0 * PI)).rem_euclid(1.0) } else { 0.0 };
    (h, ch, ll)
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let alpha = (f.f(0) / 100.0) as f32;
    let (cx, cy) = (f.f(1) * sc, f.f(2) * sc);
    let dir = f.f(3) / -180.0 * PI;
    let range = f.f(4) * sc;
    let blend = f.i(5);
    let kind = f.i(6);
    let mut lp = f.b(7);
    let flip = f.b(8);
    let cs = f.i(9);
    let use_value = f.b(10);
    let (mut h, mut s, l) = if use_value {
        if cs == 2 {
            (f.f(12) / 360.0, f.f(14) / 100.0, f.f(16) / 100.0)
        } else {
            (f.f(11) / 360.0, f.f(13) / 100.0, f.f(15) / 100.0)
        }
    } else {
        let c = f.color(17);
        let (h, s, l) = if cs == 2 { lin2oklch(c) } else { rgb2hsl([0, 1, 2].map(|i| color::linear_to_srgb(c[i]))) };
        ((h + f.f(18) / 360.0).rem_euclid(1.0), s, l)
    };
    h = h.rem_euclid(1.0);
    if range == 0.0 && lp {
        lp = false;
        s = 0.0;
    }
    let (w, hh) = (f.width, f.height);
    let (rs, rc) = dir.sin_cos();
    let org = f.input();
    let grad = Image::generate(w, hh, |x, y| {
        let n = [x as f64 + 0.5 - w as f64 / 2.0 - cx, y as f64 + 0.5 - hh as f64 / 2.0 - cy];
        let px = n[0] * rc - n[1] * rs;
        let py = n[0] * rs + n[1] * rc;
        let mut nx = match kind {
            1 => px.hypot(py),
            2 => px.abs() + py.abs(),
            3 => (-py).atan2(-px) / (2.0 * PI) * 360.0 + range / 2.0,
            _ => px + range / 2.0,
        };
        if flip {
            nx = -nx;
        }
        let t = if range == 0.0 { 0.0 } else { nx / range };
        let hue = fract(if lp { t } else { t.clamp(0.0, 1.0) } + h);
        match cs {
            0 => { let c = hsl2rgb(hue, s, l); color::from_display([c[0] as f32, c[1] as f32, c[2] as f32, 1.0]) }
            1 => { let (a, b, c) = hsl2_to_hsl(hue, s, l); let c = hsl2rgb(a, b, c); color::from_display([c[0] as f32, c[1] as f32, c[2] as f32, 1.0]) }
            _ => {
                // 元と同じく sRGB の範囲に切り詰めてからリニアに戻す
                let c = oklch(hue, s, l).map(|v| color::linear_to_srgb(v.max(0.0) as f32).clamp(0.0, 1.0));
                color::from_display([c[0], c[1], c[2], 1.0])
            }
        }
    });
    // 元の絵の上にグラデーションを重ね、アルファは元のまま
    let mut out = Image::new(w, hh);
    let mut opaque = Image::new(w, hh);
    for (o, p) in opaque.data.chunks_exact_mut(4).zip(org.data.chunks_exact(4)) {
        if p[3] > 0.0 {
            o.copy_from_slice(&[p[0] / p[3], p[1] / p[3], p[2] / p[3], 1.0]);
        }
    }
    aul::blend(&mut opaque, &grad, blend, alpha);
    for ((o, q), p) in out.data.chunks_exact_mut(4).zip(opaque.data.chunks_exact(4)).zip(org.data.chunks_exact(4)) {
        let a = p[3];
        o.copy_from_slice(&[q[0] * a, q[1] * a, q[2] * a, a]);
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "もも猫（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "momo_neeeko.RainbowGradationM", name: "虹色グラデーションM", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"alpha", c"強さ", 100.0, 0.0, 100.0),
                float(c"cx", c"中心X", 0.0, -10000.0, 10000.0),
                float(c"cy", c"中心Y", 0.0, -10000.0, 10000.0),
                float(c"dir", c"角度", 0.0, -36000.0, 36000.0),
                float(c"range", c"幅", 100.0, 0.0, 10000.0),
                choice(c"blend", c"合成モード", 0, c"通常|加算|減算|乗算|スクリーン|オーバーレイ|比較(明)|比較(暗)|輝度|色差|陰影|明暗|差分"),
                choice(c"type", c"タイプ", 0, c"線形|円形|矩形|環状"),
                boolean(c"loop", c"繰り返す", true),
                boolean(c"flip", c"反転", false),
                choice(c"cs", c"色補間", 0, c"HSV|HSV(補正)|OKLAB"),
                boolean(c"use_value", c"数値で基準色を指定", false),
                float(c"hue_hsl", c"基準色相", 0.0, -36000.0, 36000.0),
                float(c"hue_oklab", c"基準色相(OKLAB)", 29.23, -36000.0, 36000.0),
                float(c"saturation", c"彩度", 100.0, 0.0, 100.0),
                float(c"chroma", c"彩度(OKLAB)", 25.77, 0.0, 40.0),
                float(c"lightness_hsl", c"明度", 50.0, 0.0, 100.0),
                float(c"lightness_oklab", c"明度(OKLAB)", 62.8, 0.0, 100.0),
                color_hex(c"col", c"基準色", 0xff0000),
                float(c"shift", c"シフト", 0.0, -36000.0, 36000.0),
            ],
            render: render,
        },
    ]
}
