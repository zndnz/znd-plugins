//! 連結ライン（arcana）の移植: 2 点（2 つのレイヤーの位置）を線でつなぐ。実線・点線（流れる）、平ら・丸い端、始点・終点の空き、表示割合。
//! 端点はレイヤー上の点のパラメータ。2 つのレイヤーの位置へのつなぎ込みはパッケージのコマンドがエクスプレッションで行う。

use znd_plugin_sdk::draw::{circle, fill_union, paint, segment};
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let (a, b) = (f.point(0), f.point(1));
    let (p1, p2) = (V2::new(a[0], a[1]), V2::new(b[0], b[1]));
    let width = f.f(2) * s;
    let col = color::premul(f.color(3), 1.0);
    let dashed = f.i(4) == 1;
    let (dash, gap) = (f.f(5).max(1.0) * s, f.f(6).max(1.0) * s);
    let round = f.i(7) == 1;
    let (trim1, trim2) = (f.f(8) * s, f.f(9) * s);
    let progress = f.f(10).clamp(0.0, 100.0) / 100.0;
    let flow = f.f(11) * s;
    let d = p2.sub(p1);
    let dist = d.len();
    let mut img = Image::new(f.width, f.height);
    let s_start = trim1;
    let s_end = s_start + (dist - trim2 - s_start) * progress;
    if dist > 0.0 && s_end > s_start {
        let u = d.mul(1.0 / dist);
        let at = |t: f64| p1.add(u.mul(t));
        let half = width / 2.0;
        let mut shapes = Vec::new();
        let mut piece = |t0: f64, t1: f64| {
            if t1 <= t0 {
                return;
            }
            // 丸端でも見える長さは t1 - t0 のまま
            let r = if round { half.min((t1 - t0) / 2.0) } else { 0.0 };
            let (c0, c1) = (t0 + r, t1 - r);
            if c1 > c0 {
                shapes.push(vec![segment(at(c0), at(c1), width)]);
            }
            if r > 0.0 {
                shapes.push(vec![circle(at(c0), r)]);
                shapes.push(vec![circle(at(c1), r)]);
            }
        };
        if !dashed {
            piece(s_start, s_end);
        } else {
            let period = dash + gap;
            let phase = (flow * f.time()).rem_euclid(period);
            let mut t = s_start + phase - period;
            let mut n = 0;
            while t < s_end && n < 100_000 {
                piece(t.max(s_start), (t + dash).min(s_end));
                t += period;
                n += 1;
            }
        }
        let cov = fill_union(img.w, img.h, &shapes);
        paint(&mut img, &cov, col);
    }
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "arcana（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "arcana.connection_line", name: "連結ライン", category: "描画", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                point(c"start", c"始点", 0.3, 0.5),
                point(c"end", c"終点", 0.7, 0.5),
                float(c"width", c"太さ", 10.0, 1.0, 500.0),
                color_hex(c"line_color", c"ラインの色", 0xffffff),
                choice(c"line_style", c"ラインの種類", 0, c"実線|点線"),
                float(c"dash_length", c"点線の長さ", 20.0, 1.0, 200.0),
                float(c"gap_length", c"点線の間隔", 10.0, 1.0, 200.0),
                choice(c"cap_style", c"端の形", 0, c"平ら|丸"),
                float(c"trim1", c"始点の空き", 0.0, 0.0, 1000.0),
                float(c"trim2", c"終点の空き", 0.0, 0.0, 1000.0),
                float(c"progress", c"表示割合", 100.0, 0.0, 100.0),
                float(c"flow", c"点線の流れ", 0.0, -1000.0, 1000.0),
            ],
            render: render,
        },
    ]
}
