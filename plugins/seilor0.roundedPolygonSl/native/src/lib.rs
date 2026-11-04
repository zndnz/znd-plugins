//! 角丸正多角形_sl（seilor0）の移植: 角丸・星形にできる正多角形。元のピクセルシェーダーの距離関数をそのまま移した。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

struct P {
    n: f64,
    radius: f64,
    depth: f64,
    c_ratio: f64,
    corner_type: usize,
    c_is_top: bool,
}

fn rot(a: f64, p: (f64, f64)) -> (f64, f64) {
    // float2x2(cos, -sin, sin, cos) と p の積
    let (s, c) = a.sin_cos();
    (c * p.0 - s * p.1, s * p.0 + c * p.1)
}

fn sgn(v: f64) -> f64 {
    if v > 0.0 {
        1.0
    } else if v < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn sd_dir(p: (f64, f64), a: (f64, f64), angle: f64, ray: bool) -> f64 {
    let d = (angle.cos(), angle.sin());
    let ap = (p.0 - a.0, p.1 - a.1);
    let mut t = d.0 * ap.0 + d.1 * ap.1;
    if ray {
        t = t.max(0.0);
    }
    let s1 = sgn(d.0 * ap.1 - d.1 * ap.0);
    let s2 = sgn(if d.0 == 0.0 { d.1 } else { d.0 });
    s1 * s2 * (ap.0 - d.0 * t).hypot(ap.1 - d.1 * t)
}

fn sd_star(p: (f64, f64), q: &P) -> f64 {
    let half = PI / q.n;
    let notch = (q.depth / half.tan()).atan();
    let half_arm = 0.5 * PI - half - notch;
    let c_radius = q.c_ratio * q.radius * half.sin() / notch.cos() * half_arm.tan();
    let radius = q.radius - c_radius / half_arm.sin();
    if radius < 1e-6 {
        return p.0.hypot(p.1) - c_radius;
    }
    let mut p = (p.1, p.0);
    let a1 = half * ((q.n + 1.0) % 2.0) * if q.c_is_top { 1.0 } else { 0.0 };
    p = rot(-a1, p);
    let angle = p.1.atan2(p.0);
    let r = 2.0 * half * (angle * 0.5 / half + 0.5).floor();
    p = rot(-r, p);
    p.1 = p.1.abs();
    let pt1 = (half.cos() * radius, half.sin() * radius);
    let rad = (-pt1.1).atan2(-pt1.0 * q.depth);
    let ray = !(q.corner_type == 0 && q.c_ratio == 0.0);
    -sd_dir(p, pt1, rad, ray) - c_radius
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let size = f.f(0) * s;
    let n = f.f(1).round().clamp(3.0, 40.0);
    let depth = if f.b(4) && n > 4.0 { (PI / n).tan().powi(2) } else { f.f(3) / 100.0 };
    if size < 2.0 || depth >= 1.0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let fill_c = color::premul(f.color(6), (f.f(7) / 100.0) as f32);
    let line_w = f.f(8) * s;
    let line_pos = f.i(9) as f64;
    let corner_type = f.i(10);
    let line_c = color::premul(f.color(11), (f.f(12) / 100.0) as f32);
    let offset = if corner_type == 0 {
        let angle = PI * (0.5 - 1.0 / n) - (depth / (PI / n).tan()).atan();
        0.5 * line_pos * line_w / angle.sin()
    } else {
        0.5 * line_pos * line_w
    };
    let side = (size + offset * 2.0).ceil().max(1.0) as usize;
    let half_im = size * 0.5 + offset;
    let q = P { n, radius: size * 0.5, depth, c_ratio: f.f(2) / 100.0, corner_type, c_is_top: f.b(5) };
    let mut img = Image::new(side, side);
    let line_off = (line_pos - 1.0) * line_w * 0.5;
    img.map(|x, y, _| {
        let d = sd_star((x as f64 + 0.5 - half_im, y as f64 + 0.5 - half_im), &q);
        let a1 = ((if line_pos == 0.0 && line_w > 0.0 { 0.0 } else { 0.5 }) - d).clamp(0.0, 1.0) as f32;
        let a2 = if line_w > 0.0 { ((line_w + 1.0) / 2.0 - (d - line_off).abs()).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let (src, dst) = (line_c.map(|v| v * a2), fill_c.map(|v| v * a1));
        let mut o = [0.0; 4];
        for k in 0..4 {
            o[k] = src[k] + dst[k] * (1.0 - src[3]);
        }
        o
    });
    let ox = (f.width as i32 - side as i32) / 2;
    let oy = (f.height as i32 - side as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "seilor0（znd 移植: znd-plugins）", version: "1.2.1", license: "MIT",
    effects: [
        {
            id: "seilor0.roundedPolygonSl", name: "角丸正多角形_sl", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ", 400.0, 0.0, 4000.0),
                float(c"n", c"頂点数", 3.0, 3.0, 40.0),
                float(c"c_ratio", c"角丸（%）", 0.0, 0.0, 100.0),
                float(c"depth", c"星形（%）", 0.0, 0.0, 100.0),
                boolean(c"is_star", c"綺麗な星形にする", false),
                boolean(c"c_is_top", c"常に頂点を真上にする", true),
                color_hex(c"fill_color", c"塗り", 0xffffff),
                float(c"fill_alpha", c"塗りα", 100.0, 0.0, 100.0),
                float(c"line_w", c"ライン幅", 0.0, 0.0, 2000.0),
                choice(c"line_pos", c"ライン位置", 0, c"内側|中央|外側"),
                choice(c"corner_type", c"角の形状", 1, c"マイター|ラウンド"),
                color_hex(c"line_color", c"ライン", 0xff0000),
                float(c"line_alpha", c"ラインα", 100.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}
