//! 墨だまり（saluf）の移植: 文字の角や細い隙間に墨が溜まったように、アルファを膨張 → 収縮（クロージング）して埋め、1 色で塗る。

use znd_plugin_sdk::*;

/// 半径 r・丸め roundness（0: 正方形、1: 円）の近傍の最大値（max = true）／最小値。範囲外は透明。
fn morph(a: &[f32], w: usize, h: usize, r: f32, roundness: f32, max: bool) -> Vec<f32> {
    let ri = r.ceil() as isize;
    let offs: Vec<(isize, isize)> = (-ri..=ri)
        .flat_map(|y| (-ri..=ri).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            let round = ((x * x + y * y) as f32).sqrt();
            let boxd = x.abs().max(y.abs()) as f32;
            boxd + (round - boxd) * roundness <= r
        })
        .collect();
    let mut out = vec![0.0f32; w * h];
    for y in 0..h as isize {
        for x in 0..w as isize {
            let mut m = if max { 0.0f32 } else { 1.0f32 };
            for &(dx, dy) in &offs {
                let (sx, sy) = (x + dx, y + dy);
                let v = if sx < 0 || sy < 0 || sx >= w as isize || sy >= h as isize { 0.0 } else { a[sy as usize * w + sx as usize] };
                m = if max { m.max(v) } else { m.min(v) };
            }
            out[y as usize * w + x as usize] = m;
        }
    }
    out
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale() as f32;
    let r = (f.f(0) as f32 * s).max(0.5);
    let t = (f.f(1) / 100.0) as f32;
    let roundness = (f.f(2) / 100.0) as f32;
    let src = f.input();
    let (w, h) = (src.w, src.h);
    // 塗る色: 指定が無ければ、いちばん不透明な画素の色（元はテキストの文字色を使う）
    let col = if f.b(3) {
        f.color(4)
    } else {
        let best = src.data.chunks_exact(4).fold([0.0f32; 4], |b, p| if p[3] > b[3] { [p[0], p[1], p[2], p[3]] } else { b });
        if best[3] > 0.0 {
            [best[0] / best[3], best[1] / best[3], best[2] / best[3], 1.0]
        } else {
            [1.0; 4]
        }
    };
    let mask: Vec<f32> = src.data.chunks_exact(4).map(|p| smoothstep(t - 0.25, t + 0.25, p[3])).collect();
    let mut a = morph(&mask, w, h, r, roundness, true);
    a = morph(&a, w, h, r, roundness, false);
    // 継ぎ目の修正（3px の円で同じことをもう一度）
    let seam = 3.0 * s;
    a = morph(&a, w, h, seam, 1.0, true);
    a = morph(&a, w, h, seam, 1.0, false);
    let mut out = Image::new(w, h);
    for (p, &v) in out.data.chunks_exact_mut(4).zip(&a) {
        // 輪郭を少し締めて、ほぼベタ塗りにする（元の solidifyAlpha と同じ値）
        let soft = smoothstep(0.01, 0.08, v);
        let hard = if soft > 0.001 { 1.0 } else { 0.0 };
        let v = soft + (hard - soft) * 0.7;
        p.copy_from_slice(&color::premul(col, v));
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "saluf（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "saluf.SumiDamari", name: "墨だまり", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                float(c"radius", c"溜まり半径", 6.0, 1.0, 20.0),
                float(c"threshold", c"しきい値", 50.0, 25.0, 100.0),
                float(c"roundness", c"丸め強度", 100.0, 0.0, 100.0),
                boolean(c"use_color", c"色を指定する", false),
                color_hex(c"color", c"色", 0xffffff),
            ],
            render: render,
        },
    ]
}
