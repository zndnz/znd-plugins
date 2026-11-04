//! BezierPath（猫愛）の移植: 任意の次数のベジェ曲線に沿って、オブジェクトを並べる。
//! このエフェクトは「ガイド」で、曲線の点と並べ方の設定を持ち、曲線と点を描く（ガイド表示）。
//! 並べるレイヤーの位置・回転・不透明度は、エクスプレッション用ライブラリの bezierPath() が、このエフェクトの値から計算する。

use znd_plugin_sdk::draw::{fill, fill_union, paint, regular_polygon, circle, segment};
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

pub const P0: usize = 7;
pub const MAX_POINTS: usize = 10;

fn bezier(t: f64, pts: &[V2]) -> V2 {
    let n = pts.len();
    let mut out = V2::new(0.0, 0.0);
    let mut binom = 1.0;
    for i in 0..n {
        let m = binom * t.powi(i as i32) * (1.0 - t).powi((n - 1 - i) as i32);
        out = out.add(pts[i].mul(m));
        binom = binom * (n - 1 - i) as f64 / (i + 1) as f64;
    }
    out
}

fn render(f: &mut Frame) -> FxResult {
    let mut img = Image::new(f.width, f.height);
    if !f.b(4) {
        f.set(img);
        return Ok(());
    }
    let s = f.scale();
    let n = (f.f(0).round().clamp(1.0, (MAX_POINTS - 1) as f64) as usize) + 1;
    let pts: Vec<V2> = (0..n).map(|i| { let p = f.point(P0 + i); V2::new(p[0], p[1]) }).collect();
    let lw = f.f(5) * s;
    let hs = f.f(6) * s;
    let curve: Vec<V2> = (0..=400).map(|i| bezier(i as f64 / 400.0, &pts)).collect();
    let shapes: Vec<Vec<Vec<V2>>> = curve.windows(2).map(|w| vec![segment(w[0], w[1], lw)]).chain(curve.iter().map(|p| vec![circle(*p, lw / 2.0)])).collect();
    let (w, h) = (img.w, img.h);
    paint(&mut img, &fill_union(w, h, &shapes), color::premul([0.025, 0.4, 0.75, 1.0], 1.0));
    for (i, p) in pts.iter().enumerate() {
        let (shape, col) = if i == 0 {
            (regular_polygon(*p, hs * 0.7, 4, 45.0), [0.05, 0.75, 0.03, 1.0])
        } else if i == n - 1 {
            (regular_polygon(*p, hs * 0.7, 4, 45.0), [0.55, 0.01, 0.65, 1.0])
        } else {
            (circle(*p, hs / 2.0), [0.025, 0.4, 0.75, 1.0])
        };
        let cov = fill(w, h, &[shape]);
        paint(&mut img, &cov, color::premul(col, 1.0));
    }
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "猫愛（znd 移植: znd-plugins）", version: "r3", license: "MIT",
    effects: [
        {
            id: "NeAc.BezierPath", name: "BezierPath（ガイド）", category: "配置", flags: THREAD_SAFE,
            params: [
                float(c"order", c"次数", 3.0, 1.0, 9.0),
                float(c"offset", c"オフセット", 0.0, -1000.0, 1000.0),
                float(c"space", c"字間倍率", 100.0, -9000.0, 9000.0),
                float(c"rz", c"回転", 0.0, -180.0, 180.0),
                boolean(c"guide", c"ガイド表示", true),
                float(c"line_size", c"線幅", 5.0, 1.0, 1000.0),
                float(c"handle_size", c"ハンドルサイズ", 30.0, 1.0, 1000.0),
                point(c"p1", c"点1", 0.2, 0.7),
                point(c"p2", c"点2", 0.35, 0.2),
                point(c"p3", c"点3", 0.65, 0.8),
                point(c"p4", c"点4", 0.8, 0.3),
                point(c"p5", c"点5", 0.5, 0.5),
                point(c"p6", c"点6", 0.5, 0.5),
                point(c"p7", c"点7", 0.5, 0.5),
                point(c"p8", c"点8", 0.5, 0.5),
                point(c"p9", c"点9", 0.5, 0.5),
                point(c"p10", c"点10", 0.5, 0.5),
                boolean(c"fix", c"向く", true),
            ],
            render: render,
        },
    ]
}
