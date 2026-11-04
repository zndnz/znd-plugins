//! 油絵（Karoterra）の移植: 桑原フィルタ。注目画素の左上・右上・左下・右下の 4 つの正方形のうち、
//! 輝度の分散が最も小さい領域の平均色で置き換える。

use znd_plugin_sdk::color::to_display;
use znd_plugin_sdk::*;

fn value(p: [f32; 4], mode: usize) -> f32 {
    // 元のシェーダーと同じく、ストレートの表示値で輝度を取る
    let d = to_display(p);
    match mode {
        1 => (d[0] + d[1] + d[2]) / 3.0,
        2 => d[0].max(d[1]).max(d[2]),
        _ => d[0] * 0.299 + d[1] * 0.587 + d[2] * 0.114,
    }
}

fn render(f: &mut Frame) -> FxResult {
    let bs = f.f(0).round().max(0.0) as isize;
    if bs == 0 {
        return Ok(());
    }
    let mode = f.i(1);
    let src = f.input();
    let (w, h) = (src.w, src.h);
    // 輝度と色の積分画像（領域の和を O(1) で求める）
    let iw = w + 1;
    let mut sc = vec![[0.0f64; 4]; iw * (h + 1)];
    let mut sv = vec![[0.0f64; 2]; iw * (h + 1)];
    for y in 0..h {
        for x in 0..w {
            let p = src.px(x, y);
            let v = value(p, mode) as f64;
            let i = (y + 1) * iw + x + 1;
            for k in 0..4 {
                sc[i][k] = p[k] as f64 + sc[i - 1][k] + sc[i - iw][k] - sc[i - iw - 1][k];
            }
            sv[i] = [v + sv[i - 1][0] + sv[i - iw][0] - sv[i - iw - 1][0], v * v + sv[i - 1][1] + sv[i - iw][1] - sv[i - iw - 1][1]];
        }
    }
    let n = ((bs + 1) * (bs + 1)) as f64;
    let mut out = Image::new(w, h);
    for y in 0..h as isize {
        for x in 0..w as isize {
            let mut best = [0.0f32; 4];
            let mut best_var = f64::INFINITY;
            for (ox, oy) in [(-bs, -bs), (0, -bs), (-bs, 0), (0, 0)] {
                // 画像の外は端の画素（元のシェーダーの clamp と同じ）。積分画像では外側の分を端の画素で数え直す
                let (x0, y0) = (x + ox, y + oy);
                let (mut c, mut v) = ([0.0f64; 4], [0.0f64; 2]);
                let inside = x0 >= 0 && y0 >= 0 && x0 + bs < w as isize && y0 + bs < h as isize;
                if inside {
                    let (a, b) = (x0 as usize, y0 as usize);
                    let (c1, d1) = (a + bs as usize + 1, b + bs as usize + 1);
                    let at = |xx: usize, yy: usize| yy * iw + xx;
                    for k in 0..4 {
                        c[k] = sc[at(c1, d1)][k] - sc[at(a, d1)][k] - sc[at(c1, b)][k] + sc[at(a, b)][k];
                    }
                    for k in 0..2 {
                        v[k] = sv[at(c1, d1)][k] - sv[at(a, d1)][k] - sv[at(c1, b)][k] + sv[at(a, b)][k];
                    }
                } else {
                    for yy in 0..=bs {
                        for xx in 0..=bs {
                            let p = src.get_clamped(x0 + xx, y0 + yy);
                            let val = value(p, mode) as f64;
                            for k in 0..4 {
                                c[k] += p[k] as f64;
                            }
                            v[0] += val;
                            v[1] += val * val;
                        }
                    }
                }
                let mean = v[0] / n;
                let var = v[1] / n - mean * mean;
                if var <= best_var {
                    best_var = var;
                    best = [(c[0] / n) as f32, (c[1] / n) as f32, (c[2] / n) as f32, (c[3] / n) as f32];
                }
            }
            out.set(x as usize, y as usize, best);
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "Karoterra（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "Karoterra.OilPainting", name: "油絵", category: "スタイライズ", flags: THREAD_SAFE,
            params: [pixels(c"size", c"ぼかし", 10.0, 0.0, 100.0), choice(c"luma_mode", c"輝度モード", 0, c"加重平均|平均|最大値")],
            render: render,
        },
    ]
}
