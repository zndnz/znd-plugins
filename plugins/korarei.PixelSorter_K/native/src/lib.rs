//! PixelSorter_K（korarei）の移植: 明るさなどが範囲内の画素が続く区間ごとに、その値で並べ替える（ピクセルソート）。
//! 方向を指定すると、画像を回してから行ごとに並べ替え、回し戻す。

use znd_plugin_sdk::color::to_display;
use znd_plugin_sdk::*;

fn metric(rgb: [f32; 3], mode: usize) -> f32 {
    match mode {
        0 => rgb[0] * 0.299 + rgb[1] * 0.587 + rgb[2] * 0.114,
        2 => rgb[0] * 0.2627 + rgb[1] * 0.6780 + rgb[2] * 0.0593,
        3 => (rgb[0] + rgb[1] + rgb[2]) / 3.0,
        4 => rgb[0] * rgb[1] * rgb[2],
        5 => rgb[0].min(rgb[1]).min(rgb[2]),
        6 => rgb[0].max(rgb[1]).max(rgb[2]),
        _ => rgb[0] * 0.2126 + rgb[1] * 0.7152 + rgb[2] * 0.0722,
    }
}

/// 中心を軸に `deg` 度回した画像を (w, h) の大きさで作る（画素の中心で逆に回して取る）。
fn rotated(src: &Image, deg: f64, w: usize, h: usize) -> Image {
    let (s, c) = deg.to_radians().sin_cos();
    let (scx, scy) = (src.w as f64 / 2.0, src.h as f64 / 2.0);
    let (dcx, dcy) = (w as f64 / 2.0, h as f64 / 2.0);
    Image::generate(w, h, |x, y| {
        let (dx, dy) = (x as f64 + 0.5 - dcx, y as f64 + 0.5 - dcy);
        let (sx, sy) = (c * dx + s * dy + scx, -s * dx + c * dy + scy);
        src.sample(sx as f32, sy as f32)
    })
}

fn render(f: &mut Frame) -> FxResult {
    let mode = f.i(0);
    let (from, to) = ((f.f(1) / 255.0) as f32, (f.f(2) / 255.0) as f32);
    let dir = f.f(3);
    let srgb = f.i(4) == 1;
    let mask = f.b(5);
    let src = f.input();
    let (w, h) = (src.w, src.h);
    let rot = dir.to_radians();
    let (c, s) = (rot.cos().abs(), rot.sin().abs());
    let turn = dir.rem_euclid(360.0) != 0.0;
    let (mw, mh) = if turn { ((c * w as f64 + s * h as f64).ceil() as usize, (s * w as f64 + c * h as f64).ceil() as usize) } else { (w, h) };
    let mut img = if turn { rotated(&src, -dir, mw, mh) } else { src.clone() };
    let mut row = vec![[0.0f32; 4]; mw];
    let mut keys = vec![(0.0f32, false); mw];
    for y in 0..mh {
        for x in 0..mw {
            let p = img.px(x, y);
            row[x] = p;
            // sRGB を選ぶとリニアの値で、リニアを選ぶと表示値のまま測る（元と同じく、表示値を基準にした呼び方）
            let v = if p[3] <= 1e-4 {
                0.0
            } else if srgb {
                metric([p[0] / p[3], p[1] / p[3], p[2] / p[3]].map(|v| v.clamp(0.0, 1.0)), mode)
            } else {
                let d = to_display(p);
                metric([d[0], d[1], d[2]], mode)
            };
            keys[x] = (v, p[3] > 1e-4 && v >= from && v <= to);
        }
        let mut st = 0;
        for x in 0..=mw {
            if x < mw && keys[x].1 {
                continue;
            }
            if st < x {
                let mut idx: Vec<usize> = (st..x).collect();
                idx.sort_by(|a, b| keys[*a].0.total_cmp(&keys[*b].0));
                for (k, i) in idx.into_iter().enumerate() {
                    img.set(st + k, y, row[i]);
                }
            }
            st = x + 1;
        }
        if mask {
            for x in 0..mw {
                if !keys[x].1 {
                    img.set(x, y, [0.0; 4]);
                }
            }
        }
    }
    f.set(if turn { rotated(&img, dir, w, h) } else { img });
    Ok(())
}

plugin! {
    vendor: "korarei（znd 移植: znd-plugins）", version: "0.1.0", license: "MIT",
    effects: [
        {
            id: "korarei.PixelSorter_K", name: "ピクセルソート（PixelSorter_K）", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                choice(c"sort_by", c"並べ替える値", 1, c"輝度（BT.601）|輝度（BT.709）|輝度（BT.2020）|RGB の平均|RGB の積|RGB の最小|RGB の最大"),
                float(c"sort_from", c"範囲（下限）", 32.0, 0.0, 255.0),
                float(c"sort_to", c"範囲（上限）", 224.0, 0.0, 255.0),
                float(c"direction", c"方向", 0.0, -3600.0, 3600.0),
                choice(c"color_space", c"色空間", 1, c"リニア|sRGB"),
                boolean(c"mask", c"範囲外を透明にする（Mask）", false),
            ],
            render: render,
        },
    ]
}
