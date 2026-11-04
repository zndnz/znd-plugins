//! シーンライティング（beive60）の移植: 背景の環境光の色を前景に馴染ませる（色の合成・ライトラップ・リムライト）。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

fn blend_ch(f: f32, b: f32, mode: usize) -> f32 {
    match mode {
        0 => f * b,
        1 => 1.0 - (1.0 - f) * (1.0 - b),
        2 => {
            if f < 0.5 {
                2.0 * f * b
            } else {
                1.0 - 2.0 * (1.0 - f) * (1.0 - b)
            }
        }
        _ => (f + b).min(1.0),
    }
}

fn render(f: &mut Frame) -> FxResult {
    let mode = f.i(1);
    let intensity = (f.f(2) / 100.0) as f32;
    let step = (f.f(3) * f.scale()).floor().max(1.0) as usize;
    let edge_w = (f.f(4) * f.scale()).floor().max(1.0) as isize;
    let thr = (f.f(5).floor().max(1.0) / 255.0) as f32;
    let (wrap, rim) = (f.b(6), f.b(7));
    let ang = f.f(8).to_radians();
    let (lx, ly) = (ang.cos() as f32, -ang.sin() as f32);
    let tint = to_display(color::premul(f.color(9), 1.0));
    // 背景の代表色（表示値）
    let mut amb = [0.5f32; 3];
    if let Some(bg) = f.layer_image(0) {
        let (mut sum, mut n) = ([0.0f32; 3], 0.0f32);
        let mut add = |p: [f32; 4]| {
            // 元は 8bit の BGRA（プリマルチプライドの表示値）を読む
            let d = to_display(p);
            for k in 0..3 {
                sum[k] += d[k] * d[3];
            }
            n += 1.0;
        };
        if f.i(10) == 1 {
            for (fx, fy) in [(0.25, 0.25), (0.75, 0.25), (0.5, 0.5), (0.25, 0.75), (0.75, 0.75)] {
                add(bg.px(((bg.w as f32 * fx) as usize).min(bg.w - 1), ((bg.h as f32 * fy) as usize).min(bg.h - 1)));
            }
        } else {
            for y in (0..bg.h).step_by(step) {
                for x in (0..bg.w).step_by(step) {
                    add(bg.px(x, y));
                }
            }
        }
        if n > 0.0 {
            amb = sum.map(|v| v / n);
        }
    }
    let amb = [amb[0] * tint[0], amb[1] * tint[1], amb[2] * tint[2]];
    let src = f.input();
    let (w, h) = (src.w as isize, src.h as isize);
    let alpha = |x: isize, y: isize| src.px(x.clamp(0, w - 1) as usize, y.clamp(0, h - 1) as usize)[3];
    let mut out = src.clone();
    out.map(|x, y, p| {
        let a = p[3];
        if a < thr {
            return p;
        }
        let d = to_display(p);
        let mut rgb = [0.0f32; 3];
        for k in 0..3 {
            rgb[k] = d[k] + (blend_ch(d[k], amb[k], mode) - d[k]) * intensity;
        }
        let (ix, iy) = (x as isize, y as isize);
        for k in 1..=edge_w {
            let (al, ar, au, ad) = (alpha(ix - k, iy), alpha(ix + k, iy), alpha(ix, iy - k), alpha(ix, iy + k));
            if al.min(ar).min(au).min(ad) < thr {
                let em = 1.0 - (k as f32 - 1.0) / edge_w as f32;
                let st = |v: f32| -> f32 { if v >= thr { 1.0 } else { 0.0 } };
                let (nx, ny) = (st(ar) - st(al), st(ad) - st(au));
                if wrap {
                    let ws = em * intensity * 0.5;
                    for k in 0..3 {
                        rgb[k] = (rgb[k] + amb[k] * ws).clamp(0.0, 1.0);
                    }
                }
                let gl = nx.hypot(ny);
                if rim && gl > 0.001 {
                    let r = ((nx / gl) * lx + (ny / gl) * ly).max(0.0) * em * intensity * 0.6;
                    for k in 0..3 {
                        rgb[k] = (rgb[k] + amb[k] * r).clamp(0.0, 1.0);
                    }
                }
                break;
            }
        }
        from_display([rgb[0], rgb[1], rgb[2], a])
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "beive60（znd 移植: znd-plugins）", version: "2", license: "MIT",
    effects: [
        {
            id: "beive60.scene-lighting", name: "シーンライティング", category: "光", flags: THREAD_SAFE,
            params: [
                layer(c"background", c"背景レイヤー"),
                choice(c"blend_mode", c"ブレンドモード", 1, c"乗算|スクリーン|オーバーレイ|加算"),
                float(c"intensity", c"強度", 50.0, 0.0, 100.0),
                float(c"blur_radius", c"サンプリング間隔", 15.0, 1.0, 100.0),
                float(c"edge_width", c"エッジ幅", 10.0, 1.0, 50.0),
                float(c"edge_threshold", c"エッジ閾値", 16.0, 1.0, 255.0),
                boolean(c"wrap", c"ライトラップ", true),
                boolean(c"rim", c"リムライト", true),
                float(c"rim_angle", c"リムライト角度", 45.0, -180.0, 180.0),
                color_hex(c"tint", c"ベースカラー", 0xffffff),
                choice(c"sampling", c"サンプリング", 0, c"平均|中央点"),
            ],
            render: render,
        },
    ]
}
