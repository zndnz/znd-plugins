//! Kaleidoscope_K（korarei）の移植: 万華鏡（After Effects の CC Kaleida に近い 10 種類の折り返し）。元のシェーダーをそのまま移した。

use znd_plugin_sdk::*;

type V = (f64, f64);
const RS2: f64 = std::f64::consts::FRAC_1_SQRT_2;

fn r45(p: V) -> V {
    ((p.0 - p.1) * RS2, (p.0 + p.1) * RS2)
}
fn fract(v: f64) -> f64 {
    v - v.floor()
}
fn pingpong(p: V) -> V {
    (1.0 - (fract(p.0 * 0.5) * 2.0 - 1.0).abs(), 1.0 - (fract(p.1 * 0.5) * 2.0 - 1.0).abs())
}
/// (奇数か, タイル内の位置)
fn wrap(p: V) -> ((i64, i64), V) {
    let (ix, iy) = (p.0.floor(), p.1.floor());
    (((ix as i64) & 1, (iy as i64) & 1), (p.0 - ix, p.1 - iy))
}
fn mirror(p: V) -> ((i64, i64), V) {
    let (ix, iy) = (p.0.floor(), p.1.floor());
    let odd = ((ix as i64) & 1, (iy as i64) & 1);
    (odd, ((odd.0 as f64 + ix - p.0).abs(), (odd.1 as f64 + iy - p.1).abs()))
}
fn flower(p: V) -> V {
    let p = pingpong(p);
    if p.1 <= p.0 {
        (p.1, p.0)
    } else {
        p
    }
}

fn fold(mode: usize, p: V) -> V {
    match mode {
        0 => pingpong(p),
        1 => {
            let (o, t) = mirror(p);
            if (o.0 ^ o.1) != 0 {
                (t.1, t.0)
            } else {
                t
            }
        }
        2 => {
            let (o, t) = mirror(p);
            if o.0 != 0 {
                t
            } else {
                (t.1, t.0)
            }
        }
        3 => {
            let (o, t) = mirror(p);
            if o.1 != 0 {
                t
            } else {
                (t.1, t.0)
            }
        }
        4 => {
            let (o, t) = wrap(p);
            ((o.0 as f64 - t.0).abs(), (1.0 - t.1).abs())
        }
        5 => flower(p),
        6 => {
            let q = (fract(p.0 * 0.5 + 0.5) * 2.0 - 1.0, fract(p.1 * 0.5 + 0.5) * 2.0 - 1.0);
            let a = r45(q);
            r45((a.1.abs(), a.0.abs()))
        }
        7 => {
            let (o, t) = wrap(p);
            ((o.1 as f64 - t.0).abs(), (o.1 as f64 - t.1).abs())
        }
        8 => {
            let p0 = flower(p);
            let p1 = r45((p0.1, p0.0));
            if p1.0 <= p0.0 {
                p1
            } else {
                p0
            }
        }
        _ => p,
    }
}

fn mirror_coord(v: f64) -> f64 {
    // サンプラーの mirror（0..1 を往復）
    let t = v.rem_euclid(2.0);
    if t > 1.0 {
        2.0 - t
    } else {
        t
    }
}

fn render(f: &mut Frame) -> FxResult {
    let (w, h) = (f.width as f64, f.height as f64);
    let c = f.point(0);
    let (cx, cy) = (c[0] / w, c[1] / h);
    let size = (f.f(1) / 100.0).max(1e-4);
    let rot = f.f(2).to_radians();
    let scale = (f.f(3) / 100.0).max(1e-4);
    let floating = f.b(4);
    let mode = f.i(5);
    let aspect = h / w;
    let (ox, oy) = if floating { (cx, cy) } else { (0.5, 0.5) };
    let (cs, sn) = (rot.cos(), rot.sin());
    let src = f.input();
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (u, v) = ((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
        let mut p = ((u - ox) * 2.0 / size / scale, (v - oy) * 2.0 / size / scale * aspect);
        p = (-p.0, -p.1);
        p = fold(mode, p);
        let q = (-p.0, -p.1);
        let mut r = (cs * q.0 - sn * q.1, sn * q.0 + cs * q.1);
        r.1 /= aspect;
        let (su, sv) = (mirror_coord(r.0 * 0.5 * size + cx), mirror_coord(r.1 * 0.5 * size + cy));
        src.sample_clamped((su * w) as f32, (sv * h) as f32)
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "korarei（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "korarei.Kaleidoscope_K", name: "万華鏡（Kaleidoscope_K）", category: "ディストーション", flags: THREAD_SAFE,
            params: [
                point(c"center", c"中心", 0.5, 0.5),
                float(c"tile_size", c"タイルサイズ", 100.0, 0.0, 1000.0),
                float(c"rotation", c"回転", 0.0, -3600.0, 3600.0),
                float(c"scale", c"拡大率", 100.0, 0.0, 1000.0),
                boolean(c"floating_center", c"中心を浮かせる（Floating Center）", false),
                choice(c"mirroring", c"折り返し", 0, c"Unfold|Wheel|Fish Head|Can Meas|Flip Flop|Flower|Dia Cross|Flipper|Starlish|Tiler"),
            ],
            render: render,
        },
    ]
}
