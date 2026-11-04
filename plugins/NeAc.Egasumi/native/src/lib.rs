//! エ霞（猫愛）の移植: 日本画の「エ霞（すやり霞）」の形の雲を描く。
//! 雲 1 つにつき 2 点（左上・右下）、雲と雲の間のつなぎに 1 点。つなぎの両脇は丸くえぐる。
//! 平面などに掛けると、レイヤーの絵を雲で置き換える（雲の点はレイヤー上の位置。ビューアでドラッグできる）。

use znd_plugin_sdk::draw::{circle, fill, fill_union};
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

const P0: usize = 6;

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Vec<V2>> {
    let (x0, x1) = (x0.min(x1), x0.max(x1));
    let (y0, y1) = (y0.min(y1), y0.max(y1));
    vec![vec![V2::new(x0, y0), V2::new(x1, y0), V2::new(x1, y1), V2::new(x0, y1)]]
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let n = f.f(0).round().clamp(1.0, 5.0) as usize;
    let line = f.f(1) * s / 2.0;
    let col = color::premul(f.color(2), 1.0);
    let fix = f.b(3);
    let (cloud_h, connect_h) = (f.f(4) * s, f.f(5) * s);
    let count = 2 + (n - 1) * 3;
    let (cx, cy) = (f.width as f64 / 2.0, f.height as f64 / 2.0);
    // 点はレイヤー中心からの相対位置にする（元のスクリプトと同じ座標系）
    let mut p: Vec<V2> = (0..count).map(|k| { let q = f.point(P0 + k); V2::new(q[0] - cx, q[1] - cy) }).collect();
    if fix {
        let layout = |p: &mut Vec<V2>| {
            for k in 1..count {
                let num = k + 1; // 1 始まりの点の番号
                p[k].y = p[k - 1].y + if num % 3 == 0 { 0.0 } else if num % 3 == 1 { connect_h } else { cloud_h };
            }
        };
        layout(&mut p);
        p[0].y = -(p[count - 1].y - p[0].y) / 2.0;
        layout(&mut p);
        let (lo, hi) = p.iter().fold((f64::MAX, f64::MIN), |(a, b), q| (a.min(q.x), b.max(q.x)));
        let avg = hi - (hi - lo) / 2.0;
        p.iter_mut().for_each(|q| q.x -= avg);
    }
    // 雲: 長方形と、両端の円
    let mut shapes = Vec::new();
    for c in 0..n {
        let (a, b) = (p[3 * c], p[3 * c + 1]);
        let r = ((a.y - b.y).abs() + 2.0 * s) / 2.0;
        let mid = (a.y + b.y) / 2.0;
        shapes.push(vec![circle(V2::new(a.x, mid), r)]);
        shapes.push(vec![circle(V2::new(b.x, mid), r)]);
        shapes.push(rect(a.x, a.y, b.x, b.y));
    }
    // つなぎと、両脇のえぐり（反転した円マスク）
    let mut holes = Vec::new();
    for c in 0..n.saturating_sub(1) {
        let (a0, b0, cc, a1, b1) = (p[3 * c], p[3 * c + 1], p[3 * c + 2], p[3 * c + 3], p[3 * c + 4]);
        let plus = b0.x.min(b1.x);
        let minus = a0.x.max(a1.x);
        let (top, bottom) = (b0.y - s, a1.y + s);
        let mid = (b0.y + a1.y) / 2.0;
        let d = (a1.y - b0.y).abs() + 3.0 * s;
        let mut x = cc.x;
        if plus - minus < line * 2.0 {
            shapes.push(rect(minus, top, plus, bottom));
            if line > 0.0 {
                holes.push(circle(V2::new(minus, mid), d / 2.0));
                holes.push(circle(V2::new(plus, mid), d / 2.0));
            }
            continue;
        }
        if x - line < a0.x || x - line < a1.x {
            x = minus + line;
        } else if x + line > b0.x || x + line > b1.x {
            x = plus - line;
        }
        shapes.push(rect(x - line, top, x + line, bottom));
        if line > 0.0 {
            holes.push(circle(V2::new(x - line, mid), d / 2.0));
            holes.push(circle(V2::new(x + line, mid), d / 2.0));
        }
    }
    // 外接矩形の大きさで描く
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for sh in &shapes {
        for q in sh.iter().flatten() {
            x0 = x0.min(q.x);
            y0 = y0.min(q.y);
            x1 = x1.max(q.x);
            y1 = y1.max(q.y);
        }
    }
    let (ox, oy) = ((x0 - 2.0).floor(), (y0 - 2.0).floor());
    let (w, h) = (((x1 + 2.0).ceil() - ox) as usize, ((y1 + 2.0).ceil() - oy) as usize);
    let shift = |sh: &Vec<Vec<V2>>| -> Vec<Vec<V2>> { sh.iter().map(|c| c.iter().map(|q| V2::new(q.x - ox, q.y - oy)).collect()).collect() };
    let shapes: Vec<Vec<Vec<V2>>> = shapes.iter().map(shift).collect();
    let mut cov = fill_union(w, h, &shapes);
    for hole in &holes {
        let hc = fill(w, h, &shift(&vec![hole.clone()]));
        cov.iter_mut().zip(hc).for_each(|(c, m)| *c *= 1.0 - m);
    }
    let mut img = Image::new(w, h);
    img.data.chunks_exact_mut(4).zip(&cov).for_each(|(px, &c)| px.copy_from_slice(&col.map(|v| v * c)));
    f.output(img, (ox + cx).round() as i32, (oy + cy).round() as i32);
    Ok(())
}

plugin! {
    vendor: "猫愛（znd 移植: znd-plugins）", version: "r2", license: "MIT",
    effects: [
        {
            id: "NeAc.Egasumi", name: "エ霞", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"cloud_amount", c"雲個数", 5.0, 1.0, 5.0),
                float(c"line_width", c"繋ぎ太さ", 110.0, 0.0, 4000.0),
                color_hex(c"color", c"色", 0xffffff),
                boolean(c"height_fix", c"高さ統一", true),
                float(c"cloud_height", c"雲高さ", 40.0, 1.0, 200.0),
                float(c"connect_height", c"繋ぎ高さ", 40.0, 1.0, 200.0),
                point(c"p1", c"点1", 0.343229, 0.333333),
                point(c"p2", c"点2", 0.563021, 0.370370),
                point(c"p3", c"点3", 0.517708, 0.370370),
                point(c"p4", c"点4", 0.428646, 0.407407),
                point(c"p5", c"点5", 0.701042, 0.444444),
                point(c"p6", c"点6", 0.577604, 0.444444),
                point(c"p7", c"点7", 0.374479, 0.481481),
                point(c"p8", c"点8", 0.627604, 0.518519),
                point(c"p9", c"点9", 0.423958, 0.518519),
                point(c"p10", c"点10", 0.298958, 0.555556),
                point(c"p11", c"点11", 0.585938, 0.592593),
                point(c"p12", c"点12", 0.496354, 0.592593),
                point(c"p13", c"点13", 0.398958, 0.629630),
                point(c"p14", c"点14", 0.669792, 0.666667),
            ],
            render: render,
        },
    ]
}
