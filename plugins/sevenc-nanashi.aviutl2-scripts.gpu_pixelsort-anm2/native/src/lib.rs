//! GPUピクセルソート（Nanashi.、BitonicPixelSorter by ruccho がもと）の移植:
//! 明るさが しきい値の最小〜最大 に入る画素が続く区間を、明るさの順に並べ替える。範囲の可視化もできる。
//! 元は GPU のバイトニックソートだが、ここでは CPU で区間ごとに並べ替える（結果は同じ並び）。

use znd_plugin_sdk::*;

fn bright(p: [f32; 4]) -> f32 {
    let d = color::to_display(p);
    // 元のシェーダーは乗算済みの色から明るさを求める
    let a = d[3];
    (0.298912 * d[0] * a + 0.586611 * d[1] * a + 0.114478 * d[2] * a).clamp(0.0, 1.0)
}

fn render(f: &mut Frame) -> FxResult {
    let dir = f.i(0);
    let (tmin, tmax) = (f.f(1) as f32, f.f(2) as f32);
    let vis = f.b(3);
    let vcol = f.color(4);
    let (vfill, vsrc) = (f.f(5).clamp(0.0, 1.0) as f32, f.f(6).clamp(0.0, 1.0) as f32);
    let vertical = dir % 2 == 1;
    let reverse = dir >= 2;
    let src = f.input();
    let (w, h) = (src.w, src.h);
    let (len, lines) = if vertical { (h, w) } else { (w, h) };
    let at = |k: usize, l: usize| if vertical { l + k * w } else { k + l * w };
    let mut out = src.clone();
    let vc = [0, 1, 2].map(|i| color::linear_to_srgb(vcol[i]));
    let mut idx: Vec<(f32, usize)> = Vec::with_capacity(len);
    for l in 0..lines {
        let px = |k: usize| { let i = at(k, l) * 4; [src.data[i], src.data[i + 1], src.data[i + 2], src.data[i + 3]] };
        let br: Vec<f32> = (0..len).map(|k| bright(px(k))).collect();
        let mut k = 0;
        while k < len {
            if !(tmin <= br[k] && br[k] <= tmax) {
                if vis {
                    let p = px(k);
                    out.data[at(k, l) * 4..at(k, l) * 4 + 4].copy_from_slice(&p.map(|v| v * vsrc));
                }
                k += 1;
                continue;
            }
            let start = k;
            while k < len && tmin <= br[k] && br[k] <= tmax {
                k += 1;
            }
            idx.clear();
            idx.extend((start..k).map(|j| (br[j], j)));
            idx.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            if reverse {
                idx.reverse();
            }
            let span = k - start > 1;
            for (n, &(_, j)) in idx.iter().enumerate() {
                let mut p = px(j);
                if vis {
                    if span {
                        let d = color::to_display(p);
                        let a = d[3];
                        let rgb = [0, 1, 2].map(|i| (d[i] * a * vsrc + vc[i] * vfill).clamp(0.0, 1.0));
                        p = if a > 0.0 { color::from_display([rgb[0] / a, rgb[1] / a, rgb[2] / a, a].map(|v| v.clamp(0.0, 1.0))) } else { [0.0; 4] };
                    } else {
                        p = p.map(|v| v * vsrc);
                    }
                }
                out.data[at(start + n, l) * 4..at(start + n, l) * 4 + 4].copy_from_slice(&p);
            }
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.gpu_pixelsort-anm2", name: "GPUピクセルソート", category: "加工", flags: THREAD_SAFE,
            params: [
                choice(c"direction", c"方向", 0, c"横方向|縦方向|横方向（反転）|縦方向（反転）"),
                float(c"threshold_min", c"しきい値::最小", 0.0, 0.0, 1.0),
                float(c"threshold_max", c"しきい値::最大", 1.0, 0.0, 1.0),
                boolean(c"visualize", c"ソート範囲可視化", false),
                color_hex(c"visualize_color", c"可視化の色", 0xff00ff),
                float(c"visualize_fill_opacity", c"可視化の塗り", 0.5, 0.0, 1.0),
                float(c"visualize_source_opacity", c"可視化の元画像", 0.5, 0.0, 1.0),
            ],
            render: render,
        },
    ]
}
