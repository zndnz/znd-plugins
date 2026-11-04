//! 曲げKR（karoterra）の移植: 始点と終点の間を円弧かベジェ曲線に沿って曲げる。
//! 始点・終点を結ぶ向きを v 軸、直角を u 軸とした座標で、曲線上の位置と法線方向のずれに写す（元と同じ式）。
//! 始点より手前・終点より先は、曲線の端の向きに合わせて回して平行移動するだけ（元の transformEdge）。
//! 元は帯ごとの drawpoly で描くが、ここでは細かい格子を写して三角形で貼る。

use std::f64::consts::PI;
use znd_plugin_sdk::draw::textured_quad;
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

struct Rigid {
    from: V2,
    to: V2,
    rot: f64,
}
impl Rigid {
    fn apply(&self, p: V2) -> V2 {
        let (s, c) = self.rot.sin_cos();
        let d = p.sub(self.from);
        V2::new(d.x * c - d.y * s + self.to.x, d.x * s + d.y * c + self.to.y)
    }
}

/// 曲げの写像。座標はオブジェクトの中心が原点。
struct Bend<'a> {
    anc_s: V2,
    rot_c: f64,
    rot_s: f64,
    dist: f64,
    curve: &'a dyn Fn(f64, f64) -> V2,
    start: Rigid,
    end: Rigid,
}
impl Bend<'_> {
    /// (u, v): 始点からの直角方向・沿う方向の距離。
    fn uv(&self, p: V2) -> (f64, f64) {
        let (x, y) = (p.x - self.anc_s.x, p.y - self.anc_s.y);
        (x * self.rot_c - y * self.rot_s, x * self.rot_s + y * self.rot_c)
    }
    fn map(&self, p: V2) -> V2 {
        let (u, v) = self.uv(p);
        if v < 0.0 {
            self.start.apply(p)
        } else if v > self.dist {
            self.end.apply(p)
        } else {
            (self.curve)(u, v)
        }
    }
}

struct Common {
    div: usize,
    subdiv: usize,
    start_mode: bool,
    end_mode: bool,
    center_mode: usize,
}

fn common(f: &Frame, base: usize) -> Common {
    Common {
        div: f.f(base).round().max(1.0) as usize,
        subdiv: f.f(base + 1).round().max(1.0) as usize,
        start_mode: f.b(base + 2),
        end_mode: f.b(base + 3),
        center_mode: f.i(base + 4),
    }
}

/// 格子を写して描き、(画像, 左上の位置) を返す。marks は中心の選択肢で使う点（0: 変形後の中心の後に続く）。
fn warp(f: &mut Frame, b: &Bend, cm: &Common, marks: &[V2]) {
    let src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    // 格子の細かさ: 始点と終点の間を 分割数×細分化 に割った長さ（2～16px）
    let step = (b.dist / (cm.div * cm.subdiv) as f64).clamp(2.0, 16.0);
    let nx = (w / step).ceil().max(1.0) as usize;
    let ny = (h / step).ceil().max(1.0) as usize;
    let at = |i: usize, j: usize| V2::new((i as f64 * step).min(w), (j as f64 * step).min(h));
    let mut grid = vec![V2::new(0.0, 0.0); (nx + 1) * (ny + 1)];
    let (mut lo, mut hi) = (V2::new(f64::INFINITY, f64::INFINITY), V2::new(f64::NEG_INFINITY, f64::NEG_INFINITY));
    let c0 = V2::new(w / 2.0, h / 2.0);
    let keep = |p: V2| {
        let (_, v) = b.uv(p.sub(c0));
        (cm.start_mode || v >= 0.0) && (cm.end_mode || v <= b.dist)
    };
    for j in 0..=ny {
        for i in 0..=nx {
            let p = at(i, j);
            let q = b.map(p.sub(c0));
            grid[j * (nx + 1) + i] = q;
        }
    }
    for j in 0..ny {
        for i in 0..nx {
            let mid = at(i, j).lerp(at(i + 1, j + 1), 0.5);
            if !keep(mid) {
                continue;
            }
            for (a, bb) in [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)] {
                let q = grid[bb * (nx + 1) + a];
                lo = V2::new(lo.x.min(q.x), lo.y.min(q.y));
                hi = V2::new(hi.x.max(q.x), hi.y.max(q.y));
            }
        }
    }
    if !lo.x.is_finite() {
        f.output(Image::new(1, 1), 0, 0);
        return;
    }
    let ox = (lo.x - 1.0).floor();
    let oy = (lo.y - 1.0).floor();
    let ow = ((hi.x + 1.0).ceil() - ox).clamp(1.0, 16384.0) as usize;
    let oh = ((hi.y + 1.0).ceil() - oy).clamp(1.0, 16384.0) as usize;
    let off = V2::new(ox, oy);
    let mut out = Image::new(ow, oh);
    for j in 0..ny {
        for i in 0..nx {
            let mid = at(i, j).lerp(at(i + 1, j + 1), 0.5);
            if !keep(mid) {
                continue;
            }
            let g = |a: usize, bb: usize| grid[bb * (nx + 1) + a].sub(off);
            textured_quad(
                &mut out,
                &src,
                [g(i, j), g(i + 1, j), g(i + 1, j + 1), g(i, j + 1)],
                [at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)],
                1.0,
            );
        }
    }
    // 中心: 0 は元の中心のまま、1 は変形後の範囲の中心、2 以降は印の点を元の中心に合わせる
    let shift = match cm.center_mode {
        0 => V2::new(0.0, 0.0),
        1 => V2::new((lo.x + hi.x) / 2.0, (lo.y + hi.y) / 2.0),
        k => marks.get(k - 2).copied().unwrap_or(V2::new(0.0, 0.0)),
    };
    let px = (ox + c0.x - shift.x).round() as i32;
    let py = (oy + c0.y - shift.y).round() as i32;
    f.output(out, px, py);
}

fn anchors(f: &Frame, i: usize) -> V2 {
    V2::new(f.f(i) * f.scale(), f.f(i + 1) * f.scale())
}

fn clamp_pt(p: V2, w: f64, h: f64) -> V2 {
    V2::new(p.x.clamp(-w / 2.0, w / 2.0), p.y.clamp(-h / 2.0, h / 2.0))
}

fn render_arc(f: &mut Frame) -> FxResult {
    let curvature = f.f(0) / 10000.0;
    let rate = f.f(1) / 100.0;
    let cm = common(f, 2);
    let collapse = f.b(7);
    let (w, h) = (f.width as f64, f.height as f64);
    let anc_s = clamp_pt(anchors(f, 8), w, h);
    let anc_e = clamp_pt(anchors(f, 10), w, h);
    let anc_c = anc_s.lerp(anc_e, rate);
    let anc_rad = (anc_e.y - anc_s.y).atan2(anc_e.x - anc_s.x);
    let dist = anc_e.sub(anc_s).len();
    let rr = -anc_rad + PI / 2.0;
    if curvature == 0.0 || dist < 1e-6 {
        let ident = |u: f64, v: f64| {
            let (s, c) = (-rr).sin_cos();
            V2::new(anc_s.x + u * c - v * s, anc_s.y + u * s + v * c)
        };
        let id_r = |p: V2| Rigid { from: p, to: p, rot: 0.0 };
        let b = Bend { anc_s, rot_c: rr.cos(), rot_s: rr.sin(), dist: dist.max(1e-6), curve: &ident, start: id_r(anc_s), end: id_r(anc_e) };
        warp(f, &b, &cm, &[anc_s, anc_e, anc_c, anc_s, anc_e]);
        return Ok(());
    }
    let radius = 1.0 / curvature;
    let curv_rad = anc_rad + PI / 2.0;
    let cc = V2::new(anc_c.x + radius * curv_rad.cos(), anc_c.y + radius * curv_rad.sin());
    let s_rad = curv_rad - PI - rate * dist / radius;
    let e_rad = curv_rad - PI + (1.0 - rate) * dist / radius;
    let curv_s = V2::new(cc.x + radius * s_rad.cos(), cc.y + radius * s_rad.sin());
    let curv_e = V2::new(cc.x + radius * e_rad.cos(), cc.y + radius * e_rad.sin());
    let curve = move |u: f64, v: f64| {
        let mut r = radius + u;
        // 破綻回避: 中心を越えて裏返らないよう半径を 0 で止める
        if collapse && r * radius < 0.0 {
            r = 0.0;
        }
        let a = s_rad + v / radius;
        V2::new(cc.x + r * a.cos(), cc.y + r * a.sin())
    };
    let b = Bend {
        anc_s,
        rot_c: rr.cos(),
        rot_s: rr.sin(),
        dist,
        curve: &curve,
        start: Rigid { from: anc_s, to: curv_s, rot: s_rad - anc_rad + PI / 2.0 },
        end: Rigid { from: anc_e, to: curv_e, rot: e_rad - anc_rad + PI / 2.0 },
    };
    warp(f, &b, &cm, &[anc_s, anc_e, anc_c, curv_s, curv_e]);
    Ok(())
}

fn render_bezier(f: &mut Frame) -> FxResult {
    let cm = common(f, 0);
    let (w, h) = (f.width as f64, f.height as f64);
    let anc_s = clamp_pt(anchors(f, 5), w, h);
    let anc_e = clamp_pt(anchors(f, 7), w, h);
    let p3 = anchors(f, 9);
    let p1 = anchors(f, 11);
    let p2 = anchors(f, 13);
    let anc_rad = (anc_e.y - anc_s.y).atan2(anc_e.x - anc_s.x);
    let dist = anc_e.sub(anc_s).len().max(1e-6);
    let rr = -anc_rad + PI / 2.0;
    let p0 = anc_s;
    let dt = move |t: f64| {
        let d = |a: f64, b: f64, c: f64, e: f64| 3.0 * (-a + 3.0 * (b - c) + e) * t * t + 6.0 * (a - 2.0 * b + c) * t + 3.0 * (-a + b);
        V2::new(d(p0.x, p1.x, p2.x, p3.x), d(p0.y, p1.y, p2.y, p3.y))
    };
    let pt = move |t: f64| {
        let m = 1.0 - t;
        V2::new(
            m * m * m * p0.x + 3.0 * m * m * t * p1.x + 3.0 * m * t * t * p2.x + t * t * t * p3.x,
            m * m * m * p0.y + 3.0 * m * m * t * p1.y + 3.0 * m * t * t * p2.y + t * t * t * p3.y,
        )
    };
    // 弧長の表（台形積分）。終点が曲線より遠いときのために t は 1 を越えて伸ばす
    let step = 0.001;
    let mut table = vec![(0.0f64, 0.0f64)];
    let (mut t, mut l, mut dl_prev) = (0.0, 0.0, dt(0.0).len());
    while l < dist && t < 50.0 {
        t += step;
        let dl = dt(t).len();
        l += (dl + dl_prev) * step / 2.0;
        dl_prev = dl;
        table.push((t, l));
    }
    let t_of = |v: f64| -> f64 {
        let i = table.partition_point(|e| e.1 < v);
        if i == 0 {
            return 0.0;
        }
        if i >= table.len() {
            return table.last().unwrap().0;
        }
        let (a, b) = (table[i - 1], table[i]);
        let k = if b.1 > a.1 { (v - a.1) / (b.1 - a.1) } else { 0.0 };
        a.0 + (b.0 - a.0) * k
    };
    let curve = |u: f64, v: f64| {
        let t = t_of(v);
        let d = dt(t);
        let rad = d.y.atan2(d.x) - PI / 2.0;
        let p = pt(t);
        V2::new(p.x + u * rad.cos(), p.y + u * rad.sin())
    };
    let te = t_of(dist);
    let end_pt = pt(te);
    let de = dt(te);
    let start_rot = (p1.y - anc_s.y).atan2(p1.x - anc_s.x) - anc_rad;
    let b = Bend {
        anc_s,
        rot_c: rr.cos(),
        rot_s: rr.sin(),
        dist,
        curve: &curve,
        start: Rigid { from: anc_s, to: anc_s, rot: start_rot },
        end: Rigid { from: anc_e, to: end_pt, rot: de.y.atan2(de.x) - anc_rad },
    };
    warp(f, &b, &cm, &[anc_s, anc_e, end_pt, p3]);
    Ok(())
}

plugin! {
    vendor: "かろてら（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "Karoterra.Bend.arc", name: "円弧@曲げKR", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"curvature", c"曲率", 0.0, -10000.0, 10000.0),
                float(c"rate", c"割合", 0.0, 0.0, 100.0),
                float(c"div", c"分割数", 16.0, 1.0, 5000.0),
                float(c"subdiv", c"細分化", 1.0, 1.0, 10.0),
                boolean(c"start", c"始点", true),
                boolean(c"end", c"終点", true),
                choice(c"center", c"中心", 0, c"オリジナルの中心|変形後の中心|赤マーカー（始点）|青マーカー（終点）|緑マーカー（割合の点）|マゼンダマーカー（曲げた始点）|シアンマーカー（曲げた終点）"),
                boolean(c"collapse", c"破綻回避", true),
                float(c"sx", c"始点X", 0.0, -4000.0, 4000.0),
                float(c"sy", c"始点Y", 100.0, -4000.0, 4000.0),
                float(c"ex", c"終点X", 0.0, -4000.0, 4000.0),
                float(c"ey", c"終点Y", -100.0, -4000.0, 4000.0),
            ],
            render: render_arc,
        },
        {
            id: "Karoterra.Bend.bezier", name: "ベジェ曲線@曲げKR", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"div", c"分割数", 16.0, 1.0, 5000.0),
                float(c"subdiv", c"細分化", 1.0, 1.0, 10.0),
                boolean(c"start", c"始点", true),
                boolean(c"end", c"終点", true),
                choice(c"center", c"中心", 0, c"オリジナルの中心|変形後の中心|赤マーカー（始点）|青マーカー（終点）|シアンマーカー（曲げた終点）|緑マーカー（曲線の終点）"),
                float(c"sx", c"始点X", 0.0, -4000.0, 4000.0),
                float(c"sy", c"始点Y", 100.0, -4000.0, 4000.0),
                float(c"ex", c"終点X", 0.0, -4000.0, 4000.0),
                float(c"ey", c"終点Y", -100.0, -4000.0, 4000.0),
                float(c"p3x", c"曲線の終点X", 100.0, -4000.0, 4000.0),
                float(c"p3y", c"曲線の終点Y", -100.0, -4000.0, 4000.0),
                float(c"p1x", c"制御点1X", 0.0, -4000.0, 4000.0),
                float(c"p1y", c"制御点1Y", 0.0, -4000.0, 4000.0),
                float(c"p2x", c"制御点2X", 100.0, -4000.0, 4000.0),
                float(c"p2y", c"制御点2Y", 0.0, -4000.0, 4000.0),
            ],
            render: render_bezier,
        },
    ]
}
