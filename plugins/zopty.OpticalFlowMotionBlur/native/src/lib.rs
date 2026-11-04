//! オプティカルフローモーションブラー（zopty）の移植: 前のフレームとの明るさの変化から動き（Lucas–Kanade 法）を求め、
//! その向きに沿ってぼかす疑似モーションブラー。式は元のシェーダーと同じ（フローは画面に対する割合）。
//! 元は前のフレームの絵とフローを覚えて混ぜていくが、ここでは毎回 1 つ前と 2 つ前のフレームから求めて混ぜる。

use znd_plugin_sdk::*;

fn luma(img: &Image) -> Vec<f64> {
    img.data.chunks_exact(4).map(|p| {
        let d = color::to_display([p[0], p[1], p[2], p[3]]);
        let a = d[3] as f64;
        (0.299 * d[0] as f64 + 0.587 * d[1] as f64 + 0.114 * d[2] as f64) * a
    }).collect()
}

/// 現フレームと前フレームの明るさから、画素ごとのフロー（画面に対する割合）を求める。
fn flow(cur: &[f64], prev: &[f64], w: usize, h: usize, r: usize, gs: f64, maxf: f64, eps: f64) -> Vec<[f64; 2]> {
    let at = |v: &[f64], x: isize, y: isize| v[(y.clamp(0, h as isize - 1) as usize) * w + x.clamp(0, w as isize - 1) as usize];
    let n = w * h;
    let mut terms = vec![[0.0f64; 5]; n];
    for y in 0..h as isize {
        for x in 0..w as isize {
            let ix = (at(cur, x + 1, y) - at(cur, x - 1, y)) * 0.5 * gs;
            let iy = (at(cur, x, y + 1) - at(cur, x, y - 1)) * 0.5 * gs;
            let it = at(cur, x, y) - at(prev, x, y);
            terms[y as usize * w + x as usize] = [ix * ix, iy * iy, ix * iy, ix * it, iy * it];
        }
    }
    // 窓の合計（端は端の画素をくり返す。元のサンプラーの clamp と同じ）を、横 → 縦の累積和で求める
    let r = r as isize;
    let mut tmp = vec![[0.0f64; 5]; n];
    for y in 0..h {
        for x in 0..w as isize {
            let mut s = [0.0; 5];
            for dx in -r..=r {
                let t = terms[y * w + (x + dx).clamp(0, w as isize - 1) as usize];
                for k in 0..5 {
                    s[k] += t[k];
                }
            }
            tmp[y * w + x as usize] = s;
        }
    }
    let mut out = vec![[0.0f64; 2]; n];
    for y in 0..h as isize {
        for x in 0..w {
            let mut s = [0.0; 5];
            for dy in -r..=r {
                let t = tmp[(y + dy).clamp(0, h as isize - 1) as usize * w + x];
                for k in 0..5 {
                    s[k] += t[k];
                }
            }
            let det = s[0] * s[1] - s[2] * s[2];
            let mut fl = [0.0, 0.0];
            if det.abs() > eps {
                fl = [(-s[1] * s[3] + s[2] * s[4]) / det, (s[2] * s[3] - s[0] * s[4]) / det];
            }
            let l = fl[0].hypot(fl[1]);
            if l > maxf {
                fl = [fl[0] * maxf / l.max(1e-5), fl[1] * maxf / l.max(1e-5)];
            }
            out[y as usize * w + x] = fl;
        }
    }
    out
}

fn render(f: &mut Frame) -> FxResult {
    let scale = f.f(0) / 100.0;
    let hist = (f.f(1) / 100.0).clamp(0.0, 1.0);
    let show = f.b(2);
    let maxf = f.f(3);
    let samples = f.f(4).round().clamp(4.0, 64.0) as usize;
    let r = (f.f(5).round().max(1.0) * f.scale()).round().max(1.0) as usize;
    let eps = f.f(6);
    let gs = f.f(7);
    let cur = f.input();
    let (w, h) = (cur.w, cur.h);
    let dt = 1.0 / f.fps().max(1.0);
    let t = f.layer_time();
    let same = |im: Option<Image>| im.filter(|i| i.w == w && i.h == h);
    let Some(prev) = same(f.frame_at(f.time() - dt)) else { return Ok(()) };
    let lc = luma(&cur);
    let lp = luma(&prev);
    let mut fl = flow(&lc, &lp, w, h, r, gs, maxf, eps);
    // 前のフローを混ぜる（元の「速度」。1 つ前のフレームでのフローで代わりにする）
    if hist > 0.0 && t >= 2.0 * dt {
        if let Some(prev2) = same(f.frame_at(f.time() - 2.0 * dt)) {
            let fp = flow(&lp, &luma(&prev2), w, h, r, gs, maxf, eps);
            for (a, b) in fl.iter_mut().zip(&fp) {
                *a = [a[0] * (1.0 - hist) + b[0] * hist, a[1] * (1.0 - hist) + b[1] * hist];
            }
        }
    }
    if show {
        let img = Image::generate(w, h, |x, y| {
            let v = fl[y * w + x];
            [v[0].max(0.0) as f32, v[1].max(0.0) as f32, 0.0, 1.0]
        });
        f.set(img);
        return Ok(());
    }
    let out = Image::generate(w, h, |x, y| {
        let v = fl[y * w + x];
        let d = [v[0] * scale * w as f64, v[1] * scale * h as f64];
        let mut acc = [0.0f32; 4];
        let mut ws = 0.0f32;
        for i in 0..samples {
            let tt = i as f64 / (samples as f64 - 1.0) - 0.5;
            let wt = (1.0 - tt.abs() * 2.0) as f32;
            let p = cur.sample_clamped((x as f64 + 0.5 + d[0] * tt) as f32, (y as f64 + 0.5 + d[1] * tt) as f32);
            for k in 0..4 {
                acc[k] += p[k] * wt;
            }
            ws += wt;
        }
        acc.map(|v| v / ws.max(1e-4))
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "zopty（znd 移植: znd-plugins）", version: "1.0.0", license: "BSD-3-Clause",
    effects: [
        {
            id: "zopty.OpticalFlowMotionBlur", name: "オプティカルフローモーションブラー", category: "ぼかし", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"blur_scale", c"強度", 10.0, 0.0, 100.0),
                float(c"history_blend", c"速度", 50.0, 0.0, 100.0),
                boolean(c"show_flow", c"フロー表示", false),
                float(c"max_flow", c"最大フロー長", 0.05, 0.0, 1.0),
                float(c"sample_count", c"サンプル数", 64.0, 4.0, 64.0),
                float(c"kernel_radius", c"カーネル半径", 8.0, 1.0, 10.0),
                float(c"epsilon", c"しきい値", 0.1, 0.0001, 1.0),
                float(c"gradient_scale", c"勾配スケール", 120.0, 0.1, 200.0),
            ],
            render: render,
        },
    ]
}
