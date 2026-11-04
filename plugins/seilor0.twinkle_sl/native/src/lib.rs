//! twinkle_sl（seilor0）の移植: ✨ の形（直線・カーブ）を描く。元のシェーダーの距離関数をそのまま移した。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

type V = (f64, f64);

fn sgn(v: f64) -> f64 {
    if v > 0.0 {
        1.0
    } else if v < 0.0 {
        -1.0
    } else {
        0.0
    }
}
fn len(v: V) -> f64 {
    v.0.hypot(v.1)
}
fn sd_ray(p: V, a: V, angle: f64) -> f64 {
    let d = (angle.cos(), angle.sin());
    let ap = (p.0 - a.0, p.1 - a.1);
    let t = (d.0 * ap.0 + d.1 * ap.1).max(0.0);
    sgn(d.0 * (d.0 * ap.1 - d.1 * ap.0)) * len((ap.0 - d.0 * t, ap.1 - d.1 * t))
}
fn sd_line(p: V, a: V, b: V) -> f64 {
    let ab = (b.0 - a.0, b.1 - a.1);
    let ap = (p.0 - a.0, p.1 - a.1);
    let t = (ab.0 * ap.0 + ab.1 * ap.1) / (ab.0 * ab.0 + ab.1 * ab.1);
    sgn(ab.0 * (ab.0 * ap.1 - ab.1 * ap.0)) * len((ap.0 - ab.0 * t, ap.1 - ab.1 * t))
}

fn sd_twinkle(p: V, hs: V, a: f64, c_ratio: f64, corner: usize) -> f64 {
    let p = (p.0.abs(), p.1.abs());
    if corner == 0 && c_ratio == 0.0 {
        return sd_line(p, (a, a), (hs.0, 0.0)).min(sd_line(p, (a, a), (0.0, hs.1)));
    }
    let ang = (a.atan2(hs.0 - a), a.atan2(hs.1 - a));
    let radius = (c_ratio * a / ang.0.cos(), c_ratio * a / ang.1.cos());
    let delta = (c_ratio * 2.0 * a / (2.0 * ang.0).sin(), c_ratio * 2.0 * a / (2.0 * ang.1).sin());
    let vx = (hs.0 - a, -a);
    let vy = (-a, hs.1 - a);
    let ar = a * (hs.0 + hs.1) / (hs.0 * hs.1);
    let bis = if 1.0 - ar < 1e-6 { (hs.1, hs.0) } else { (vx.0 / len(vx) + vy.0 / len(vy), vx.1 / len(vx) + vy.1 / len(vy)) };
    let s1 = sgn(bis.0 * (p.1 - a) - bis.1 * (p.0 - a));
    if s1 < 0.0 {
        sd_ray(p, (hs.0 - delta.0, 0.0), PI - ang.0) - radius.0
    } else {
        sd_ray(p, (0.0, hs.1 - delta.1), ang.1 - 0.5 * PI) - radius.1
    }
}

fn bisearch(p: V, hs: V, n: f64, mut xa: f64, mut xb: f64) -> f64 {
    for _ in 0..6 {
        let x = 0.5 * (xa + xb);
        let (c, s) = (x.cos(), x.sin());
        let cn = hs.0 * c.powf(n);
        let sn = hs.1 * s.powf(n);
        let y = (p.0 - cn) * cn * s * s - (p.1 - sn) * sn * c * c;
        if y < 0.0 {
            xa = x;
        } else {
            xb = x;
        }
    }
    let q = |t: f64| (hs.0 * t.cos().abs().powf(n), hs.1 * t.sin().abs().powf(n));
    let (qa, qb) = (q(xa), q(xb));
    let (pa, ba) = ((p.0 - qa.0, p.1 - qa.1), (qb.0 - qa.0, qb.1 - qa.1));
    let hh = ((pa.0 * ba.0 + pa.1 * ba.1) / (ba.0 * ba.0 + ba.1 * ba.1)).clamp(0.0, 1.0);
    len((pa.0 - ba.0 * hh, pa.1 - ba.1 * hh)) * sgn(pa.0 * ba.1 - pa.1 * ba.0)
}

fn sd_squircle(p: V, hs: V, n: f64) -> f64 {
    let p = (p.0.abs(), p.1.abs());
    let n = 2.0 / n;
    let d1 = bisearch(p, hs, n, 0.0, 0.25 * PI);
    let d2 = bisearch(p, hs, n, 0.25 * PI, 0.5 * PI);
    if d1.abs() < d2.abs() {
        d1
    } else {
        d2
    }
}

fn paint(f: &mut Frame, hx: f64, hy: f64, fill: [f32; 4], line: [f32; 4], sd: impl Fn(V) -> (f32, f32)) {
    let (w, h) = ((hx * 2.0).ceil().max(1.0) as usize, (hy * 2.0).ceil().max(1.0) as usize);
    let mut img = Image::new(w, h);
    img.map(|x, y, _| {
        let (a1, a2) = sd((x as f64 + 0.5 - hx, y as f64 + 0.5 - hy));
        let (s, d) = (line.map(|v| v * a2), fill.map(|v| v * a1));
        let mut o = [0.0; 4];
        for k in 0..4 {
            o[k] = s[k] + d[k] * (1.0 - s[3]);
        }
        o
    });
    let ox = (f.width as i32 - w as i32) / 2;
    let oy = (f.height as i32 - h as i32) / 2;
    f.output(img, ox, oy);
}

fn line_star(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let size = f.f(0) * s;
    let sy = size / 2.0;
    let sx = sy * f.f(1) / 100.0;
    let a = sy * f.f(1) / (100.0 + f.f(1)) * (100.0 - f.f(2)) / 100.0;
    if sx < 1.0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let c_ratio = f.f(3) / 100.0;
    let fill = color::premul(f.color(4), (f.f(5) / 100.0) as f32);
    let line_w = if f.b(7) { f.f(6) * s } else { size * f.f(6) / 200.0 };
    let pos = f.i(8) as f64;
    let corner = f.i(9);
    let line = color::premul(f.color(10), (f.f(11) / 100.0) as f32);
    let (mut ix, mut iy) = (sx, sy);
    if corner == 0 {
        let (rx, ry) = (a.atan2(sx - a), a.atan2(sy - a));
        ix += line_w / rx.sin() * 0.5 * pos;
        iy += line_w / ry.sin() * 0.5 * pos;
    } else {
        ix += line_w * 0.5 * pos;
        iy += line_w * 0.5 * pos;
    }
    let off = (pos - 1.0) * line_w / 2.0;
    paint(f, ix, iy, fill, line, |p| {
        let d = sd_twinkle(p, (sx, sy), a, c_ratio, corner);
        let a1 = ((if pos == 0.0 && line_w > 1.0 { 0.0 } else { 0.5 }) - d).clamp(0.0, 1.0) as f32;
        let a2 = if line_w > 0.0 { ((line_w + 1.0) / 2.0 - (d - off).abs()).clamp(0.0, 1.0) as f32 } else { 0.0 };
        (a1, a2)
    });
    Ok(())
}

fn curve_star(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let size = f.f(0) * s;
    let n0 = -3.0 * f.f(2) / 100.0;
    let n = (3.0 + n0) / (3.0 - n0);
    if size < 2.0 || n < 0.002 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let fill = color::premul(f.color(3), (f.f(4) / 100.0) as f32);
    let line_w = if f.b(6) { f.f(5) * s } else { size * f.f(5) / 200.0 };
    let pos = f.i(7) as f64;
    let remain = if f.b(8) { 1.0 } else { 0.0 };
    let line = color::premul(f.color(9), (f.f(10) / 100.0) as f32);
    let sy = size * 0.5;
    let sx = sy * f.f(1) / 100.0;
    let off_im = line_w * 0.5 * pos;
    let off = ((pos - 1.0) * line_w + remain) / 2.0;
    paint(f, sx + off_im, sy + off_im, fill, line, |p| {
        let d = sd_squircle(p, (sx, sy), n);
        let a1 = ((remain + if pos == 0.0 && line_w > 0.0 { 0.0 } else { 1.0 }) / 2.0 - d).clamp(0.0, 1.0) as f32;
        let a2 = if line_w > 0.0 { ((line_w + 1.0) / 2.0 - (d - off).abs()).clamp(0.0, 1.0) as f32 } else { 0.0 };
        (a1, a2)
    });
    Ok(())
}

plugin! {
    vendor: "seilor0（znd 移植: znd-plugins）", version: "1.2.1", license: "MIT",
    effects: [
        {
            id: "seilor0.twinkle_sl.line", name: "twinkle_sl（直線）", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ", 500.0, 0.0, 4000.0),
                float(c"x_ratio", c"幅（%）", 70.0, 0.0, 100.0),
                float(c"depth", c"深さ（%）", 75.0, 0.0, 100.0),
                float(c"c_ratio", c"角丸（%）", 0.0, 0.0, 100.0),
                color_hex(c"fill_color", c"塗り", 0xffffff),
                float(c"fill_alpha", c"塗りα（%）", 100.0, 0.0, 100.0),
                float(c"line_w", c"ライン幅（%）", 0.0, 0.0, 200.0),
                boolean(c"line_abs", c"ライン幅を絶対値（px）にする", false),
                choice(c"line_pos", c"ライン位置", 1, c"内側|中央|外側"),
                choice(c"corner_type", c"角の形状", 1, c"マイター|ラウンド"),
                color_hex(c"line_color", c"ライン", 0xff0000),
                float(c"line_alpha", c"ラインα（%）", 100.0, 0.0, 100.0),
            ],
            render: line_star,
        },
        {
            id: "seilor0.twinkle_sl.curve", name: "twinkle_sl（カーブ）", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ", 500.0, 0.0, 4000.0),
                float(c"x_ratio", c"幅（%）", 70.0, 0.0, 100.0),
                float(c"depth", c"深さ（%）", 50.0, 0.0, 100.0),
                color_hex(c"fill_color", c"塗り", 0xffffff),
                float(c"fill_alpha", c"塗りα（%）", 100.0, 0.0, 100.0),
                float(c"line_w", c"ライン幅（%）", 0.0, 0.0, 200.0),
                boolean(c"line_abs", c"ライン幅を絶対値（px）にする", false),
                choice(c"line_pos", c"ライン位置", 1, c"内側|中央|外側"),
                boolean(c"remain_line", c"ラインを最低 1px 残す", true),
                color_hex(c"line_color", c"ライン", 0xff0000),
                float(c"line_alpha", c"ラインα（%）", 100.0, 0.0, 100.0),
            ],
            render: curve_star,
        },
    ]
}
