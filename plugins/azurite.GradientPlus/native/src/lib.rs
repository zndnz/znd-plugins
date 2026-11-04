//! グラデーション+（azurite）の移植: sRGB 以外の色空間（リニア・HSV・HSL・Lab・LCh・Oklab・Oklch）で補間するグラデーション。
//! 色の変換と形の式は元のシェーダーと同じ。

use std::f64::consts::{PI, TAU};
use znd_plugin_sdk::*;

type C = [f64; 3];
const CHROMA_T: f64 = 0.02;
const D50: C = [0.96422, 1.0, 0.82521];

fn modf(x: f64, y: f64) -> f64 {
    x - y * (x / y).floor()
}
fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
fn dec(x: f64) -> f64 {
    if x <= 0.04045 { x / 12.92 } else { ((x + 0.055) / 1.055).abs().powf(2.4) }
}
fn enc(x: f64) -> f64 {
    if x <= 0.0031308 { 12.92 * x } else { 1.055 * x.abs().powf(1.0 / 2.4) - 0.055 }
}
fn s2l(c: C) -> C {
    c.map(dec)
}
fn l2s(c: C) -> C {
    c.map(|v| enc(v.clamp(0.0, 1.0)))
}
fn hue_of(c: C, mx: f64, chroma: f64) -> f64 {
    let h = if chroma == 0.0 {
        0.0
    } else if mx == c[0] {
        modf((c[1] - c[2]) / chroma, 6.0)
    } else if mx == c[1] {
        (c[2] - c[0]) / chroma + 2.0
    } else {
        (c[0] - c[1]) / chroma + 4.0
    };
    (h * 60.0).to_radians()
}
fn rgb2hsv(c: C) -> C {
    let mx = c[0].max(c[1]).max(c[2]);
    let mn = c[0].min(c[1]).min(c[2]);
    let ch = mx - mn;
    [hue_of(c, mx, ch), if mx == 0.0 { 0.0 } else { ch / mx }, mx]
}
fn hsv2rgb(h: C) -> C {
    let f = |n: f64| {
        let k = modf(n + h[0].to_degrees() / 60.0, 6.0);
        h[2] - h[2] * h[1] * k.min(4.0 - k).min(1.0).max(0.0)
    };
    [f(5.0), f(3.0), f(1.0)]
}
fn rgb2hsl(c: C) -> C {
    let mx = c[0].max(c[1]).max(c[2]);
    let mn = c[0].min(c[1]).min(c[2]);
    let ch = mx - mn;
    let l = (mx + mn) / 2.0;
    let s = if l != 1.0 && l != 0.0 { ch / (1.0 - (2.0 * l - 1.0).abs()) } else { 0.0 };
    [hue_of(c, mx, ch), s, l]
}
fn hsl2rgb(h: C) -> C {
    let f = |n: f64| {
        let k = modf(n + h[0].to_degrees() / 30.0, 12.0);
        let a = h[1] * h[2].min(1.0 - h[2]);
        h[2] - a * (k - 3.0).min(9.0 - k).min(1.0).max(-1.0)
    };
    [f(0.0), f(8.0), f(4.0)]
}
fn lin2lab(c: C) -> C {
    let xyz = [
        0.4360747 * c[0] + 0.3850649 * c[1] + 0.1430804 * c[2],
        0.2225045 * c[0] + 0.7168786 * c[1] + 0.0606169 * c[2],
        0.0139322 * c[0] + 0.0971045 * c[1] + 0.7141733 * c[2],
    ];
    let f = |x: f64| if x > 0.008856 { x.abs().powf(1.0 / 3.0) } else { (903.3 * x + 16.0) / 116.0 };
    let s = [f(xyz[0] / D50[0]), f(xyz[1] / D50[1]), f(xyz[2] / D50[2])];
    [116.0 * s[1] - 16.0, 500.0 * (s[0] - s[1]), 200.0 * (s[1] - s[2])]
}
fn lab2lin(l: C) -> C {
    let fy = (l[0] + 16.0) / 116.0;
    let g = |x: f64| if x.abs().powi(3) > 0.008856 { x.abs().powi(3) } else { (116.0 * x - 16.0) / 903.3 };
    let y = if l[0] > 0.008856 * 903.3 { fy.abs().powi(3) } else { l[0] / 903.3 };
    let xyz = [D50[0] * g(fy + l[1] / 500.0), D50[1] * y, D50[2] * g(fy - l[2] / 200.0)];
    [
        xyz[0] * 3.1338561 + xyz[1] * -1.6168667 + xyz[2] * -0.4906146,
        xyz[0] * -0.9787684 + xyz[1] * 1.9161415 + xyz[2] * 0.0334540,
        xyz[0] * 0.0719453 + xyz[1] * -0.2289914 + xyz[2] * 1.4052427,
    ]
}
fn lab2lch(l: C) -> C {
    let ch = l[1].hypot(l[2]);
    let h = if ch > CHROMA_T { let h = l[2].atan2(l[1]); if h < 0.0 { h + TAU } else { h } } else { 0.0 };
    [l[0], ch, h]
}
fn lch2lab(l: C) -> C {
    [l[0], l[1] * l[2].cos(), l[1] * l[2].sin()]
}
fn lin2ok(c: C) -> C {
    let l = (0.4122214708 * c[0] + 0.5363325363 * c[1] + 0.0514459929 * c[2]).cbrt();
    let m = (0.2119034982 * c[0] + 0.6806995451 * c[1] + 0.1073969566 * c[2]).cbrt();
    let s = (0.0883024619 * c[0] + 0.2817188376 * c[1] + 0.6299787005 * c[2]).cbrt();
    [
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    ]
}
fn ok2lin(o: C) -> C {
    let l = (o[0] + 0.3963377774 * o[1] + 0.2158037573 * o[2]).powi(3);
    let m = (o[0] - 0.1055613458 * o[1] - 0.0638541728 * o[2]).powi(3);
    let s = (o[0] - 0.0894841775 * o[1] - 1.2914855480 * o[2]).powi(3);
    [
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    ]
}
fn hue_mix(mut h1: f64, mut h2: f64, v1: bool, v2: bool, t: f64, long: bool) -> f64 {
    if v1 && !v2 {
        h2 = h1;
    } else if !v1 && v2 {
        h1 = h2;
    } else if !v1 && !v2 {
        h1 = 0.0;
        h2 = 0.0;
    }
    let d = h2 - h1;
    if !long {
        if d > PI {
            h1 += TAU;
        } else if d < -PI {
            h2 += TAU;
        }
    } else if 0.0 < d && d < PI {
        h1 += TAU;
    } else if -PI < d && d <= 0.0 {
        h2 += TAU;
    }
    modf(lerp(h1, h2, t), TAU)
}

fn sd_square(u: [f64; 2]) -> f64 {
    let s = 2f64.sqrt();
    (u[0] * s).abs().max((u[1] * s).abs())
}
fn sd_rsq(u: [f64; 2], b: f64) -> f64 {
    let d = [u[0].abs() - b, u[1].abs() - b];
    d[0].max(0.0).hypot(d[1].max(0.0)) + d[0].max(d[1]).min(0.0)
}
/// HLSL の fmod（符号は被除数）。
fn fmod(x: f64, y: f64) -> f64 {
    x % y
}
fn shape(kind: usize, u: [f64; 2], w: f64) -> f64 {
    let len = u[0].hypot(u[1]);
    match kind {
        1 => smoothstep(-w * 0.5, w * 0.5, len - w * 0.5),
        2 => smoothstep(0.0, w, sd_square(u)),
        3 => smoothstep(0.0, w, u[1].abs()),
        4 => smoothstep(-w * 0.5, w * 0.5, sd_rsq(u, w * 0.5)),
        5 => { let w = w.max(1.0); let d = fmod(len - w + w, 2.0 * w) - w; smoothstep(w, 0.0, d.abs()) }
        6 => { let w = w.max(1.0); let d = fmod(sd_square(u) + w, 2.0 * w) - w; smoothstep(0.0, w, d.abs()) }
        7 => { let w = w.max(1.0); let d = (u[1] + w - 2.0 * w * ((u[1] + w) / (2.0 * w)).floor()) - w; smoothstep(0.0, w, d.abs()) }
        8 => { let w = w.max(1.0); let d = fmod(sd_rsq(u, w) + w, 2.0 * w) - w; smoothstep(0.0, w, d.abs()) }
        _ => { let w = w.max(1.0); smoothstep(-w * 0.5, w * 0.5, u[1]) }
    }
}

/// 2 色（表示値とアルファ）を色空間 cs で t の位置に混ぜた、表示値とアルファ。
fn mix(cs: usize, c1: C, a1: f64, c2: C, a2: f64, t: f64, long: bool) -> (C, f64) {
    let ma = lerp(a1, a2, t).clamp(0.0, 1.0).max(1e-6);
    let lerp3 = |x: C, y: C| [0, 1, 2].map(|i| lerp(x[i] * a1, y[i] * a2, t) / ma);
    let rgb = match cs {
        0 => lerp3(c1, c2),
        2 | 3 => {
            let (f, g): (fn(C) -> C, fn(C) -> C) = if cs == 2 { (rgb2hsv, hsv2rgb) } else { (rgb2hsl, hsl2rgb) };
            let (h1, h2) = (f(c1), f(c2));
            let hue = hue_mix(h1[0], h2[0], h1[1] > 0.0, h2[1] > 0.0, t, long);
            let s = lerp(h1[1] * a1, h2[1] * a2, t) / ma;
            let v = lerp(h1[2] * a1, h2[2] * a2, t) / ma;
            g([hue, s, v])
        }
        4 => l2s(lab2lin(lerp3(lin2lab(s2l(c1)), lin2lab(s2l(c2))))),
        5 | 7 => {
            let (h1, h2) = if cs == 5 { (lab2lch(lin2lab(s2l(c1))), lab2lch(lin2lab(s2l(c2)))) } else { (lab2lch(lin2ok(s2l(c1))), lab2lch(lin2ok(s2l(c2)))) };
            let hue = hue_mix(h1[2], h2[2], h1[1] > CHROMA_T, h2[1] > CHROMA_T, t, long);
            let lmax = if cs == 5 { 100.0 } else { 1.0 };
            let l = (lerp(h1[0] * a1, h2[0] * a2, t) / ma).clamp(0.0, lmax);
            let c = (lerp(h1[1] * a1, h2[1] * a2, t) / ma).max(0.0);
            let lab = lch2lab([l, c, hue]);
            l2s(if cs == 5 { lab2lin(lab) } else { ok2lin(lab) })
        }
        6 => l2s(ok2lin(lerp3(lin2ok(s2l(c1)), lin2ok(s2l(c2))))),
        _ => l2s([0, 1, 2].map(|i| lerp(dec(c1[i]) * a1, dec(c2[i]) * a2, t) / ma)),
    };
    (rgb, ma)
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let strength = (f.f(0) / 100.0) as f32;
    let center = [f.f(1) * sc, f.f(2) * sc];
    let kind = f.i(6);
    let mut angle = f.f(3);
    if matches!(kind, 2 | 4 | 6 | 8) {
        angle -= 45.0;
    }
    let gw = f.f(4) * sc;
    let blend = f.i(5);
    let cs = f.i(7);
    let long = f.i(8) == 1;
    let disp = |c: [f32; 4]| [0, 1, 2].map(|i| color::linear_to_srgb(c[i]) as f64);
    let (c1, c2) = (disp(f.color(9)), disp(f.color(10)));
    let (a1, a2) = (1.0 - f.f(11) / 100.0, 1.0 - f.f(12) / 100.0);
    let (w, h) = (f.width, f.height);
    let r = angle.to_radians();
    let (m00, m01, m10, m11) = (r.cos(), -r.sin(), r.sin(), r.cos());
    let grad = Image::generate(w, h, |x, y| {
        let st = [x as f64 + 0.5 - 0.5 * w as f64 - center[0], y as f64 + 0.5 - 0.5 * h as f64 - center[1]];
        let u = [m00 * st[0] + m01 * st[1], m10 * st[0] + m11 * st[1]];
        let t = shape(kind, u, gw);
        let (c, a) = mix(cs, c1, a1, c2, a2, t, long);
        color::from_display([c[0] as f32, c[1] as f32, c[2] as f32, a as f32])
    });
    // 元の絵を不透明にして重ね、アルファを元に戻す
    let org = f.input();
    let mut base = Image::new(w, h);
    for (o, p) in base.data.chunks_exact_mut(4).zip(org.data.chunks_exact(4)) {
        if p[3] > 0.0 {
            o.copy_from_slice(&[p[0] / p[3], p[1] / p[3], p[2] / p[3], 1.0]);
        }
    }
    aul::blend(&mut base, &grad, blend, strength);
    for (o, p) in f.rgba.chunks_exact_mut(4).zip(base.data.chunks_exact(4)) {
        let a = o[3];
        o.copy_from_slice(&[p[0] * a, p[1] * a, p[2] * a, a]);
    }
    Ok(())
}

plugin! {
    vendor: "azurite（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "azurite.GradientPlus", name: "グラデーション+", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"strength", c"強さ", 100.0, 0.0, 100.0),
                float(c"center_x", c"中心X", 0.0, -5000.0, 5000.0),
                float(c"center_y", c"中心Y", 0.0, -5000.0, 5000.0),
                float(c"angle", c"角度", 0.0, -3600.0, 3600.0),
                float(c"gradient_w", c"幅", 100.0, 0.0, 2000.0),
                choice(c"blend_mode", c"合成モード", 0, c"通常|加算|減算|乗算|スクリーン|オーバーレイ|比較(明)|比較(暗)|輝度|色差|陰影|明暗|差分"),
                choice(c"gradient_type", c"形状", 0, c"線形|円形|矩形|凸形|角丸矩形|円形ループ|矩形ループ|凸形ループ|角丸矩形ループ"),
                choice(c"color_space", c"色空間", 1, c"sRGB|Linear sRGB|HSV|HSL|L*a*b|LCh|Oklab|Oklch"),
                choice(c"interp_dir", c"補間経路", 0, c"短経路|長経路"),
                color_hex(c"color1", c"開始色", 0xffffff),
                color_hex(c"color2", c"終了色", 0x000000),
                float(c"alpha1", c"開始色透明度", 0.0, 0.0, 100.0),
                float(c"alpha2", c"終了色透明度", 0.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}
