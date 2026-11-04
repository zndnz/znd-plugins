//! 図形の描画（アンチエイリアス付き）。AviUtl のカスタムオブジェクト（図形・多角形など）の移植に使う。
//!
//! 塗りは「面積の積算」方式（font-rs と同じ）で、画素ごとの被覆率を正確に求める。巻き数は非ゼロ規則。

use crate::image::Image;
use crate::math::V2;

/// 多角形（複数の輪郭）の被覆率（0..1、w*h）。
pub fn fill(w: usize, h: usize, contours: &[Vec<V2>]) -> Vec<f32> {
    let mut acc = vec![0.0f32; w * h + 2];
    for c in contours {
        if c.len() < 3 {
            continue;
        }
        for i in 0..c.len() {
            let (a, b) = (c[i], c[(i + 1) % c.len()]);
            line(&mut acc, w, h, a, b);
        }
    }
    let mut out = vec![0.0f32; w * h];
    for y in 0..h {
        let mut s = 0.0f32;
        for x in 0..w {
            s += acc[y * w + x];
            out[y * w + x] = s.abs().min(1.0);
        }
    }
    out
}

fn line(acc: &mut [f32], w: usize, h: usize, p0: V2, p1: V2) {
    if (p0.y - p1.y).abs() < 1e-12 {
        return;
    }
    let (dir, p0, p1) = if p0.y < p1.y { (1.0f32, p0, p1) } else { (-1.0f32, p1, p0) };
    let dxdy = (p1.x - p0.x) / (p1.y - p0.y);
    let mut x = p0.x;
    let y0 = p0.y.max(0.0);
    if p0.y < 0.0 {
        x -= p0.y * dxdy;
    }
    let y_end = p1.y.min(h as f64);
    let mut y = y0;
    let mut yi = y0.floor() as usize;
    while (yi as f64) < y_end && yi < h {
        let linestart = yi * w;
        let dy = ((yi + 1) as f64).min(p1.y) - (yi as f64).max(y);
        let xnext = x + dxdy * dy;
        let d = (dy as f32) * dir;
        let (x0, x1) = if x < xnext { (x, xnext) } else { (xnext, x) };
        let x0floor = x0.floor();
        let x0i = x0floor as i64;
        let x1ceil = x1.ceil();
        let x1i = x1ceil as i64;
        let add = |acc: &mut [f32], xi: i64, v: f32| {
            let xi = xi.clamp(0, w as i64) as usize;
            acc[linestart + xi] += v;
        };
        if x1i <= x0i + 1 {
            let xmf = (0.5 * (x + xnext) - x0floor) as f32;
            add(acc, x0i, d - d * xmf);
            add(acc, x0i + 1, d * xmf);
        } else {
            let s = (1.0 / (x1 - x0)) as f32;
            let x0f = (x0 - x0floor) as f32;
            let a0 = 0.5 * s * (1.0 - x0f) * (1.0 - x0f);
            let x1f = (x1 - x1ceil + 1.0) as f32;
            let am = 0.5 * s * x1f * x1f;
            add(acc, x0i, d * a0);
            if x1i == x0i + 2 {
                add(acc, x0i + 1, d * (1.0 - a0 - am));
            } else {
                let a1 = s * (1.5 - x0f);
                add(acc, x0i + 1, d * (a1 - a0));
                for xi in x0i + 2..x1i - 1 {
                    add(acc, xi, d * s);
                }
                let a2 = a1 + (x1i - x0i - 3) as f32 * s;
                add(acc, x1i - 1, d * (1.0 - a2 - am));
            }
            add(acc, x1i, d * am);
        }
        x = xnext;
        y = (yi + 1) as f64;
        yi += 1;
    }
}

/// 複数の図形の和（各図形を別々に塗って最大値を取る。向きの違う輪郭が打ち消し合わない）。
pub fn fill_union(w: usize, h: usize, shapes: &[Vec<Vec<V2>>]) -> Vec<f32> {
    let mut out = vec![0.0f32; w * h];
    for s in shapes {
        for (o, v) in out.iter_mut().zip(fill(w, h, s)) {
            *o = o.max(v);
        }
    }
    out
}

/// 被覆率に色（プリマルチプライド）を掛けて重ねる。
pub fn paint(img: &mut Image, coverage: &[f32], color: [f32; 4]) {
    for (p, &c) in img.data.chunks_exact_mut(4).zip(coverage) {
        if c <= 0.0 {
            continue;
        }
        let k = 1.0 - color[3] * c;
        for i in 0..4 {
            p[i] = color[i] * c + p[i] * k;
        }
    }
}

/// 円（中心・半径）を多角形で近似する。
pub fn circle(c: V2, r: f64) -> Vec<V2> {
    ellipse(c, r, r, 0.0)
}

pub fn ellipse(c: V2, rx: f64, ry: f64, rot_deg: f64) -> Vec<V2> {
    let n = ((rx.max(ry) * 2.0 * std::f64::consts::PI / 2.0).ceil() as usize).clamp(16, 2048);
    (0..n)
        .map(|i| {
            let t = i as f64 / n as f64 * std::f64::consts::TAU;
            c.add(V2::new(rx * t.cos(), ry * t.sin()).rot(rot_deg))
        })
        .collect()
}

/// 正多角形（頂点数・外接円の半径・回転）。最初の頂点は真上。
pub fn regular_polygon(c: V2, r: f64, n: usize, rot_deg: f64) -> Vec<V2> {
    let n = n.max(3);
    (0..n)
        .map(|i| {
            let t = (i as f64 / n as f64 * 360.0 + rot_deg - 90.0).to_radians();
            c.add(V2::new(r * t.cos(), r * t.sin()))
        })
        .collect()
}

/// 星形（外側の半径・内側の半径）。
pub fn star(c: V2, r_out: f64, r_in: f64, n: usize, rot_deg: f64) -> Vec<V2> {
    let n = n.max(2);
    (0..n * 2)
        .map(|i| {
            let r = if i % 2 == 0 { r_out } else { r_in };
            let t = (i as f64 / (n * 2) as f64 * 360.0 + rot_deg - 90.0).to_radians();
            c.add(V2::new(r * t.cos(), r * t.sin()))
        })
        .collect()
}

/// 角を丸めた多角形（各頂点を半径 `r` の円弧で置き換える）。
pub fn round_corners(poly: &[V2], r: f64) -> Vec<V2> {
    let n = poly.len();
    if n < 3 || r <= 0.0 {
        return poly.to_vec();
    }
    let mut out = Vec::new();
    for i in 0..n {
        let (p, a, b) = (poly[i], poly[(i + n - 1) % n], poly[(i + 1) % n]);
        let (u, v) = (a.sub(p).norm(), b.sub(p).norm());
        let cosang = u.dot(v).clamp(-1.0, 1.0);
        let ang = cosang.acos();
        if ang < 1e-6 || (std::f64::consts::PI - ang) < 1e-6 {
            out.push(p);
            continue;
        }
        // 角から接点までの距離。辺の半分を超えないようにする
        let max_d = a.sub(p).len().min(b.sub(p).len()) * 0.5;
        let d = (r / (ang / 2.0).tan()).min(max_d);
        let rr = d * (ang / 2.0).tan();
        let (t1, t2) = (p.add(u.mul(d)), p.add(v.mul(d)));
        let bis = u.add(v).norm();
        let center = p.add(bis.mul((d * d + rr * rr).sqrt()));
        let (s, e) = ((t1.y - center.y).atan2(t1.x - center.x), (t2.y - center.y).atan2(t2.x - center.x));
        let mut sweep = e - s;
        while sweep > std::f64::consts::PI {
            sweep -= std::f64::consts::TAU;
        }
        while sweep < -std::f64::consts::PI {
            sweep += std::f64::consts::TAU;
        }
        let steps = ((rr * sweep.abs()).ceil() as usize).clamp(2, 256);
        for k in 0..=steps {
            let t = s + sweep * k as f64 / steps as f64;
            out.push(V2::new(center.x + rr * t.cos(), center.y + rr * t.sin()));
        }
    }
    out
}

/// 太さ `width` の線分（端は丸くしない）を四角形にする。
pub fn segment(a: V2, b: V2, width: f64) -> Vec<V2> {
    let n = V2::new(-(b.y - a.y), b.x - a.x).norm().mul(width / 2.0);
    vec![a.add(n), b.add(n), b.sub(n), a.sub(n)]
}

/// 折れ線を太さ `width` で描くための図形の列（線分ごとの四角形と、つなぎ目の円）。`fill_union` で塗る。
pub fn stroke(points: &[V2], width: f64, closed: bool, round: bool) -> Vec<Vec<Vec<V2>>> {
    let mut shapes = Vec::new();
    let n = points.len();
    if n < 2 {
        if let (Some(p), true) = (points.first(), round) {
            shapes.push(vec![circle(*p, width / 2.0)]);
        }
        return shapes;
    }
    let segs = if closed { n } else { n - 1 };
    for i in 0..segs {
        shapes.push(vec![segment(points[i], points[(i + 1) % n], width)]);
    }
    if round {
        for p in points {
            shapes.push(vec![circle(*p, width / 2.0)]);
        }
    }
    shapes
}

/// 3 次ベジェを折れ線にする。
pub fn cubic(p0: V2, p1: V2, p2: V2, p3: V2, steps: usize) -> Vec<V2> {
    (0..=steps)
        .map(|i| {
            let t = i as f64 / steps as f64;
            let u = 1.0 - t;
            p0.mul(u * u * u).add(p1.mul(3.0 * u * u * t)).add(p2.mul(3.0 * u * t * t)).add(p3.mul(t * t * t))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_coverage_is_exact() {
        let sq = vec![V2::new(1.5, 1.0), V2::new(3.5, 1.0), V2::new(3.5, 3.0), V2::new(1.5, 3.0)];
        let c = fill(5, 5, &[sq]);
        assert!((c[5 + 1] - 0.5).abs() < 1e-5);
        assert!((c[5 + 2] - 1.0).abs() < 1e-5);
        assert!((c[5 + 3] - 0.5).abs() < 1e-5);
        assert_eq!(c[0], 0.0);
        let total: f32 = c.iter().sum();
        assert!((total - 4.0).abs() < 1e-4);
    }

    #[test]
    fn circle_area_matches() {
        let c = fill(64, 64, &[circle(V2::new(32.0, 32.0), 20.0)]);
        let area: f32 = c.iter().sum();
        assert!((area - std::f32::consts::PI * 400.0).abs() < 4.0, "{area}");
    }
}

/// 三角形 `d` に、`src` の三角形 `s`（画素座標）を貼って `dst` に重ねる（アフィン、バイリニア、プリマルチプライドの通常合成）。
/// 隣り合う三角形の継ぎ目が二重に塗られないよう、左上規則で塗る。
pub fn textured_triangle(dst: &mut Image, src: &Image, d: [V2; 3], s: [V2; 3], alpha: f32) {
    let area = (d[1].x - d[0].x) * (d[2].y - d[0].y) - (d[2].x - d[0].x) * (d[1].y - d[0].y);
    if area.abs() < 1e-12 {
        return;
    }
    // 向きをそろえる（反時計回りにする）
    let (d, s) = if area < 0.0 { ([d[0], d[2], d[1]], [s[0], s[2], s[1]]) } else { (d, s) };
    let area = area.abs();
    let x0 = d.iter().map(|p| p.x).fold(f64::INFINITY, f64::min).floor().max(0.0) as usize;
    let y0 = d.iter().map(|p| p.y).fold(f64::INFINITY, f64::min).floor().max(0.0) as usize;
    let x1 = (d.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max).ceil().max(0.0) as usize).min(dst.w);
    let y1 = (d.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max).ceil().max(0.0) as usize).min(dst.h);
    let edge = |a: V2, b: V2, p: V2| (b.x - a.x) * (p.y - a.y) - (p.x - a.x) * (b.y - a.y);
    // 左上規則: 上の辺・左の辺だけ境界を含める
    let top_left = |a: V2, b: V2| (a.y == b.y && b.x < a.x) || b.y < a.y;
    let incl = [top_left(d[1], d[2]), top_left(d[2], d[0]), top_left(d[0], d[1])];
    for y in y0..y1 {
        for x in x0..x1 {
            let p = V2::new(x as f64 + 0.5, y as f64 + 0.5);
            let w = [edge(d[1], d[2], p), edge(d[2], d[0], p), edge(d[0], d[1], p)];
            if (0..3).any(|k| w[k] < 0.0 || (w[k] == 0.0 && !incl[k])) {
                continue;
            }
            let (b0, b1, b2) = (w[0] / area, w[1] / area, w[2] / area);
            let u = s[0].x * b0 + s[1].x * b1 + s[2].x * b2;
            let v = s[0].y * b0 + s[1].y * b1 + s[2].y * b2;
            let c = src.sample(u as f32, v as f32);
            if c[3] <= 0.0 {
                continue;
            }
            let i = dst.idx(x, y);
            let k = 1.0 - c[3] * alpha;
            for ch in 0..4 {
                dst.data[i + ch] = c[ch] * alpha + dst.data[i + ch] * k;
            }
        }
    }
}

/// AviUtl の `obj.drawpoly` に当たるもの: 四角形 `d` に `src` の四角形 `s` を 2 つの三角形に分けて貼る。
pub fn textured_quad(dst: &mut Image, src: &Image, d: [V2; 4], s: [V2; 4], alpha: f32) {
    textured_triangle(dst, src, [d[0], d[1], d[2]], [s[0], s[1], s[2]], alpha);
    textured_triangle(dst, src, [d[0], d[2], d[3]], [s[0], s[2], s[3]], alpha);
}
