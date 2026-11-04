//! ドット絵変形（Nanashi.）の移植: ドット絵を、ドットのまま（最近傍で）拡大縮小・回転する。
//! 「発展補間」を入れると cleanEdge（torcado、MIT）で線を残しながら斜めをなめらかにする。

mod clean_edge;
use clean_edge::{Ce, C};
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let (cx0, cy0) = (f.f(0) * sc, f.f(1) * sc);
    let s = f.f(2) / 100.0;
    let (rsx, rsy) = (f.f(3) / 100.0 * s, f.f(4) / 100.0 * s);
    let ang = f.f(5).to_radians();
    let clean = f.b(6);
    let slopes = f.i(8);
    let hc = f.color(9);
    let ce = Ce {
        highest: [0, 1, 2].map(|i| color::linear_to_srgb(hc[i]) as f64),
        thr: f.f(10) / 255.0,
        line_width: f.f(7),
        slope: slopes >= 1,
        cleanup: slopes >= 2,
    };
    let grid = (f.f(11) * sc).round();
    let mut src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    if grid > 0.0 {
        // 透明な所に市松模様を敷く
        let g = grid;
        let base = Image::generate(src.w, src.h, |x, y| {
            let v = if ((x as f64 / g).floor() + (y as f64 / g).floor()) % 2.0 < 1.0 { 0.25 } else { 0.0 };
            color::from_display([v, v, v, 1.0])
        });
        let mut b = base;
        b.draw(&src, 0, 0, 1.0);
        src = b;
    }
    let (cx, cy) = (w / 2.0 + cx0, h / 2.0 + cy0);
    let rot = |x: f64, y: f64, a: f64| (a.cos() * x - a.sin() * y, a.sin() * x + a.cos() * y);
    let corners = [(-cx * rsx, -cy * rsy), ((w - cx) * rsx, -cy * rsy), (-cx * rsx, (h - cy) * rsy), ((w - cx) * rsx, (h - cy) * rsy)].map(|(x, y)| {
        let (a, b) = rot(x, y, ang);
        (a + cx, b + cy)
    });
    let min_x = corners.iter().map(|c| c.0).fold(f64::INFINITY, f64::min).floor();
    let max_x = corners.iter().map(|c| c.0).fold(f64::NEG_INFINITY, f64::max).ceil();
    let min_y = corners.iter().map(|c| c.1).fold(f64::INFINITY, f64::min).floor();
    let max_y = corners.iter().map(|c| c.1).fold(f64::NEG_INFINITY, f64::max).ceil();
    let (nw, nh) = (((max_x - min_x) as usize).clamp(1, 16384), ((max_y - min_y) as usize).clamp(1, 16384));
    // 色の比べ方は元と同じく、乗算済みの表示値
    let disp: Vec<C> = src.data.chunks_exact(4).map(|p| {
        let d = color::to_display([p[0], p[1], p[2], p[3]]);
        let a = d[3] as f64;
        [d[0] as f64 * a, d[1] as f64 * a, d[2] as f64 * a, a]
    }).collect();
    let (sw, shh) = (src.w as isize, src.h as isize);
    let get = |x: f64, y: f64| -> C {
        let (xi, yi) = ((x.floor() as isize).clamp(0, sw - 1), (y.floor() as isize).clamp(0, shh - 1));
        disp[yi as usize * src.w + xi as usize]
    };
    let out = Image::generate(nw, nh, |x, y| {
        let (rx, ry) = (min_x + x as f64 + 0.5 - cx, min_y + y as f64 + 0.5 - cy);
        let (a, b) = if ang == 0.0 { (rx, ry) } else { rot(rx, ry, -ang) };
        let (sx, sy) = (a / rsx + cx, b / rsy + cy);
        let c = if !clean {
            if sx < 0.0 || sx >= w || sy < 0.0 || sy >= h { [0.0; 4] } else { get(sx, sy) }
        } else {
            let spx = [sx / w * (w + 0.0001), sy / h * (h + 0.0001)];
            let loc = [spx[0] - spx[0].floor(), spx[1] - spx[1].floor()];
            let sp = [spx[0].floor(), spx[1].floor()];
            let pd = [if loc[0] >= 0.5 { 1.0 } else { -1.0 }, if loc[1] >= 0.5 { 1.0 } else { -1.0 }];
            let g = |dx: f64, dy: f64| get(sp[0] + pd[0] * dx + 0.5, sp[1] + pd[1] * dy + 0.5);
            let (uub, uup, uuf) = (g(-1.0, -2.0), g(0.0, -2.0), g(1.0, -2.0));
            let (ubb, ub, up, uf, uff) = (g(-2.0, -1.0), g(-1.0, -1.0), g(0.0, -1.0), g(1.0, -1.0), g(2.0, -1.0));
            let (cbb, cb, cp, cf, cff) = (g(-2.0, 0.0), g(-1.0, 0.0), g(0.0, 0.0), g(1.0, 0.0), g(2.0, 0.0));
            let (dbb, db, dp, df, dff) = (g(-2.0, 1.0), g(-1.0, 1.0), g(0.0, 1.0), g(1.0, 1.0), g(2.0, 1.0));
            let (ddb, ddp, ddf) = (g(-1.0, 2.0), g(0.0, 2.0), g(1.0, 2.0));
            let mut col = cp;
            let c1 = ce.slice(loc, [1.0, 1.0], pd, ub, up, uf, uff, cb, cp, cf, cff, db, dp, df, dff, ddb, ddp, ddf);
            let c2 = ce.slice(loc, [-1.0, 1.0], pd, uf, up, ub, ubb, cf, cp, cb, cbb, df, dp, db, dbb, ddf, ddp, ddb);
            let c3 = ce.slice(loc, [1.0, -1.0], pd, db, dp, df, dff, cb, cp, cf, cff, ub, up, uf, uff, uub, uup, uuf);
            for c in [c1, c2, c3] {
                if c[0] >= 0.0 {
                    col = c;
                }
            }
            if sx < 0.0 || sx >= w || sy < 0.0 || sy >= h { [0.0; 4] } else { col }
        };
        let a = c[3].clamp(0.0, 1.0) as f32;
        if a <= 0.0 {
            return [0.0; 4];
        }
        color::from_display([(c[0] / c[3]) as f32, (c[1] / c[3]) as f32, (c[2] / c[3]) as f32, a].map(|v| v.clamp(0.0, 1.0)))
    });
    f.output(out, min_x as i32, min_y as i32);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.pixel-transform-anm2", name: "ドット絵変形", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"center_x", c"中心X", 0.0, -5000.0, 5000.0),
                float(c"center_y", c"中心Y", 0.0, -5000.0, 5000.0),
                float(c"scale", c"拡大率", 100.0, 1.0, 10000.0),
                float(c"scale_x", c"X拡大率", 100.0, 1.0, 10000.0),
                float(c"scale_y", c"Y拡大率", 100.0, 1.0, 10000.0),
                float(c"angle", c"回転（度）", 0.0, -360.0, 360.0),
                boolean(c"cleanedge", c"発展補間", false),
                float(c"line_width", c"線の太さ", 1.0, 0.0, 4.0),
                choice(c"slopes", c"斜め補間", 2, c"1:1のみ|1:1 + 1:2|1:1 + 1:2（補正）"),
                color_hex(c"highest_color", c"基準色", 0xffffff),
                float(c"similar_threshold", c"補間閾値", 16.0, 0.0, 255.0),
                float(c"alpha_grid", c"透明グリッド", 0.0, 0.0, 1000.0),
            ],
            render: render,
        },
    ]
}
