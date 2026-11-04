//! スピード線（猫愛）の移植: 画面の上下（または左右）に、流れる横線（縦線）を描く。
//! 元は「背景」を「斜めクリッピング」で中央の帯を抜き、「ノイズ」（周期 X を小さく、Y を大きくして筋にする）を
//! アルファに掛けて、濃さ・薄さでしきい値を掛け、単色化する。ここでは同じ流れを 1 つのエフェクトで行う。

use znd_plugin_sdk::noise::value3;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let vertical = f.b(5);
    let (w, h) = (f.width as f64, f.height as f64);
    // 線の帯の幅（両側の合計）。割合なら 0..4000 を画面の高さ（幅）に対する割合として読む
    let side = if vertical { w } else { h };
    let band = if f.b(12) { f.f(0) / 4000.0 * side } else { f.f(0) * s };
    let clear_half = ((side - band) / 2.0).max(0.0);
    let blur = (f.f(7) * s).max(0.0);
    let threshold = (f.f(1) / 100.0) as f32;
    let a1 = (f.f(2) / 100.0) as f32;
    let a2 = (1.0 - a1) * (1.0 - (f.f(3) / 100.0) as f32) + a1;
    let speed = f.f(6) * s;
    let (px, py) = (f.f(8) / 100.0 / s, f.f(9) / 100.0 / s);
    let seed = f.f(10) as u32;
    let invert = f.b(11);
    let col = f.color(4);
    let t = f.layer_time();
    let wi = f.width;
    for (i, p) in f.rgba.chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i % wi) as f64 + 0.5, (i / wi) as f64 + 0.5);
        // 線の向きに沿う座標 u、直交する座標 v
        let (u, v, c) = if vertical { (y, x, w / 2.0) } else { (x, y, h / 2.0) };
        let d = (v - c).abs() - clear_half;
        let edge = if blur > 0.0 { (d / blur + 0.5).clamp(0.0, 1.0) } else if d > 0.0 { 1.0 } else { 0.0 } as f64;
        let mut n = value3(((u - speed * t) * px) as f32, (v * py) as f32, 0.0, seed);
        n = ((n - threshold) / (1.0 - threshold).max(1e-4)).clamp(0.0, 1.0);
        if invert {
            n = 1.0 - n;
        }
        let a = n * edge as f32;
        // 濃さ・薄さ: 透明度がしきい値より小さければ不透明、大きければ透明
        let tr = 1.0 - a;
        let tr = if tr < a1 { 0.0 } else if tr > a2 { 1.0 } else { tr };
        let a = 1.0 - tr;
        p.copy_from_slice(&color::premul(col, a));
    }
    Ok(())
}

plugin! {
    vendor: "猫愛（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "NeAc.SpeedLine", name: "スピード線", category: "描画", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"width", c"幅", 300.0, 0.0, 4000.0),
                float(c"threshold", c"しきい値", 20.0, 0.0, 100.0),
                float(c"thickness", c"濃さ", 80.0, 0.0, 100.0),
                float(c"thinness", c"薄さ", 0.0, 0.0, 100.0),
                color_hex(c"color", c"色", 0xffffff),
                boolean(c"vertical", c"縦方向", false),
                float(c"speed", c"速度", 20.0, -800.0, 800.0),
                float(c"blur", c"ぼかし", 150.0, 0.0, 1000.0),
                float(c"period_x", c"周期X", 0.1, 0.0, 1000.0),
                float(c"period_y", c"周期Y", 6.0, 0.0, 1000.0),
                float(c"seed", c"シード", 0.0, 0.0, 1000.0),
                boolean(c"invert_alpha", c"透明度反転", true),
                boolean(c"width_per", c"幅を割合に", false),
            ],
            render: render,
        },
    ]
}
