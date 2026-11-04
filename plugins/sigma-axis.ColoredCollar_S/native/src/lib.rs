//! ColoredCollar_S（σ軸）の移植: 境界の色を外へ引き延ばしたり、距離に応じた色のグラデーションで縁取る。
//! 最寄りの不透明画素はジャンプフラッド法（元と同じ）で求める。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

const NONE: (i32, i32) = (i32::MIN / 4, i32::MIN / 4);

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let thick = (f.f(0).clamp(0.0, 500.0) * s) as f32;
    let blur = (f.f(1) / 100.0).clamp(0.0, 1.0) as f32;
    let mut col_blur = (f.f(2) / 100.0).clamp(0.0, 1.0) as f32;
    let thr = (f.f(3) / 100.0).clamp(2f64.powi(-24), 1.0 - 2f64.powi(-13)) as f32;
    let fixed = f.b(4);
    let c1 = f.color(5);
    let a1 = (f.f(6) / 100.0).clamp(0.0, 1.0) as f32;
    let (c2, a2) = if f.b(7) { (f.color(8), (f.f(9) / 100.0).clamp(0.0, 1.0) as f32) } else { (c1, a1) };
    let alpha_back = (1.0 - f.f(10) / 100.0).clamp(0.0, 1.0) as f32;
    let alpha_fore = (1.0 - f.f(11) / 100.0).clamp(0.0, 1.0) as f32;
    let mollify = (f.f(12) / 100.0).clamp(0.0, 1.0) as f32;
    let src = f.input();
    let pad = if fixed { 0 } else { thick.ceil() as usize };
    if thick <= 0.0 || alpha_back <= 0.0 {
        let mut out = src.padded(pad, pad, pad, pad);
        out.data.iter_mut().for_each(|v| *v *= alpha_fore);
        f.output(out, -(pad as i32), -(pad as i32));
        return Ok(());
    }
    // 縁の色は表示値で計算する（元のシェーダーと同じ）
    let disp = |c: [f32; 4], a: f32| {
        let d = to_display([c[0], c[1], c[2], 1.0]);
        [d[0] * a, d[1] * a, d[2] * a, a]
    };
    let (f1, f2) = (disp(c1, a1), disp(c2, a2));
    if a1 >= 1.0 && a2 >= 1.0 {
        col_blur = 0.0;
    }
    let img = src.padded(pad, pad, pad, pad);
    let (w, h) = (img.w, img.h);
    // ジャンプフラッド: 各画素に最寄りの「中身」の画素
    let mut near: Vec<(i32, i32)> = (0..w * h).map(|i| if img.data[i * 4 + 3] >= thr { ((i % w) as i32, (i / w) as i32) } else { NONE }).collect();
    let mut step = (w.max(h) as f32 / 2.0).log2().ceil().max(0.0) as i32;
    loop {
        let len = 1i32 << step;
        let prev = near.clone();
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let mut best = prev[(y as usize) * w + x as usize];
                let mut bd = if best == NONE { i64::MAX } else { ((best.0 - x) as i64).pow(2) + ((best.1 - y) as i64).pow(2) };
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let (sx, sy) = (x + dx * len, y + dy * len);
                        if sx < 0 || sy < 0 || sx >= w as i32 || sy >= h as i32 {
                            continue;
                        }
                        let p = prev[sy as usize * w + sx as usize];
                        if p == NONE {
                            continue;
                        }
                        let d = ((p.0 - x) as i64).pow(2) + ((p.1 - y) as i64).pow(2);
                        if d < bd {
                            bd = d;
                            best = p;
                        }
                    }
                }
                near[y as usize * w + x as usize] = best;
            }
        }
        if step == 0 {
            break;
        }
        step -= 1;
    }
    // 距離と、最寄りの画素のまわりの色（表示値・ストレート）
    let straight = |x: i32, y: i32| -> [f32; 4] {
        let d = to_display(img.get(x as isize, y as isize));
        [d[0] * d[3], d[1] * d[3], d[2] * d[3], d[3]]
    };
    let mut map: Vec<[f32; 4]> = near
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            if p == NONE {
                return [0.0, 0.0, 0.0, 1e9];
            }
            let (x, y) = ((i % w) as i32, (i / w) as i32);
            let mut c = [0.0f32; 4];
            for dy in -1..=1i32 {
                for dx in -1..=1i32 {
                    let wt = ((2 - dx.abs()) * (2 - dy.abs())) as f32;
                    let q = straight(p.0 + dx, p.1 + dy);
                    for k in 0..4 {
                        c[k] += q[k] * wt;
                    }
                }
            }
            let rgb = if c[3] > 0.0 { [c[0] / c[3], c[1] / c[3], c[2] / c[3]] } else { [0.0; 3] };
            [rgb[0], rgb[1], rgb[2], (((p.0 - x) as f32).powi(2) + ((p.1 - y) as f32).powi(2)).sqrt()]
        })
        .collect();
    // 錯視軽減: 距離と色を箱型にならす（縦横 2 回ずつ）
    if mollify > 0.0 {
        let r = (mollify * thick).round() as usize;
        if r > 0 {
            let mut buf = Image { w, h, data: map.iter().flatten().copied().collect() };
            buf.box_blur(r);
            buf.box_blur(r);
            map = buf.data.chunks_exact(4).map(|c| [c[0], c[1], c[2], c[3]]).collect();
        }
    }
    let rate = (col_blur / ((thick + 12.0) / 1024.0)).sqrt();
    let inv_blur = 1.0 / (thick * blur).max(if blur > 0.0 { 1.0 } else { 2f32.powi(-8) });
    let mut out = Image::new(w, h);
    out.map(|x, y, _| {
        let fore = img.px(x, y);
        let d = map[y * w + x][3];
        let a = ((thick - d) * inv_blur).clamp(0.0, 1.0);
        let t = (d / thick).clamp(0.0, 1.0);
        let fill: [f32; 4] = std::array::from_fn(|k| f1[k] + (f2[k] - f1[k]) * t);
        let l = if a > 0.0 && fill[3] < 1.0 { rate * ((d + 1.0).sqrt() - 1.0) } else { 0.0 };
        let wr = -3.0 / ((l + 1.0) * (l + 1.0));
        let n = l.ceil() as isize;
        let mut acc = [0.0f32; 4];
        for dy in -n..=n {
            for dx in -n..=n {
                let (sx, sy) = (x as isize + dx, y as isize + dy);
                if sx < 0 || sy < 0 || sx >= w as isize || sy >= h as isize {
                    continue;
                }
                let wt = (wr * (dx * dx + dy * dy) as f32).exp();
                let m = map[sy as usize * w + sx as usize];
                for k in 0..3 {
                    acc[k] += wt * m[k];
                }
                acc[3] += wt;
            }
        }
        let back_rgb: [f32; 3] = std::array::from_fn(|k| fill[k] + (1.0 - fill[3]) * if acc[3] > 0.0 { acc[k] / acc[3] } else { 0.0 });
        let back_a = (a - fore[3]).max(0.0);
        let back = from_display([back_rgb[0].clamp(0.0, 1.0), back_rgb[1].clamp(0.0, 1.0), back_rgb[2].clamp(0.0, 1.0), back_a]);
        std::array::from_fn(|k| alpha_fore * fore[k] + alpha_back * back[k])
    });
    f.output(out, -(pad as i32), -(pad as i32));
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "sigma-axis.ColoredCollar_S", name: "色付き縁取り（ColoredCollar_S）", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                float(c"thick", c"サイズ", 5.0, 0.0, 500.0),
                float(c"blur", c"ぼかし", 5.0, 0.0, 100.0),
                float(c"col_blur", c"色拡散", 5.0, 0.0, 100.0),
                float(c"threshold", c"αしきい値", 50.0, 0.0, 100.0),
                boolean(c"fixed_size", c"サイズ固定", false),
                color_hex(c"color", c"縁色", 0xffffff),
                float(c"col_alpha", c"色の濃さ", 30.0, 0.0, 100.0),
                boolean(c"use_outer", c"外側の色を別にする", false),
                color_hex(c"color_outer", c"縁色外側", 0xffffff),
                float(c"col_alpha_outer", c"外側の色の濃さ", 30.0, 0.0, 100.0),
                float(c"alpha", c"透明度", 0.0, 0.0, 100.0),
                float(c"front_alpha", c"前景透明度", 0.0, 0.0, 100.0),
                float(c"mollify", c"錯視軽減", 0.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}
