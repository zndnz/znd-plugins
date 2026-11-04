//! Erosion（こんにゃくねこ）の移植: 3×3 の最小アルファを strength 回くり返して輪郭を削る。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    // 3×3 の最小を n 回 = 一辺 2n+1 の正方形の最小（横 → 縦に分けて計算する）
    let n = (f.f(0).round().max(0.0) * f.scale()).round() as usize;
    if n == 0 {
        return Ok(());
    }
    let (w, h) = (f.width, f.height);
    let a: Vec<f32> = f.rgba.chunks_exact(4).map(|p| p[3]).collect();
    let mut t = vec![0.0f32; w * h];
    for y in 0..h {
        for x in 0..w {
            let (x0, x1) = (x.saturating_sub(n), (x + n).min(w - 1));
            // 範囲外は透明として数える（元のシェーダーの範囲外読み出しは 0）
            let edge = x < n || x + n >= w;
            t[y * w + x] = if edge { 0.0 } else { a[y * w + x0..=y * w + x1].iter().cloned().fold(1.0, f32::min) };
        }
    }
    for y in 0..h {
        for x in 0..w {
            let m = if y < n || y + n >= h { 0.0 } else { (y - n..=y + n).map(|yy| t[yy * w + x]).fold(1.0, f32::min) };
            let i = (y * w + x) * 4;
            let pa = f.rgba[i + 3];
            let k = if pa > 0.0 { m / pa } else { 0.0 };
            for c in 0..4 {
                f.rgba[i + c] *= k;
            }
        }
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Erosion", name: "Erosion", category: "加工", flags: THREAD_SAFE,
            params: [
                float(c"str", c"strength", 5.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}
