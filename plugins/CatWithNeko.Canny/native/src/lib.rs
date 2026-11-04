//! Canny（こんにゃくねこ）の移植: ぼかし → Sobel → 細線化 → 2 つの閾値 → つなぐ、の Canny 風エッジ抽出。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let blur = f.f(0) * f.scale();
    let hi = f.f(1) as f32;
    let lo = f.f(2) as f32;
    let rounds = f.f(3).round().max(0.0) as usize;
    let mut src = f.input();
    if blur > 0.0 {
        aul::blur(&mut src, blur as f32);
    }
    let (w, h) = (src.w, src.h);
    let gray: Vec<f32> = src.data.chunks_exact(4).map(|p| {
        let d = color::to_display([p[0], p[1], p[2], p[3]]);
        // 乗算済みの表示値で計算する（元と同じく透明は黒）
        (0.299 * d[0] + 0.587 * d[1] + 0.114 * d[2]) * d[3]
    }).collect();
    let g = |x: isize, y: isize| {
        if x < 0 || y < 0 || x >= w as isize || y >= h as isize { 0.0 } else { gray[y as usize * w + x as usize] }
    };
    let mut mag = vec![0.0f32; w * h];
    let mut ang = vec![0.0f64; w * h];
    for y in 0..h as isize {
        for x in 0..w as isize {
            let gx = -g(x - 1, y - 1) + g(x + 1, y - 1) - 2.0 * g(x - 1, y) + 2.0 * g(x + 1, y) - g(x - 1, y + 1) + g(x + 1, y + 1);
            let gy = -g(x - 1, y - 1) - 2.0 * g(x, y - 1) - g(x + 1, y - 1) + g(x - 1, y + 1) + 2.0 * g(x, y + 1) + g(x + 1, y + 1);
            let i = y as usize * w + x as usize;
            mag[i] = gx.hypot(gy).min(1.0);
            ang[i] = (gy as f64).atan2(gx as f64);
        }
    }
    let m = |x: isize, y: isize| if x < 0 || y < 0 || x >= w as isize || y >= h as isize { 0.0 } else { mag[y as usize * w + x as usize] };
    // 0 抑制 / 1 弱 / 2 強
    let mut cls = vec![0u8; w * h];
    for y in 0..h as isize {
        for x in 0..w as isize {
            let i = y as usize * w + x as usize;
            let t = ang[i];
            // 元のシェーダーと同じ区分（負の角度は横として扱う）
            let d: (isize, isize) = if t < PI / 8.0 { (1, 0) } else if t < 3.0 * PI / 8.0 { (1, 1) } else if t < 5.0 * PI / 8.0 { (0, 1) } else { (-1, 1) };
            let v = mag[i];
            let v = if v >= m(x + d.0, y + d.1) && v >= m(x - d.0, y - d.1) { v } else { 0.0 };
            cls[i] = if v > hi { 2 } else if v > lo { 1 } else { 0 };
        }
    }
    for _ in 0..rounds {
        let prev = cls.clone();
        let mut changed = false;
        for y in 0..h {
            for x in 0..w {
                if prev[y * w + x] != 1 {
                    continue;
                }
                let strong = (-1isize..=1).any(|dy| (-1isize..=1).any(|dx| {
                    let (nx, ny) = (x as isize + dx, y as isize + dy);
                    nx >= 0 && ny >= 0 && nx < w as isize && ny < h as isize && prev[ny as usize * w + nx as usize] == 2
                }));
                if strong {
                    cls[y * w + x] = 2;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    // 強いエッジだけを白で残し、ほかは透明にする
    for (p, c) in f.rgba.chunks_exact_mut(4).zip(&cls) {
        let v = if *c == 2 { 1.0 } else { 0.0 };
        p.copy_from_slice(&[v, v, v, v]);
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Canny", name: "Canny", category: "抽出", flags: THREAD_SAFE,
            params: [
                float(c"blur", c"Blur", 3.0, 0.0, 30.0),
                float(c"high", c"High", 0.3, 0.0, 1.0),
                float(c"low", c"Low", 0.1, 0.0, 1.0),
                float(c"links", c"Connect", 3.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}
