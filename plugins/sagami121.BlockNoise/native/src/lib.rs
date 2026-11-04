//! BlockNoise（sagami121）の移植: 一定時間ごとに、画像の一部を長方形のブロックとしてずらして重ねるグリッチ。

use znd_plugin_sdk::noise::rand_int;
use znd_plugin_sdk::*;

fn range(f: &Frame, i: usize) -> (i64, i64) {
    let (a, b) = (f.f(i).round() as i64, f.f(i + 1).round() as i64);
    (a.min(b), a.max(b))
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let threshold = f.f(0);
    let change_ms = f.f(1);
    let percent = f.b(2);
    let draw_bg = f.b(13);
    let fixed = f.b(14);
    let a = if change_ms > 0.0 { (f.layer_time() / change_ms * 1000.0).floor() as i64 } else { 0 };
    // 元の obj.rand は seed が負ならオブジェクトによらない。正ならレイヤーごとに変わる
    let base = if fixed { 0 } else { f.seed() as i64 };
    let rand = |lo: i64, hi: i64, seed: i64| rand_int(lo, hi, seed + base, a);
    let r = rand(0, 100, 1);
    let compare = rand(0, 100, 1 + r);
    let src = f.input();
    let (w, h) = (src.w as i64, src.h as i64);
    let mut out = if draw_bg { src.clone() } else { Image::new(src.w, src.h) };
    if threshold < compare as f64 {
        let (k0, k1) = range(f, 3);
        let count = rand(k0, k1, 2 + r);
        let (xs0, xs1) = range(f, 5);
        let (ys0, ys1) = range(f, 7);
        let (zx0, zx1) = range(f, 9);
        let (zy0, zy1) = range(f, 11);
        for i in 1..=count {
            // サイズ・位置はレイヤー座標の px。描くときに縮尺を掛ける
            let xs = rand(xs0, xs1, 3 + i + r) as f64;
            let ys = rand(ys0, ys1, 4 + i + r) as f64;
            let lw = (w as f64 / s) as i64;
            let lh = (h as f64 / s) as i64;
            let x = rand(-lw / 2, lw / 2, 5 + i + r) as f64;
            let y = rand(-lh / 2, lh / 2, 6 + i + r) as f64;
            let (dx, dy) = if percent {
                (xs * rand(zx0, zx1, 7 + i + r) as f64 / 100.0, ys * rand(zy0, zy1, 8 + i + r) as f64 / 100.0)
            } else {
                (rand(zx0, zx1, 7 + i + r) as f64, rand(zy0, zy1, 8 + i + r) as f64)
            };
            let (bw, bh) = ((xs * s).round() as isize, (ys * s).round() as isize);
            let sx = ((x - xs / 2.0) * s + w as f64 / 2.0).round() as isize;
            let sy = ((y - ys / 2.0) * s + h as f64 / 2.0).round() as isize;
            let block = src.crop(sx, sy, bw.max(0) as usize, bh.max(0) as usize);
            out.draw(&block, sx + (dx * s).round() as isize, sy + (dy * s).round() as isize, 1.0);
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "sagami121（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "sagami121.BlockNoise", name: "ブロックノイズ", category: "スタイライズ", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"threshold", c"しきい値", 80.0, 0.0, 100.0),
                float(c"change_ms", c"変化（ms）", 100.0, 0.0, 5000.0),
                boolean(c"percent", c"ずれ%指定", false),
                float(c"count_min", c"表示個数（最小）", 5.0, 0.0, 1000.0),
                float(c"count_max", c"表示個数（最大）", 15.0, 0.0, 1000.0),
                float(c"xsize_min", c"Xサイズ（最小）", 300.0, 0.0, 10000.0),
                float(c"xsize_max", c"Xサイズ（最大）", 500.0, 0.0, 10000.0),
                float(c"ysize_min", c"Yサイズ（最小）", 10.0, 0.0, 10000.0),
                float(c"ysize_max", c"Yサイズ（最大）", 100.0, 0.0, 10000.0),
                float(c"xzure_min", c"Xずれ（最小）", -30.0, -10000.0, 10000.0),
                float(c"xzure_max", c"Xずれ（最大）", 30.0, -10000.0, 10000.0),
                float(c"yzure_min", c"Yずれ（最小）", -30.0, -10000.0, 10000.0),
                float(c"yzure_max", c"Yずれ（最大）", 30.0, -10000.0, 10000.0),
                boolean(c"draw_background", c"背景描画", true),
                boolean(c"fixed_seed", c"乱数固定", false),
            ],
            render: render,
        },
    ]
}
