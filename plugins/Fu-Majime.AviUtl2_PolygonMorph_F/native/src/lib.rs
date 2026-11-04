//! 多角形モーフ_F（Fu-Majime）の移植: 頂点数の違う正多角形（星形・角丸）の間を滑らかにモーフィングする。
//! 元はトラックバーの区間の両端の頂点数からモーフ元・先を決める。ここでは開始・終了の頂点数と進行度をパラメータで渡す。

use std::f64::consts::{PI, TAU};
use znd_plugin_sdk::*;

type V = (f64, f64);

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
fn lerp2(a: V, b: V, t: f64) -> V {
    (lerp(a.0, b.0, t), lerp(a.1, b.1, t))
}
fn len(v: V) -> f64 {
    v.0.hypot(v.1)
}

fn morphed_index(idx: f64, n: f64, m_add: f64) -> f64 {
    if m_add <= 0.0 {
        return idx;
    }
    let q = (m_add / n).floor() + 1.0;
    let rem = m_add - (m_add / n).floor() * n;
    if rem <= 0.0 {
        return idx / q;
    }
    let rem_p = (rem * 0.5).floor();
    let lucky = (rem - 2.0 * rem_p) > 0.5;
    let m = (n * 0.5).floor();
    let b1 = rem_p * (q + 1.0);
    let b2 = b1 + (m - rem_p).max(0.0) * q;
    let b3 = b2 + if lucky { q + 1.0 } else { q };
    let b4 = b3 + (n - rem_p - 1.0 - m).max(0.0) * q;
    if idx < b1 {
        let j = (idx / (q + 1.0)).floor();
        j + (idx - j * (q + 1.0)) / (q + 1.0)
    } else if idx < b2 {
        let r = idx - b1;
        let j = rem_p + (r / q).floor();
        j + (r - (j - rem_p) * q) / q
    } else if idx < b3 {
        m + (idx - b2) / if lucky { q + 1.0 } else { q }
    } else if idx < b4 {
        let r = idx - b3;
        let j = m + 1.0 + (r / q).floor();
        j + (r - (j - (m + 1.0)) * q) / q
    } else {
        let r = idx - b4;
        let j = n - rem_p + (r / (q + 1.0)).floor();
        j + (r - (j - (n - rem_p)) * (q + 1.0)) / (q + 1.0)
    }
}

fn star_perim(idx: f64, r: f64, n: f64, sf: f64, rot: f64) -> V {
    let j = (idx + 0.0001).floor();
    let f = idx - j;
    let (a1, a2) = (j * TAU / n + rot, (j + 1.0) * TAU / n + rot);
    let v1 = (r * a1.cos(), r * a1.sin());
    let v2 = (r * a2.cos(), r * a2.sin());
    let mid_a = (a1 + a2) * 0.5;
    let k = r * (PI / n).cos() * (1.0 - sf * 0.01);
    let mid = (k * mid_a.cos(), k * mid_a.sin());
    if f < 0.5 {
        lerp2(v1, mid, f * 2.0)
    } else {
        lerp2(mid, v2, (f - 0.5) * 2.0)
    }
}

fn symmetric_parent(i: f64, n: f64, q: f64, rem_p: f64, l0: bool) -> f64 {
    let c0 = if l0 { q + 1.0 } else { q };
    if i < c0 {
        return 0.0;
    }
    let mut r = i - c0;
    let b1 = rem_p * (q + 1.0);
    if r < b1 {
        return 1.0 + (r / (q + 1.0)).floor();
    }
    r -= b1;
    let b2 = (n - 1.0 - 2.0 * rem_p) * q;
    if r < b2 {
        return 1.0 + rem_p + (r / q).floor();
    }
    r -= b2;
    1.0 + rem_p + (n - 1.0 - 2.0 * rem_p) + (r / (q + 1.0)).floor()
}

#[allow(clippy::too_many_arguments)]
fn morphed_point(k: f64, total: f64, n: f64, m: f64, t: f64, rot1: f64, rot2: f64, sw: f64, linear: bool, r: f64) -> V {
    let idx = k * 0.5;
    if linear {
        return lerp2(star_perim(morphed_index(idx, n, m), r, n, sw, rot1), star_perim(idx, r, total, sw, rot2), t);
    }
    let i_f = idx.floor();
    let is_mid = idx - i_f > 0.25;
    let q = (total / n).floor();
    let rem = total - q * n;
    let rem_p = (rem * 0.5).floor();
    let l0 = (rem - 2.0 * rem_p) > 0.5;
    let i_a = symmetric_parent(i_f, n, q, rem_p, l0);
    let mut a1_b = i_f * TAU / total + rot2;
    let mut a1_a = i_a * TAU / n + rot1;
    if a1_b - a1_a > PI {
        a1_a += TAU;
    }
    if a1_a - a1_b > PI {
        a1_b += TAU;
    }
    if !is_mid {
        let a = lerp(a1_a, a1_b, t);
        return (r * a.cos(), r * a.sin());
    }
    let mut i_an = symmetric_parent((i_f + 1.01).floor(), n, q, rem_p, l0);
    if i_an >= n {
        i_an -= n;
    }
    let mut a2_a = i_an * TAU / n + rot1;
    let mut a2_b = (i_f + 1.0) * TAU / total + rot2;
    if a2_b - a2_a > PI {
        a2_a += TAU;
    }
    if a2_a - a2_b > PI {
        a2_b += TAU;
    }
    let (s, e) = (lerp(a1_a, a1_b, t), lerp(a2_a, a2_b, t));
    let sf = 1.0 - sw * 0.01;
    let sf = if i_a == i_an { lerp(1.0, sf, t) } else { sf };
    ((r * s.cos() + r * e.cos()) * 0.5 * sf, (r * s.sin() + r * e.sin()) * 0.5 * sf)
}

fn sd_regular(p: V, r: f64, n: f64, sf: f64, rot_deg: f64) -> f64 {
    if r < 1e-3 {
        return len(p);
    }
    let rad = (rot_deg - 90.0).to_radians();
    let (c, s) = (rad.cos(), rad.sin());
    let p = (p.0 * c + p.1 * s, -p.0 * s + p.1 * c);
    let step = PI / n;
    let sector = ((p.1.atan2(p.0) + step) / (2.0 * step)).floor();
    let ra = sector * 2.0 * step;
    let (cs, ss) = (ra.cos(), ra.sin());
    let mut p = (p.0 * cs + p.1 * ss, -p.0 * ss + p.1 * cs);
    p.1 = p.1.abs();
    let srf = step.cos() * (1.0 - sf * 0.01);
    let v0 = (r, 0.0);
    let v1 = (r * srf * step.cos(), r * srf * step.sin());
    let e = (v1.0 - v0.0, v1.1 - v0.1);
    let pr = (p.0 - v0.0, p.1 - v0.1);
    let d2 = e.0 * e.0 + e.1 * e.1;
    let h = if d2 > 1e-6 { ((pr.0 * e.0 + pr.1 * e.1) / d2).max(0.0) } else { 0.0 };
    let dist = len((pr.0 - e.0 * h, pr.1 - e.1 * h));
    let nrm = (-e.1, e.0);
    let sgn = pr.0 * nrm.0 + pr.1 * nrm.1;
    dist * if sgn > 0.0 { -1.0 } else if sgn < 0.0 { 1.0 } else { 0.0 }
}

fn winding(p: V, a: V, b: V) -> bool {
    let e = (b.0 - a.0, b.1 - a.1);
    (a.1 <= p.1 && p.1 < b.1 && e.0 * (p.1 - a.1) > e.1 * (p.0 - a.0)) || (b.1 <= p.1 && p.1 < a.1 && e.0 * (p.1 - a.1) < e.1 * (p.0 - a.0))
}

fn sd_morph(p: V, pts: &[V], line_w: f64) -> f64 {
    let n = pts.len();
    let mut d = f64::MAX;
    let mut sign = 1.0;
    for k in 0..n {
        let (prev, cur, next) = (pts[(k + n - 1) % n], pts[k], pts[(k + 1) % n]);
        let e = (next.0 - cur.0, next.1 - cur.1);
        let l2 = e.0 * e.0 + e.1 * e.1;
        if l2 > 1e-10 {
            let pa = (p.0 - cur.0, p.1 - cur.1);
            let h = ((pa.0 * e.0 + pa.1 * e.1) / l2).clamp(0.0, 1.0);
            d = d.min(len((pa.0 - e.0 * h, pa.1 - e.1 * h)));
        }
        let ein = (cur.0 - prev.0, cur.1 - prev.1);
        let (li, lo) = (len(ein), len(e));
        if li > 1e-5 && lo > 1e-5 && ein.0 * e.1 - ein.1 * e.0 < -1e-4 {
            // 谷（凹んだ角）: マイターの距離で埋める
            let v = (p.0 - cur.0, p.1 - cur.1);
            let cos_t = (-ein.0 / li) * (e.0 / lo) + (-ein.1 / li) * (e.1 / lo);
            let sin_half = ((1.0 - cos_t).max(1e-4) / 2.0).sqrt();
            let max_m = ((line_w + 5.0) / sin_half).min(line_w * 5.0 + 10.0);
            if v.0 * ein.0 + v.1 * ein.1 > 0.0 && v.0 * e.0 + v.1 * e.1 < 0.0 && v.0 * v.0 + v.1 * v.1 < max_m * max_m {
                let ni = (-ein.1 / li, ein.0 / li);
                let no = (-e.1 / lo, e.0 / lo);
                d = d.min((v.0 * ni.0 + v.1 * ni.1).abs().max((v.0 * no.0 + v.1 * no.1).abs()));
            }
        }
        if winding(p, cur, next) {
            sign = -sign;
        }
    }
    d * sign
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let (ns, ne) = (f.f(0).round().clamp(3.0, 100.0), f.f(1).round().clamp(3.0, 100.0));
    let mut t = f.f(2).clamp(0.0, 100.0);
    let size = f.f(3) * s;
    let line = f.f(4) * s;
    let c1 = color::premul(f.color(5), 1.0);
    let c2 = color::premul(f.color(6), (1.0 - f.f(7) / 100.0) as f32);
    let rounding = f.f(8) * s;
    let corner = f.i(9);
    let star = f.f(10);
    let linear = f.i(11) == 1;
    let (cs, ce) = (f.f(12), f.f(13));
    let (n, target) = (ns.min(ne), ns.max(ne));
    let m = target - n;
    let (mut r1, mut r2) = (-(360.0 / ns) * cs / 100.0, -(360.0 / ne) * ce / 100.0);
    if ns > ne {
        std::mem::swap(&mut r1, &mut r2);
        t = 100.0 - t;
    }
    let cr = size / 2.0;
    let n_eff = n + m * t * 0.01;
    let step = PI / n_eff;
    let srf = step.cos() * (1.0 - star * 0.01);
    let k = srf * step.sin() / (1.0 + srf * srf - 2.0 * srf * step.cos()).max(1e-6).sqrt();
    let (r_lim, r_in) = if corner == 0 {
        let rl = rounding.min(cr * k);
        (rl, ((cr * k - rl) / k).max(0.0))
    } else {
        let rl = rounding.min(cr);
        (rl, (cr - rl).max(0.0))
    };
    let regular = m == 0.0 || t == 0.0;
    let pts: Vec<V> = if regular {
        Vec::new()
    } else {
        let total = n + m;
        (0..(total * 2.0) as usize)
            .map(|k| morphed_point(k as f64, total, n, m, t * 0.01, (r1 - 90.0).to_radians(), (r2 - 90.0).to_radians(), star, linear, r_in))
            .collect()
    };
    let side = (size + 4.0 * s).ceil().max(1.0) as usize;
    let half = side as f64 / 2.0;
    let mut img = Image::new(side, side);
    img.map(|x, y, _| {
        let p = (x as f64 + 0.5 - half, y as f64 + 0.5 - half);
        let sd = if regular { sd_regular(p, r_in, n, star, r1) } else if r_in < 1e-3 { len(p) } else { sd_morph(p, &pts, line) };
        let d = sd - r_lim;
        let outer = (0.5 - d).clamp(0.0, 1.0) as f32;
        if line >= cr * 2.0 {
            return c1.map(|v| v * outer);
        }
        let inner = (0.5 - (d + line)).clamp(0.0, 1.0) as f32;
        let ol = (outer - inner).clamp(0.0, 1.0);
        std::array::from_fn(|i| c1[i] * ol + c2[i] * inner)
    });
    let ox = (f.width as i32 - side as i32) / 2;
    let oy = (f.height as i32 - side as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "Fu-Majime（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "Fu-Majime.PolygonMorph_F", name: "多角形モーフ_F", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"start_n", c"開始の頂点数", 3.0, 3.0, 100.0),
                float(c"end_n", c"終了の頂点数", 6.0, 3.0, 100.0),
                float(c"morph", c"モーフ（%）", 0.0, 0.0, 100.0),
                float(c"size", c"サイズ", 100.0, 0.0, 4000.0),
                float(c"line", c"ライン幅", 4000.0, 0.0, 4000.0),
                color_hex(c"color", c"色", 0xffffff),
                color_hex(c"color_back", c"背景色", 0xffffff),
                float(c"alpha_back", c"背景透明度", 100.0, 0.0, 100.0),
                float(c"radius", c"角半径", 20.0, 0.0, 2000.0),
                choice(c"corner_type", c"角タイプ", 1, c"谷優先|先端優先"),
                float(c"star", c"星型歪み（%）", 0.0, 0.0, 100.0),
                choice(c"morph_type", c"補間方法", 0, c"回転|直線"),
                float(c"correction_start", c"開始の回転補正（%）", 100.0, -200.0, 200.0),
                float(c"correction_end", c"終了の回転補正（%）", 100.0, -200.0, 200.0),
            ],
            render: render,
        },
    ]
}
